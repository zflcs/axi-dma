//! This file contains DMA channel related structure and constant definition
//! as well as function prototypes. Each DMA channel is managed by a Buffer
//! Descriptor ring. See [xaxidma.h](https://github.com/Xilinx/embeddedsw/blob/master/XilinxProcessorIPLib/drivers/axidma/src/xaxidma.h) 
//! for more information on how a BD ring is managed.
//! 

use alloc::collections::VecDeque;
use spin::Mutex;
use core::{
    ops::Deref, pin::Pin, sync::atomic::{compiler_fence, fence, Ordering::{self, SeqCst}}
};
use alloc::boxed::Box;
use crate::{bd::AxiDmaBD, errno::AxiDMAErr, io_fence, AxiDMAResult, AxiDmaConfig};

#[cfg(feature = "async")]
use core::task::Waker;

/// The channel direction
#[derive(Debug)]
pub enum Direaction {
    TX,
    RX
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
    #[cfg(feature = "async")]
    /// The queue of wakers of the channel
    pub wakers: VecDeque<Waker>,
}

/// The structure of BDRing, it must be access exclusively.
pub struct BDRing {
    /// Whether channel is halted
    is_halted: bool,
    /// BD ring
    bds: VecDeque<Pin<Box<AxiDmaBD>>>,
    /// The index of first BD in the work group
    hw_head: usize,
    /// The index of last BD in the work group
    hw_tail: usize,
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
        &self.bds[self.hw_head].desc as *const _ as usize
    }

    /// the pointer of tail buffer descriptor in the work group
    fn tail_desc_addr(&self) -> usize {
        &self.bds[self.hw_tail].desc as *const _ as usize
    }
}

