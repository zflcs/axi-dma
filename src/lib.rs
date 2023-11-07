
#![no_std]

mod bd;
mod bd_ring;
mod hw;
#[cfg(feature = "async")]
mod async_transfer;

#[cfg(feature = "async")]
pub use async_transfer::{AsyncRxTransfer, AsyncTxTransfer};

#[cfg(feature = "async")]
use core::task::Waker;

#[cfg(feature = "sync")]
pub mod transfer;

#[cfg(feature = "sync")]
pub use transfer::{TxTransfer, RxTransfer};
use axidma_pac;
#[macro_use]
extern crate log;

extern crate alloc;
use alloc::{sync::Arc, collections::VecDeque};

use core::{
    arch::asm,
    sync::atomic::{
        compiler_fence, fence,
        Ordering::{SeqCst, self}, AtomicBool,
    },
};

pub use crate::hw::AXI_DMA_CONFIG;

use self::{
    bd_ring::{AxiDmaBdRing, AxiDmaBdRingConfig},
    hw::{AXIDMA_RX_OFFSET, AXIDMA_TX_OFFSET},
};

use spin::Mutex;



pub struct AxiDma {
    // Immutable
    base_address: usize,
    has_sg: bool,
    #[allow(unused)]
    is_micro_dma: bool,
    #[allow(unused)]
    addr_width: isize,

    // Mutable
    is_initialized: AtomicBool,
    tx_bd_ring: Option<Mutex<AxiDmaBdRing>>,
    #[cfg(feature = "async")]
    pub tx_wakers: Mutex<VecDeque<Waker>>,
    rx_bd_ring: Option<Mutex<AxiDmaBdRing>>,
    #[cfg(feature = "async")]
    pub rx_wakers: Mutex<VecDeque<Waker>>,
}

pub struct AxiDmaConfig {
    pub device_id: u32,
    pub base_address: usize,

    pub has_sts_cntrl_strm: bool,
    pub is_micro_dma: bool,

    pub has_mm2s: bool,
    pub has_mm2s_dre: bool,
    pub mm2s_data_width: usize,
    pub mm2s_burst_size: usize,

    pub has_s2mm: bool,
    pub has_s2mm_dre: bool,
    pub s2mm_data_width: usize,
    pub s2mm_burst_size: usize,

    pub has_sg: bool,
    pub sg_length_width: usize,
    pub addr_width: isize,
}

pub struct AxiDmaIntr {
    base_address: usize,
}

// impl Default for AxiDma {
//     fn default() -> Self {
//         AxiDma::new(AXI_DMA_CONFIG)
//     }
// }



impl AxiDma {
    const RESET_TIMEOUT: isize = 500;
    pub fn new(config: AxiDmaConfig) -> Arc<Self> {
        let max_transfer_len = (1usize << config.sg_length_width) - 1;
        let tx_bd_ring = if config.has_mm2s {
            Some(Mutex::new(AxiDmaBdRing::new(AxiDmaBdRingConfig {
                chan_base_addr: config.base_address + AXIDMA_TX_OFFSET,
                is_rx_chan: false,
                has_sts_cntrl_strm: config.has_sts_cntrl_strm,
                has_dre: config.has_mm2s_dre,
                data_width: (config.mm2s_data_width >> 3),
                addr_ext: (config.addr_width > 32),
                max_transfer_len: if config.is_micro_dma {
                    config.mm2s_data_width / 8 * config.mm2s_burst_size
                } else {
                    max_transfer_len
                },
            })))
        } else {
            None
        };

        let rx_bd_ring = if config.has_s2mm {
            Some(Mutex::new(AxiDmaBdRing::new(AxiDmaBdRingConfig {
                chan_base_addr: config.base_address + AXIDMA_RX_OFFSET,
                is_rx_chan: true,
                has_sts_cntrl_strm: config.has_sts_cntrl_strm,
                has_dre: config.has_s2mm_dre,
                data_width: (config.s2mm_data_width >> 3),
                addr_ext: (config.addr_width > 32),
                max_transfer_len: if config.is_micro_dma {
                    config.s2mm_data_width / 8 * config.s2mm_burst_size
                } else {
                    max_transfer_len
                },
            })))
        } else {
            None
        };

        Arc::new(Self {
            base_address: config.base_address,
            has_sg: config.has_sg,
            is_micro_dma: config.is_micro_dma,
            addr_width: config.addr_width,
            tx_bd_ring,
            rx_bd_ring,
            is_initialized: AtomicBool::new(false),
            #[cfg(feature = "async")]
            tx_wakers: Mutex::new(VecDeque::new()),
            #[cfg(feature = "async")]
            rx_wakers: Mutex::new(VecDeque::new()),
        })
    }

