//! This file contains DMA channel related structure and constant definition
//! as well as function prototypes. Each DMA channel is managed by a Buffer
//! Descriptor ring. See [xaxidma.h](https://github.com/Xilinx/embeddedsw/blob/master/XilinxProcessorIPLib/drivers/axidma/src/xaxidma.h)
//! for more information on how a BD ring is managed.
//!

use crate::{io_fence, AxiDMAErr, AxiDMAResult, AxiDmaConfig, BufPtr};

use crate::bd::AxiDmaBD;
use alloc::{boxed::Box, collections::VecDeque};
use core::pin::Pin;
use core::sync::atomic::{compiler_fence, fence, Ordering::SeqCst};
use spin::Mutex;

/// The channel direction
#[derive(Debug)]
pub enum Direaction {
    TX,
    RX,
}

/// The structure of DMA channel for descriptor storage control.
/// It only supports the Scatter Gather mode.
/// Not supported feature:
///     - [ ] dynamic allocate buffer descriptor
///     - [ ] address translation(All address must be physical address)
pub struct AxiDMAChannel {
    // Immutable
    /// The base address of register space of Channel
    channel_baseaddr: usize,
    /// Whether has stscntrl stream
    has_sts_cntrl_strm: bool,
    /// Whether the Data Realignment Enable
    has_dre: bool,
    ///
    data_width: usize,
    ///
    max_transfer_len: usize,

    // Mutable
    /// Buffer descriptor ring
    pub ring: Mutex<BDRing>,
    /// Control field, it is also used in the interrupt handling.
    /// We assume that there is only one core will do interrupt handling.
    pub ctrl: Mutex<ControlFiled>,
}

/// The control of the channel
pub struct ControlFiled {
    /// check the interrupt enable
    is_intr_enable: bool,
    /// Check for cyclic DMA Mode
    is_cyclic: bool,
}

/// The structure of BDRing, it must be access exclusively.
pub struct BDRing {
    /// Whether channel is halted
    is_halted: bool,
    /// BD ring
    bds: VecDeque<Pin<Box<AxiDmaBD>>>,
    /// The index of first BD in the work group
    bd_head: usize,
    /// The index of last BD in the work group
    bd_tail: usize,
    /// BD to load when channel is started
    bd_restart: usize,
    /// Number of BDs in free group
    free_cnt: usize,
    /// Total Number of BDs for channel
    all_cnt: usize,
    /// Used for multiple transaction
    pending_cnt: usize,
    /// Used for multiple transaction
    submit_cnt: usize,
}

impl BDRing {
    /// the pointer of head buffer descriptor in the work group
    fn head_desc_addr(&self) -> usize {
        &self.bds[self.bd_head].desc as *const _ as usize
    }

    /// the pointer of tail buffer descriptor in the work group
    fn tail_desc_addr(&self) -> usize {
        &self.bds[self.bd_tail].desc as *const _ as usize
    }
}
impl AxiDMAChannel {
    /// Create a new channel without any buffer descriptor.
    pub fn new(direction: Direaction, cfg: &AxiDmaConfig) -> Self {
        let max_transfer_len = (1usize << cfg.sg_length_width) - 1;
        let (has_dre, data_width, channel_baseaddr) = match direction {
            Direaction::TX => (
                cfg.has_mm2s_dre,
                cfg.mm2s_data_width,
                cfg.base_address + cfg.tx_channel_offset,
            ),
            Direaction::RX => (
                cfg.has_s2mm_dre,
                cfg.s2mm_data_width,
                cfg.base_address + cfg.rx_channel_offset,
            ),
        };
        Self {
            channel_baseaddr,
            has_sts_cntrl_strm: cfg.has_sts_cntrl_strm,
            has_dre,
            data_width,
            max_transfer_len,
            ring: Mutex::new(BDRing {
                is_halted: true,
                bds: VecDeque::new(),
                bd_head: 0,
                bd_tail: 0,
                bd_restart: 0,
                free_cnt: 0,
                all_cnt: 0,
                pending_cnt: 0,
                submit_cnt: 0,
            }),
            ctrl: Mutex::new(ControlFiled {
                is_intr_enable: false,
                is_cyclic: false,
            }),
        }
    }