impl AxiDMAChannel {
    /// Create a new channel without any buffer descriptor.
    pub fn new(direction: Direaction, cfg: &AxiDmaConfig) -> Self {
        let max_transfer_len = (1usize << cfg.sg_length_width) - 1;
        let (has_dre, data_width, channel_baseaddr) = match direction {
            Direaction::TX => { (cfg.has_mm2s_dre, cfg.mm2s_data_width, cfg.base_address + cfg.tx_channel_offset) },
            Direaction::RX => { (cfg.has_s2mm_dre, cfg.s2mm_data_width, cfg.base_address + cfg.rx_channel_offset) }
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
                hw_head: 0, 
                hw_tail: 0, 
                bd_restart: 0, 
                free_cnt: 0, 
                all_cnt: 0, 
                pending_cnt: 0, 
                submit_cnt: 0 
            }),
            ctrl: Mutex::new(ControlFiled { 
                is_intr_enable: false, 
                is_cyclic: false,
                #[cfg(feature = "async")]
                wakers: VecDeque::new()
            })
        }
    }

    /// Creates and setup the BD ring.
    pub fn create(&self, bd_count: usize) -> AxiDMAResult {
        if bd_count <= 0 {
            log::error!("non-positive BD number {}", bd_count);
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
        ring.hw_head = 0;
        ring.hw_tail = 0;
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
            log::error!("invalid coalescing threshold {}", threshold);
            return Err(AxiDMAErr::InValidParam);
        }
        self.hardware().dmacr().modify(|_, w| unsafe { w.irq_threshold().bits(threshold as _) });
        Ok(())
    }

    /// Get the interrupt threshold count
    pub fn get_coalesce(&self) -> usize {
        self.hardware().dmacr().read().irq_threshold().bits() as _
    }

    /// Submit buffer to the DMA channel, and start a transaction.
    /// In order to ensure the transaction successful, the buffer must be pinned.
    pub fn submit<B>(&self, buffer: &Pin<B>) -> AxiDMAResult
    where
        B: Deref,
        B::Target: AsRef<[u8]>,
    {
        let buf = (**buffer).as_ref();
        let mut ring = self.ring.lock();
        ring.hw_head = ring.bd_restart;
        let mut buf_len = buf.len();
        let mut buf_head = 0;
        let mut bd_len = self.max_transfer_len;
        log::trace!("max_transfer_len: {}", self.max_transfer_len);
        let bd_cnt = (buf_len + bd_len - 1) / bd_len;
        if bd_cnt > ring.free_cnt {
            log::error!("bd_ring::submit: too many BD required!, bd_cnt {}, free_cnt {}", bd_cnt, ring.free_cnt);
            return Err(AxiDMAErr::DMAErr);
        }
        log::trace!("bd_ring::submit: buf_len: {}, bd_cnt: {}, restart: {}", buf_len, bd_cnt, ring.bd_restart);
        for _ in 0..bd_cnt {
            let bd = &ring.bds[ring.bd_restart];
            bd.clear();
            if buf_len < bd_len {
                bd_len = buf_len;
            }
            bd.set_buf(&buf[buf_head..buf_head + bd_len])?;
            let peek_len = 16.min(bd_len);
            log::trace!("bd_ring::submit: peek buf[{}..{}]: {:x?}", buf_head, buf_head + peek_len, &buf[buf_head..buf_head + peek_len]);
            buf_head += bd_len;
            buf_len -= bd_len;
            ring.bd_restart += 1;
            if ring.bd_restart == ring.all_cnt {
                ring.bd_restart = 0;
            }
        }
        ring.hw_tail = if ring.bd_restart == 0 {
            ring.bds.len() - 1
        } else {
            ring.bd_restart - 1
        };
        // Set the flag of the start and end buffer descriptor.
        ring.bds[ring.hw_head].set_sof();
        ring.bds[ring.hw_tail].set_eof();
        ring.free_cnt -= bd_cnt;
        ring.pending_cnt += bd_cnt;
        // The current buffer descriptor can be updated only when the channel is halted.
        if ring.is_halted {
            self.update_cur_bd(ring.head_desc_addr());
        }
        compiler_fence(Ordering::SeqCst);
        fence(Ordering::SeqCst);
        io_fence();
        // Start the channel
        self.hardware().dmacr().modify(|_, w| w.run_stop().run());
        ring.is_halted = false;
        // If there are some buffer descriptor are pended, the tail buffer descriptor must be updated.
        // For example, there is a new transaction happend when the last transaction is not finished.
        // The tail buffer descriptor must be updated on time.
        if ring.pending_cnt > 0 {
            log::trace!("pending_cnt > 0, update tail bd");
            ring.submit_cnt += ring.pending_cnt;
            ring.pending_cnt = 0;
            self.update_tail_bd(ring.tail_desc_addr());
        }
        log::trace!(
            "bd_ring::submit: done, restart: {}, head: {}, tail: {}, free: {}",
            ring.bd_restart, ring.hw_head, ring.hw_tail, ring.free_cnt
        );
        Ok(())
    }

    /// Retrieve the BD from hardware
    pub fn from_hw(&self) -> AxiDMAResult {
        let mut ring = self.ring.lock();
        let mut bd_cnt = 0;
        let mut partial_cnt = 0;
        let mut cur_bd = ring.hw_head;
        log::trace!("bd_ring::from_hw: head: {}, tail: {}",ring.hw_head, ring.hw_tail);
        compiler_fence(SeqCst);
        fence(SeqCst);
        io_fence();
        loop {
            let bd = &ring.bds[cur_bd];
            let status = bd.desc.status().read();
            // Check the status of buffer descriptor, if is not completed, this action must be stopped.
            if status.cmplt().is_false() {
                log::trace!("bd_ring::from_hw: Uncompleted BD found at {}", cur_bd);
                bd.dump();
                break;
            }
            bd_cnt += 1;
            let ctrl = bd.desc.control().read();
            // check that this buffer descriptor is the end of the transaction
            // It is found a eof of a transaction, but maybe there are more buffer descriptor under the hardware.
            if ctrl.eof().is_true() || status.rxeof().is_true() {
                log::trace!("bd_ring::from_hw: EOF found at {}", cur_bd);
                partial_cnt = 0;
            } else {
                partial_cnt += 1;
            }
            // When reach the tail buffer descriptor under the hardware, this loop must be ended.
            if cur_bd == ring.hw_tail {
                break;
            }
            cur_bd += 1;
            if cur_bd == ring.all_cnt {
                cur_bd = 0;
            }
        }
        log::trace!("bd_ring::from_hw: bd_cnt: {}, partial: {}", bd_cnt, partial_cnt);
        bd_cnt -= partial_cnt;
        if bd_cnt > 0 {
            ring.submit_cnt -= bd_cnt;
            ring.free_cnt += bd_cnt;
            ring.hw_head = (ring.hw_head + bd_cnt) % ring.all_cnt;
            log::trace!("bd_ring::from_hw: free_cnt: {}", ring.free_cnt);
        } else {
            log::warn!("bd_ring::from_hw: no completed BD!");
        }
        Ok(())
    }

    /// Wait the channel completing a transaction synchronously.
    pub fn wait(&self) {
        let mut status = self.hardware().dmasr().read();
        while status.ioc_irq().is_no_intr() && status.dly_irq().is_no_intr() && status.err_irq().is_no_intr() {
            status = self.hardware().dmasr().read();
        }
    }

    /// Enable the cyclic mode of this channel
    pub fn cyclic_enable(&self) {
        self.hardware().dmacr().write(|w| w.cyclic_enable().set_bit());
        self.ctrl.lock().is_cyclic = true;
    }

    /// Disable the cyclic mode of this channel
    pub fn cyclic_disable(&self) {
        self.hardware().dmacr().write(|w| w.cyclic_enable().clear_bit());
        self.ctrl.lock().is_cyclic = false;
    }

    /// Disable the interrupt of this channel.
    pub fn intr_disable(&self) {
        log::trace!("channel intr_disable");
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
        log::trace!("channel intr_enable");
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
        status.err_irq().is_detected() || status.dly_irq().is_detected() || status.ioc_irq().is_detected()
    }

    /// The interrupt handler
    pub fn intr_handler(&self) -> AxiDMAResult {
        let mut ctrl = self.ctrl.lock();
        // If the channel disables the interrupt, it will do nothing.
        if ctrl.is_intr_enable {
            let sr = self.hardware().dmasr();
            let status = sr.read();
            if status.err_irq().is_detected() {
                // dump regs
                // reset
                log::trace!("axidma_intr: err intr detected");
                self.dump_regs();
                sr.modify(|_, w| w.err_irq().set_bit());
                return Err(AxiDMAErr::IntrErr);
            }
            if status.ioc_irq().is_detected() {
                log::trace!("axidma_intr: cplt intr detected");
                sr.modify(|_, w| w.ioc_irq().set_bit());
                #[cfg(feature = "async")]
                if let Some(waker) = ctrl.wakers.pop_front() {
                    waker.wake();
                }
            }
            if status.dly_irq().is_detected() {
                log::trace!("axidma_intr: dly intr detected");
                sr.modify(|_, w| w.dly_irq().set_bit());
            }
        }
        Ok(())
    }

    /// Dump the register of channel
    pub fn dump_regs(&self) {
        let hw = self.hardware();
        log::info!(
            "CR: 0b{:b}, SR: 0b{:b}",
            hw.dmacr().read().bits(),
            hw.dmacr().read().bits()
        );
        log::info!(
            "CDESC_MSB: 0x{:x}, CDESC: 0x{:x}",
            hw.curdesc_msb().read().bits(),
            hw.curdesc().read().bits()
        );
        log::info!(
            "TDESC_MSB: 0x{:x}, TDESC: 0x{:x}",
            hw.taildesc_msb().read().bits(),
            hw.taildesc().read().bits()
        );
    }

    /// Update the current buffer descriptor of the channel 
    fn update_cur_bd(&self, addr: usize) {
        let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
        let addr_msb = (addr >> 32) as _;
        unsafe {
            self.hardware().curdesc().write(|w|  w.curdesc_ptr().bits(addr_lsb));
            self.hardware().curdesc_msb().write(|w|  w.curdesc_ptr().bits(addr_msb));
        }
    }

    /// Update the tail buffer descriptor of the channel 
    fn update_tail_bd(&self, addr: usize) {
        let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
        let addr_msb = (addr >> 32) as _;
        unsafe {
            self.hardware().taildesc().write(|w| w.taildesc_ptr().bits(addr_lsb));
            self.hardware().taildesc_msb().write(|w| w.taildesc_ptr().bits(addr_msb));
        }
    }

    /// Get the registers of the channel
    #[inline]
    fn hardware(&self) -> &axidma_pac::channel::RegisterBlock {
        unsafe { &*(self.channel_baseaddr as *const _) }
    }

    /// Check whether a DMA channel is started, meaning the channel is not halted.
    #[allow(unused)]
    fn hw_is_started(&self) -> bool {
        self.hardware().dmasr().read().halted().is_running()
    }
}