    #[inline]
    fn hardware(&self) -> &axidma_pac::axi_dma::RegisterBlock {
        unsafe { &*(self.base_address as *const _) }
    }

    pub fn reset(self: &Arc<Self>) {
        let hardware: &axidma_pac::axi_dma::RegisterBlock =
            unsafe { &*(self.base_address as *const _) };
        if let Some(ring) = self.tx_bd_ring.as_ref() {
            let mut ring = ring.lock();
            if self.has_sg {
                // ring.snaphot_curr_bd();
            }
            hardware.mm2s_dmacr.modify(|_, w| w.reset().reset());
            ring.is_halted = true;
        }
        if let Some(ring) = self.rx_bd_ring.as_ref() {
            let mut ring = ring.lock();
            if self.has_sg {
                // ring.snaphot_curr_bd();
            }
            hardware.s2mm_dmacr.modify(|_, w| w.reset().reset());
            ring.is_halted = true;
        }

        let mut timeout = AxiDma::RESET_TIMEOUT;
        while timeout > 0 && !self.reset_is_done() {
            timeout -= 1;
        }
        if timeout > 0 {
            self.is_initialized.store(true, Ordering::Relaxed);
        } else {
            error!("AXIDMA: failed reset in intialization");
        }
    }

    // reset is done when both went normal
    fn reset_is_done(self: &Arc<Self>) -> bool {
        if self.tx_bd_ring.is_some() && self.hardware().mm2s_dmacr.read().reset().is_reset() {
            return false;
        }
        if self.rx_bd_ring.is_some() && self.hardware().s2mm_dmacr.read().reset().is_reset() {
            return false;
        }
        true
    }

    pub fn tx_cyclic_enable(self: &Arc<Self>) {
        self.hardware().mm2s_dmacr.write(|w| w.cyclic_buffer_descriptor().set_bit())
    }

    pub fn tx_cyclic_disable(self: &Arc<Self>) {
        self.hardware().mm2s_dmacr.write(|w| w.cyclic_buffer_descriptor().clear_bit())
    }

    pub fn rx_cyclic_enable(self: &Arc<Self>) {
        self.hardware().s2mm_dmacr.write(|w| w.cyclic_buffer_descriptor().set_bit())
    }

    pub fn rx_cyclic_disable(self: &Arc<Self>) {
        self.hardware().s2mm_dmacr.write(|w| w.cyclic_buffer_descriptor().clear_bit())
    }