    /// Creates and setup the BD ring.
    pub fn create(&self, bd_count: usize) -> AxiDMAResult {
        if bd_count <= 0 {
            error!("non-positive BD number {}", bd_count);
            return Err(AxiDMAErr::InValidParam);
        }
        let mut ring = self.ring.lock();
        ring.bds.clear();
        ring.bds.reserve(bd_count);
        for _ in 0..bd_count {
            let bd = Box::pin(AxiDmaBD::new(
                self.has_sts_cntrl_strm,
                self.has_dre,
                self.data_width as _,
            ));
            ring.bds.push_back(bd);
        }
        // link bd chain
        for i in 0..bd_count {
            let next_addr = &ring.bds[(i + 1) % bd_count].desc as *const _ as usize;
            ring.bds[i].set_next_desc_addr(next_addr);
        }
        ring.pending_cnt = 0;
        ring.submit_cnt = 0;
        ring.is_halted = true;
        ring.all_cnt = bd_count;
        ring.free_cnt = bd_count;
        ring.bd_head = 0;
        ring.bd_tail = 0;
        ring.bd_restart = 0;
        Ok(())
    }

    /// Reset this channel.
    pub fn reset(&self) -> AxiDMAResult {
        self.hardware().dmacr().modify(|_, w| w.reset().reset());
        self.ring.lock().is_halted = true;
        Ok(())
    }

    /// Start this channel.
    pub fn start(&self) -> AxiDMAResult {
        self.hardware().dmacr().modify(|_, w| w.run_stop().run());
        self.ring.lock().is_halted = false;
        Ok(())
    }

    /// Stop this channel.
    pub fn stop(&self) -> AxiDMAResult {
        self.hardware().dmacr().modify(|_, w| w.run_stop().stop());
        self.ring.lock().is_halted = true;
        Ok(())
    }

    // Check whether reset is done when both went normal
    pub fn reset_is_done(&self) -> bool {
        if self.hardware().dmacr().read().reset().is_reset() {
            return false;
        }
        true
    }

    /// Set interrupt coalescing parameters for the given descriptor ring channel.
    /// Only supported interrupt threshold count not timer delay
    /// The valid threshold is 1 ~ 255
    pub fn set_coalesce(&self, threshold: usize) -> AxiDMAResult {
        if threshold == 0 || threshold > 0xff {
            error!("invalid coalescing threshold {}", threshold);
            return Err(AxiDMAErr::InValidParam);
        }
        self.hardware()
            .dmacr()
            .modify(|_, w| unsafe { w.irq_threshold().bits(threshold as _) });
        Ok(())
    }

    /// Get the interrupt threshold count
    pub fn get_coalesce(&self) -> usize {
        self.hardware().dmacr().read().irq_threshold().bits() as _
    }

    pub fn submit(&self, buffer: BufPtr) -> Result<BufPtr, AxiDMAErr> {
        if buffer.len() > self.max_transfer_len {
            error!("single buffer len has exceed max transfer len");
            return Err(AxiDMAErr::InValidParam);
        }
        let mut ring = self.ring.lock();
        if 1 > ring.free_cnt {
            error!("no free bd");
            return Err(AxiDMAErr::BDRingNoList);
        }
        let start = ring.bd_restart;
        let bd = &ring.bds[ring.bd_restart];
        bd.clear();
        bd.set_buf(&buffer)?;
        ring.bd_restart += 1;
        if ring.bd_restart == ring.all_cnt {
            ring.bd_restart = 0;
        }
        ring.bd_tail = if ring.bd_restart == 0 {
            ring.bds.len() - 1
        } else {
            ring.bd_restart - 1
        };
        ring.bds[start]
            .desc
            .control()
            .modify(|_, w| w.sof().set_bit());
        ring.bds[ring.bd_tail]
            .desc
            .control()
            .modify(|_, w| w.eof().set_bit());

        ring.free_cnt -= 1;
        ring.pending_cnt += 1;
        trace!(
            "bd_ring::submit: done, restart: {}, tail: {}, free: {}, pending: {}",
            ring.bd_restart,
            ring.bd_tail,
            ring.free_cnt,
            ring.pending_cnt
        );
        Ok(buffer)
    }

