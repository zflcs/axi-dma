use core::{ops::Deref, pin::Pin, sync::atomic::{self, Ordering, compiler_fence, fence}, hint};
use alloc::sync::Arc;

use crate::{AxiDma, io_fence};

pub struct RxTransfer<B> 
where
        B: Deref,
        B::Target: AsRef<[u8]> + 'static,
{
    // NOTE: always `Some` variant
    buffer: Option<Pin<B>>,
    dma: Arc<AxiDma>,
}

pub struct TxTransfer<B> 
where
        B: Deref,
        B::Target: AsRef<[u8]>,
{
    // NOTE: always `Some` variant
    buffer: Option<Pin<B>>,
    dma: Arc<AxiDma>,
}

impl<B> RxTransfer<B> 
where
    B: Deref,
    B::Target: AsRef<[u8]>,
{

    pub fn new(buf: Pin<B>, dma: Arc<AxiDma>) -> Self {
        Self { buffer: Some(buf), dma }
    }

    /// Blocks until the transfer is done and returns the buffer
    pub fn wait(mut self) -> Pin<B> {
        self.dma.rx_wait();
        atomic::compiler_fence(Ordering::SeqCst);
        self.dma.rx_from_hw();
        self.buffer.take().unwrap_or_else(|| unsafe { hint::unreachable_unchecked() })
    }
}

impl<B> TxTransfer<B> 
where
    B: Deref,
    B::Target: AsRef<[u8]>,
{

    pub fn new(buf: Pin<B>, dma: Arc<AxiDma>) -> Self {
        Self { buffer: Some(buf), dma }
    }

    /// Blocks until the transfer is done and returns the buffer
    pub fn wait(mut self) -> Pin<B> {
        self.dma.tx_wait();
        atomic::compiler_fence(Ordering::SeqCst);
        self.dma.tx_from_hw();
        self.buffer.take().unwrap_or_else(|| unsafe { hint::unreachable_unchecked() })

    }
}


impl AxiDma {

    pub fn tx_submit<B>(self: &Arc<Self>, buf: Pin<B>) -> Option<TxTransfer<B>>
    where
        B: Deref,
        B::Target: AsRef<[u8]>
    {
        if let Some(ring) = self.tx_bd_ring.as_ref() {
            let mut ring = ring.lock();
            ring.submit(&buf);

            let hardware: &axidma_pac::axi_dma::RegisterBlock =
                unsafe { &*(self.base_address as *const _) };
            if ring.is_halted {
                // update cur desc
                let addr = ring.head_desc_addr();
                let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
                let addr_msb = (addr >> 32) as _;
                trace!("axidma::tx_to_hw: cur desc addr: 0x{:x}", addr);
                unsafe {
                    hardware
                        .mm2s_curdesc
                        .write(|w| w.curdesc_ptr().bits(addr_lsb));
                    hardware
                        .mm2s_curdesc_msb
                        .write(|w| w.curdesc_ptr().bits(addr_msb));
                }
            } else {
                trace!("axidma::tx_to_hw: ring running, cur desc not updated");
            }
            compiler_fence(Ordering::SeqCst);
            fence(Ordering::SeqCst);
            io_fence();

            hardware.mm2s_dmacr.modify(|_, w| w.run_stop().run());
            ring.is_halted = false;
            if ring.pending_cnt > 0 {
                ring.submit_cnt += ring.pending_cnt;
                ring.pending_cnt = 0;
                // update tail desc
                let addr = ring.tail_desc_addr();
                let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
                let addr_msb = (addr >> 32) as _;
                trace!("axidma::tx_to_hw: tail desc addr: 0x{:x}", addr);
                unsafe {
                    hardware
                        .mm2s_taildesc
                        .write(|w| w.taildesc_ptr().bits(addr_lsb));
                    hardware
                        .mm2s_taildesc_msb
                        .write(|w| w.taildesc_ptr().bits(addr_msb));
                }
            } else {
                trace!("axidma::tx_to_hw: no pending BD, tail desc not updated");
            }
            Some(TxTransfer::new(buf, self.clone()))
        } else {
            trace!("axidma::tx_submit: no tx ring!");
            None
        }
    }

    pub fn rx_submit<B>(self: &Arc<Self>, buf: Pin<B>) -> Option<RxTransfer<B>>
    where
        B: Deref,
        B::Target: AsRef<[u8]>
    {
        if let Some(ring) = self.rx_bd_ring.as_ref() {
            let mut ring = ring.lock();
            ring.submit(&buf);
            let hardware: &axidma_pac::axi_dma::RegisterBlock =
                unsafe { &*(self.base_address as *const _) };
            if ring.is_halted {
                // update cur desc
                let addr = ring.head_desc_addr();
                let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
                let addr_msb = (addr >> 32) as _;
                trace!("axidma::rx_to_hw: cur desc addr: 0x{:x}", addr);

                unsafe {
                    hardware
                        .s2mm_curdesc
                        .write(|w| w.curdesc_ptr().bits(addr_lsb));
                    hardware
                        .s2mm_curdesc_msb
                        .write(|w| w.curdesc_ptr().bits(addr_msb));
                }
            } else {
                trace!("axidma::rx_to_hw: ring running, cur desc not updated");
            }

            compiler_fence(Ordering::SeqCst);
            fence(Ordering::SeqCst);
            io_fence();
            hardware.s2mm_dmacr.modify(|_, w| w.run_stop().run());
            ring.is_halted = false;
            if ring.pending_cnt > 0 {
                ring.submit_cnt += ring.pending_cnt;
                ring.pending_cnt = 0;
                // update tail desc
                let addr = ring.tail_desc_addr();
                let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
                let addr_msb = (addr >> 32) as _;
                trace!("axidma::rx_to_hw: tail desc addr: 0x{:x}", addr);
                unsafe {
                    hardware
                        .s2mm_taildesc
                        .write(|w| w.taildesc_ptr().bits(addr_lsb));
                    hardware
                        .s2mm_taildesc_msb
                        .write(|w| w.taildesc_ptr().bits(addr_msb));
                }
            } else {
                trace!("axidma::rx_to_hw: no pending BD, tail desc not updated");
            }
            Some(RxTransfer::new(buf, self.clone()))
        } else {
            trace!("axidma::rx_submit: no rx ring!");
            None
        }
    }
    
    pub fn tx_wait(self: &Arc<Self>) {
        let mut status = self.hardware().mm2s_dmasr.read();
        while status.ioc_irq().is_no_intr() && status.dly_irq().is_no_intr() && status.err_irq().is_no_intr() {
            status = self.hardware().mm2s_dmasr.read();
        }
    }


    pub fn rx_wait(self: &Arc<Self>) {
        let mut status = self.hardware().s2mm_dmasr.read();
        while status.ioc_irq().is_no_intr() && status.dly_irq().is_no_intr() && status.err_irq().is_no_intr() {
            status = self.hardware().s2mm_dmasr.read();
        }
    }
}