    fn start(self: &Arc<Self>) -> Result<(), ()> {
        if !self.is_initialized.load(Ordering::Relaxed) {
            error!("Start: Driver not initialized");
            return Err(());
        }
        let hardware: &axidma_pac::axi_dma::RegisterBlock =
            unsafe { &*(self.base_address as *const _) };
        if let Some(ring) = self.tx_bd_ring.as_ref() {
            let mut ring = ring.lock();
            if ring.is_halted {
                if self.has_sg {
                    ring.start().map_err(|e| {
                        error!("Start hw tx channel failed");
                        e
                    })?;
                } else {
                    compiler_fence(SeqCst);
                    fence(SeqCst);
                    io_fence();

                    hardware.mm2s_dmacr.modify(|_, w| w.run_stop().run())
                }
                ring.is_halted = false;
            }
        }
        if let Some(ring) = self.rx_bd_ring.as_ref() {
            let mut ring = ring.lock();
            if ring.is_halted {
                if self.has_sg {
                    ring.start().map_err(|e| {
                        error!("Start hw rx channel failed");
                        e
                    })?;
                } else {
                    compiler_fence(SeqCst);
                    fence(SeqCst);
                    io_fence();

                    hardware.s2mm_dmacr.modify(|_, w| w.run_stop().run())
                }
                ring.is_halted = false;
            }
        }
        Ok(())
    }

    pub fn pause(self: &Arc<Self>) -> Result<(), ()> {
        if !self.is_initialized.load(Ordering::Relaxed) {
            error!("Pause: Driver not initialized");
            return Err(());
        }
        let hardware: &axidma_pac::axi_dma::RegisterBlock =
            unsafe { &*(self.base_address as *const _) };
        if let Some(ring) = self.tx_bd_ring.as_ref() {
            let mut ring = ring.lock();
            if !self.has_sg {
                hardware.mm2s_dmacr.modify(|_, w| w.run_stop().stop())
            }
            ring.is_halted = true;
        }
        if let Some(ring) = self.rx_bd_ring.as_ref() {
            let mut ring = ring.lock();
            if !self.has_sg {
                hardware.s2mm_dmacr.modify(|_, w| w.run_stop().stop())
            }
            ring.is_halted = true;
        }
        Ok(())
    }

    pub fn resume(self: &Arc<Self>) -> Result<(), ()> {
        if !self.is_initialized.load(Ordering::Relaxed) {
            error!("Resume: Driver not initialized");
            return Err(());
        }
        self.start().map_err(|e| {
            error!("Resume: Failed to start engine");
            e
        })?;
        Ok(())
    }

    pub fn tx_intr_disable(self: &Arc<Self>) {
        trace!("axidma::tx_intr_disable");
        self.hardware().mm2s_dmacr.modify(|_, w| {
            w.dly_irq_en()
                .disable()
                .err_irq_en()
                .disable()
                .ioc_irq_en()
                .disable()
        })
    }

    pub fn rx_intr_disable(self: &Arc<Self>) {
        trace!("axidma::rx_intr_disable");
        self.hardware().s2mm_dmacr.modify(|_, w| {
            w.dly_irq_en()
                .disable()
                .err_irq_en()
                .disable()
                .ioc_irq_en()
                .disable()
        })
    }

    pub fn tx_intr_enable(self: &Arc<Self>) {
        trace!("axidma::tx_intr_enable");
        self.hardware().mm2s_dmacr.modify(|_, w| {
            w.dly_irq_en()
                .enable()
                .err_irq_en()
                .enable()
                .ioc_irq_en()
                .enable()
        });
    }

    pub fn rx_intr_enable(self: &Arc<Self>) {
        trace!("axidma::rx_intr_enable");
        self.hardware().s2mm_dmacr.modify(|_, w| {
            w.dly_irq_en()
                .enable()
                .err_irq_en()
                .enable()
                .ioc_irq_en()
                .enable()
        })
    }

    pub fn tx_bd_create(self: &Arc<Self>, bd_count: usize) {
        self.tx_intr_disable();
        if let Some(ring) = self.tx_bd_ring.as_ref() {
            let mut ring = ring.lock();
            ring.create(bd_count);
        }
    }

    pub fn rx_bd_create(self: &Arc<Self>, bd_count: usize) {
        self.rx_intr_disable();
        if let Some(ring) = self.rx_bd_ring.as_ref() {
            let mut ring = ring.lock();
            ring.create(bd_count);
        }
    }