    /// Retrieve the BD from hardware
    pub fn from_hw(&self) -> AxiDMAResult {
        let mut ring = self.ring.lock();
        let mut bd_cnt = 0;
        let mut partial_cnt = 0;
        let mut cur_bd = ring.bd_head;
        trace!(
            "bd_ring::from_hw: head: {}, tail: {}",
            ring.bd_head,
            ring.bd_tail
        );
        compiler_fence(SeqCst);
        fence(SeqCst);
        io_fence();

        loop {
            let bd = &ring.bds[cur_bd];
            // unsafe { ebreak() };
            let status = bd.desc.status().read();
            // Check the status of buffer descriptor, if is not completed, this action must be stopped.
            if status.cmplt().is_false() {
                // unsafe { ebreak() };
                trace!("bd_ring::from_hw: Uncompleted BD found at {}", cur_bd);
                bd.dump();
                break;
            }
            bd_cnt += 1;
            let ctrl = bd.desc.control().read();
            // check that this buffer descriptor is the end of the transaction
            // It is found a eof of a transaction, but maybe there are more buffer descriptor under the hardware.
            if ctrl.eof().is_true() || status.rxeof().is_true() {
                trace!("bd_ring::from_hw: EOF found at {}", cur_bd);
                partial_cnt = 0;
            } else {
                partial_cnt += 1;
            }
            if cur_bd == ring.bd_tail {
                break;
            }
            cur_bd += 1;
            if cur_bd == ring.all_cnt {
                cur_bd = 0;
            }
        }
        trace!(
            "bd_ring::from_hw: bd_cnt: {}, partial: {}",
            bd_cnt,
            partial_cnt
        );
        bd_cnt -= partial_cnt;
        if bd_cnt > 0 {
            ring.bd_head = (ring.bd_head + bd_cnt) % ring.all_cnt;
            ring.submit_cnt -= bd_cnt;
            ring.free_cnt += bd_cnt;
            trace!("bd_ring::from_hw: free_cnt: {}", ring.free_cnt);
        } else {
            warn!("bd_ring::from_hw: no completed BD!");
        }
        Ok(())
    }

    pub fn to_hw(&self) -> AxiDMAResult {
        let hardware = self.hardware();
        let mut ring = self.ring.lock();
        if ring.is_halted {
            let addr = ring.head_desc_addr();
            self.update_cur_bd(addr);
            trace!("axidma::tx_to_hw: cur desc addr: 0x{:x}", addr);
        }
        compiler_fence(SeqCst);
        fence(SeqCst);
        io_fence();
        hardware.dmacr().modify(|_, w| w.run_stop().run());
        ring.is_halted = false;
        if ring.pending_cnt > 0 {
            ring.submit_cnt += ring.pending_cnt;
            ring.pending_cnt = 0;
            // update tail desc
            self.update_tail_bd(ring.tail_desc_addr());
        }
        Ok(())
    }

    /// Wait the channel completing a transaction synchronously.
    pub fn wait(&self) {
        let mut status = self.hardware().dmasr().read();
        while status.ioc_irq().is_no_intr()
            && status.dly_irq().is_no_intr()
            && status.err_irq().is_no_intr()
        {
            status = self.hardware().dmasr().read();
        }
    }

    /// Enable the cyclic mode of this channel
    pub fn cyclic_enable(&self) {
        self.hardware()
            .dmacr()
            .write(|w| w.cyclic_enable().set_bit());
        self.ctrl.lock().is_cyclic = true;
    }