    pub fn tx_from_hw(self: &Arc<Self>) {
        if let Some(ring) = self.tx_bd_ring.as_ref() {
            let mut ring = ring.lock();
            ring.from_hw();
        } else {
            trace!("axidma::tx_from_hw: no tx ring!");
        }
    }

    pub fn rx_from_hw(&self) {
        if let Some(ring) = self.rx_bd_ring.as_ref() {
            let mut ring = ring.lock();
            ring.from_hw();
        } else {
            trace!("axidma::rx_from_hw: no rx ring!");
        }
    }
}

impl AxiDmaIntr {
    pub fn new(base_address: usize) -> Arc<Self> {
        Arc::new(Self { base_address })
    }

    #[inline]
    fn hardware(&self) -> &axidma_pac::axi_dma::RegisterBlock {
        unsafe { &*(self.base_address as *const _) }
    }

    pub fn tx_intr_handler(self: &Arc<Self>) -> bool {
        let sr = &self.hardware().mm2s_dmasr;
        if sr.read().err_irq().is_detected() {
            // dump regs
            // reset
            error!("axidma_intr: tx err intr detected");
            self.tx_dump_regs();
            sr.modify(|_, w| w.err_irq().set_bit());
            return false;
        }
        if sr.read().ioc_irq().is_detected() {
            trace!("axidma_intr: tx cplt intr detected");
            sr.modify(|_, w| w.ioc_irq().set_bit());
        }
        if sr.read().dly_irq().is_detected() {
            trace!("axidma_intr: tx dly intr detected");
            sr.modify(|_, w| w.dly_irq().set_bit());
        }
        true
    }

    pub fn rx_intr_handler(self: &Arc<Self>) -> bool {
        let sr = &self.hardware().s2mm_dmasr;
        if sr.read().err_irq().is_detected() {
            // dump regs
            // reset
            error!("axidma: rx err intr detected");
            self.rx_dump_regs();
            sr.modify(|_, w| w.err_irq().set_bit());
            return false;
        }
        if sr.read().ioc_irq().is_detected() {
            trace!("axidma_intr: rx cplt intr detected");
            sr.modify(|_, w| w.ioc_irq().set_bit());
        }
        if sr.read().dly_irq().is_detected() {
            trace!("axidma_intr: rx dly intr detected");
            sr.modify(|_, w| w.dly_irq().set_bit());
        }
        true
    }

    pub fn tx_dump_regs(self: &Arc<Self>) {
        let hw = self.hardware();
        info!(
            "CR: 0b{:b}, SR: 0b{:b}",
            hw.mm2s_dmacr.read().bits(),
            hw.mm2s_dmasr.read().bits()
        );
        info!(
            "CDESC_MSB: 0x{:x}, CDESC: 0x{:x}",
            hw.mm2s_curdesc_msb.read().bits(),
            hw.mm2s_curdesc_msb.read().bits()
        );
        info!(
            "TDESC_MSB: 0x{:x}, TDESC: 0x{:x}",
            hw.mm2s_taildesc_msb.read().bits(),
            hw.mm2s_taildesc.read().bits()
        );
    }

    pub fn rx_dump_regs(self: &Arc<Self>) {
        let hw = self.hardware();
        info!(
            "CR: 0b{:b}, SR: 0b{:b}",
            hw.s2mm_dmacr.read().bits(),
            hw.s2mm_dmasr.read().bits()
        );
        info!(
            "CDESC_MSB: 0x{:x}, CDESC: 0x{:x}",
            hw.s2mm_curdesc_msb.read().bits(),
            hw.s2mm_curdesc_msb.read().bits()
        );
        info!(
            "TDESC_MSB: 0x{:x}, TDESC: 0x{:x}",
            hw.s2mm_taildesc_msb.read().bits(),
            hw.s2mm_taildesc.read().bits()
        );
    }
}

#[inline]
pub fn io_fence() {
    unsafe {
        asm!("fence iorw,iorw");
    }
}