    /// Disable the cyclic mode of this channel
    pub fn cyclic_disable(&self) {
        self.hardware()
            .dmacr()
            .write(|w| w.cyclic_enable().clear_bit());
        self.ctrl.lock().is_cyclic = false;
    }

    /// Disable the interrupt of this channel.
    pub fn intr_disable(&self) {
        trace!("channel intr_disable");
        self.hardware().dmacr().modify(|_, w| {
            w.dly_irq_en()
                .disable()
                .err_irq_en()
                .disable()
                .ioc_irq_en()
                .disable()
        });
        self.ctrl.lock().is_intr_enable = false;
    }

    /// Enable the interrupt of this channel.
    pub fn intr_enable(&self) {
        trace!("channel intr_enable");
        self.hardware().dmacr().modify(|_, w| {
            w.dly_irq_en()
                .enable()
                .err_irq_en()
                .enable()
                .ioc_irq_en()
                .enable()
        });
        self.ctrl.lock().is_intr_enable = true;
    }

    /// Check whether a transaction is completed
    pub fn check_cmplt(&self) -> bool {
        let status = self.hardware().dmasr().read();
        status.err_irq().is_detected()
            || status.dly_irq().is_detected()
            || status.ioc_irq().is_detected()
    }

    /// The interrupt handler
    pub fn intr_handler(&self) -> AxiDMAResult {
        let ctrl = self.ctrl.lock();
        // If the channel disables the interrupt, it will do nothing.
        if ctrl.is_intr_enable {
            let sr = self.hardware().dmasr();
            let status = sr.read();
            if status.err_irq().is_detected() {
                // dump regs
                // reset
                trace!("axidma_intr: err intr detected");
                self.dump_regs();
                sr.modify(|_, w| w.err_irq().set_bit());
                return Err(AxiDMAErr::IntrErr);
            }
            if status.ioc_irq().is_detected() {
                trace!("axidma_intr: cplt intr detected");
                sr.modify(|_, w| w.ioc_irq().set_bit());
            }
            if status.dly_irq().is_detected() {
                trace!("axidma_intr: dly intr detected");
                sr.modify(|_, w| w.dly_irq().set_bit());
            }
        }
        Ok(())
    }

    /// Dump the register of channel
    pub fn dump_regs(&self) {
        let hw = self.hardware();
        info!(
            "CR: 0b{:b}, SR: 0b{:b}",
            hw.dmacr().read().bits(),
            hw.dmacr().read().bits()
        );
        info!(
            "CDESC_MSB: 0x{:x}, CDESC: 0x{:x}",
            hw.curdesc_msb().read().bits(),
            hw.curdesc().read().bits()
        );
        info!(
            "TDESC_MSB: 0x{:x}, TDESC: 0x{:x}",
            hw.taildesc_msb().read().bits(),
            hw.taildesc().read().bits()
        );
    }

    /// Get the registers of the channel
    #[inline]
    fn hardware(&self) -> &axidma_pac::channel::RegisterBlock {
        unsafe { &*(self.channel_baseaddr as *const _) }
    }

    /// Update the current buffer descriptor of the channel
    fn update_cur_bd(&self, addr: usize) {
        let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
        let addr_msb = (addr >> 32) as _;
        unsafe {
            self.hardware()
                .curdesc()
                .write(|w| w.curdesc_ptr().bits(addr_lsb));
            self.hardware()
                .curdesc_msb()
                .write(|w| w.curdesc_ptr().bits(addr_msb));
        }
    }

    /// Update the tail buffer descriptor of the channel
    fn update_tail_bd(&self, addr: usize) {
        let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
        let addr_msb = (addr >> 32) as _;
        unsafe {
            self.hardware()
                .taildesc()
                .write(|w| w.taildesc_ptr().bits(addr_lsb));
            self.hardware()
                .taildesc_msb()
                .write(|w| w.taildesc_ptr().bits(addr_msb));
        }
    }
}
