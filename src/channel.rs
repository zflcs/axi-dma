//! This file contains DMA channel related structure and constant definition
//! as well as function prototypes. Each DMA channel is managed by a Buffer
//! Descriptor ring. See [xaxidma.h](https://github.com/Xilinx/embeddedsw/blob/master/XilinxProcessorIPLib/drivers/axidma/src/xaxidma.h) 
//! for more information on how a BD ring is managed.
//! 

use alloc::collections::VecDeque;
use core::{
    ops::Deref,
    pin::Pin,
    sync::atomic::{compiler_fence, fence, Ordering::{self, SeqCst}},
};
use alloc::boxed::Box;
use crate::{bd::AxiDmaBD, errno::AxiDMAErr, io_fence, AxiDMAResult, AxiDmaConfig};

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
    /// The base address of register space of Channel
    channel_baseaddr: usize,
    /// Whether channel is halted
    pub is_halted: bool,
    /// Whether has stscntrl stream
    pub has_sts_cntrl_strm: bool,
    /// Whether the Data Realignment Enable
    pub has_dre: bool,
    /// 
    pub data_width: usize,
    /// 
    pub max_transfer_len: usize,
    /// BD ring
    ring: VecDeque<Pin<Box<AxiDmaBD>>>,
    /// The index of first BD in the work group
    hw_head: usize,
    /// The index of last BD in the work group
    hw_tail: usize,
    /// BD to load when channel is started
    bd_restart: usize,
    /// Check for cyclic DMA Mode
    #[allow(unused)]
    is_cyclic: bool,
    /// Number of BDs in free group
    free_cnt: usize,
    /// Total Number of BDs for channel
    all_cnt: usize,
    /// Used for multiple transaction
    pending_cnt: usize,
    /// Used for multiple transaction
    submit_cnt: usize,
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
            is_halted: true, 
            has_sts_cntrl_strm: cfg.has_sts_cntrl_strm, 
            has_dre,
            data_width, 
            max_transfer_len, 
            ring: VecDeque::new(), 
            hw_head: 0, 
            hw_tail: 0, 
            bd_restart: 0, 
            is_cyclic: false, 
            free_cnt: 0, 
            all_cnt: 0, 
            channel_baseaddr,
            pending_cnt: 0,
            submit_cnt: 0,
        }
    }

    /// Creates and setup the BD ring.
    pub fn create(&mut self, bd_count: usize) -> AxiDMAResult {
        if bd_count <= 0 {
            log::error!("non-positive BD number {}", bd_count);
            return Err(AxiDMAErr::InValidParam);
        }
        self.ring.clear();
        self.ring.reserve(bd_count);
        for _ in 0..bd_count {
            let bd = Box::pin(AxiDmaBD::new(
                self.has_sts_cntrl_strm,
                self.has_dre,
                self.data_width as _,
            ));
            self.ring.push_back(bd);
        }
        // link bd chain
        for i in 0..bd_count {
            let next_addr = &self.ring[(i + 1) % bd_count].desc as *const _ as usize;
            self.ring[i].set_next_desc_addr(next_addr);
        }
        self.pending_cnt = 0;
        self.submit_cnt = 0;
        self.is_halted = true;
        self.all_cnt = bd_count;
        self.free_cnt = bd_count;
        self.hw_head = 0;
        self.hw_tail = 0;
        self.bd_restart = 0;
        Ok(())
    }

    /// Reset this channel.
    pub fn reset(&mut self) -> AxiDMAResult {
        self.hardware().dmacr().modify(|_, w| w.reset().reset());
        self.is_halted = true;
        Ok(())
    }

    /// Start this channel.
    pub fn start(&mut self) -> AxiDMAResult {
        self.hardware().dmacr().modify(|_, w| w.run_stop().run());
        self.is_halted = false;
        Ok(())
    }

    /// Stop this channel.
    pub fn stop(&mut self) -> AxiDMAResult {
        self.hardware().dmacr().modify(|_, w| w.run_stop().stop());
        self.is_halted = true;
        Ok(())
    }

    // Check whether reset is done when both went normal
    pub fn reset_is_done(&self) -> bool {
        if self.hardware().dmacr().read().reset().is_reset() {
            return false;
        }
        true
    }

    /// Submit buffer to the DMA channel, and start a transaction.
    /// In order to ensure the transaction successful, the buffer must be pinned.
    pub fn submit<B>(&mut self, buffer: &Pin<B>) -> AxiDMAResult
    where
        B: Deref,
        B::Target: AsRef<[u8]>,
    {
        let buf = (**buffer).as_ref();
        self.hw_head = self.bd_restart;
        let mut buf_len = buf.len();
        let mut buf_head = 0;
        let mut bd_len = self.max_transfer_len;
        log::trace!("max_transfer_len: {}", self.max_transfer_len);
        let bd_cnt = (buf_len + bd_len - 1) / bd_len;
        if bd_cnt > self.free_cnt {
            log::error!("bd_ring::submit: too many BD required!, bd_cnt {}, free_cnt {}", bd_cnt, self.free_cnt);
            return Err(AxiDMAErr::DMAErr);
        }
        log::trace!("bd_ring::submit: buf_len: {}, bd_cnt: {}, restart: {}", buf_len, bd_cnt, self.bd_restart);
        for _ in 0..bd_cnt {
            let bd = &self.ring[self.bd_restart];
            bd.clear();
            if buf_len < bd_len {
                bd_len = buf_len;
            }
            bd.set_buf(&buf[buf_head..buf_head + bd_len])?;
            let peek_len = 16.min(bd_len);
            log::trace!("bd_ring::submit: peek buf[{}..{}]: {:x?}", buf_head, buf_head + peek_len, &buf[buf_head..buf_head + peek_len]);
            buf_head += bd_len;
            buf_len -= bd_len;
            self.bd_restart += 1;
            if self.bd_restart == self.all_cnt {
                self.bd_restart = 0;
            }
        }
        self.hw_tail = if self.bd_restart == 0 {
            self.ring.len() - 1
        } else {
            self.bd_restart - 1
        };
        // Set the flag of the start and end buffer descriptor.
        self.ring[self.hw_head].set_sof();
        self.ring[self.hw_tail].set_eof();
        self.free_cnt -= bd_cnt;
        self.pending_cnt += bd_cnt;
        // The current buffer descriptor can be updated only when the channel is halted.
        if self.is_halted {
            self.update_cur_bd();
        }
        compiler_fence(Ordering::SeqCst);
        fence(Ordering::SeqCst);
        io_fence();
        // Start the channel
        self.hardware().dmacr().modify(|_, w| w.run_stop().run());
        self.is_halted = false;
        // If there are some buffer descriptor are pended, the tail buffer descriptor must be updated.
        // For example, there is a new transaction happend when the last transaction is not finished.
        // The tail buffer descriptor must be updated on time.
        if self.pending_cnt > 0 {
            log::trace!("pending_cnt > 0, update tail bd");
            self.submit_cnt += self.pending_cnt;
            self.pending_cnt = 0;
            self.update_tail_bd();
        }
        log::trace!(
            "bd_ring::submit: done, restart: {}, head: {}, tail: {}, free: {}",
            self.bd_restart, self.hw_head, self.hw_tail, self.free_cnt
        );
        Ok(())
    }

    /// Retrieve the BD from hardware
    pub fn from_hw(&mut self) -> AxiDMAResult {
        let mut bd_cnt = 0;
        let mut partial_cnt = 0;
        let mut cur_bd = self.hw_head;
        log::trace!("bd_ring::from_hw: head: {}, tail: {}",self.hw_head, self.hw_tail);
        compiler_fence(SeqCst);
        fence(SeqCst);
        io_fence();
        loop {
            let bd = &self.ring[cur_bd];
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
            if cur_bd == self.hw_tail {
                break;
            }
            cur_bd += 1;
            if cur_bd == self.all_cnt {
                cur_bd = 0;
            }
        }
        log::trace!("bd_ring::from_hw: bd_cnt: {}, partial: {}", bd_cnt, partial_cnt);
        bd_cnt -= partial_cnt;
        if bd_cnt > 0 {
            self.submit_cnt -= bd_cnt;
            self.free_cnt += bd_cnt;
            self.hw_head = (self.hw_head + bd_cnt) % self.all_cnt;
            log::trace!("bd_ring::from_hw: free_cnt: {}", self.free_cnt);
        } else {
            log::warn!("bd_ring::from_hw: no completed BD!");
        }
        Ok(())
    }

    /// Wait the channel completing a transaction synchronously.
    #[allow(unused)]
    pub fn wait(&self) {
        let mut status = self.hardware().dmasr().read();
        while status.ioc_irq().is_no_intr() && status.dly_irq().is_no_intr() && status.err_irq().is_no_intr() {
            status = self.hardware().dmasr().read();
        }
    }

    /// Enable the cyclic mode of this channel
    pub fn cyclic_enable(&mut self) {
        self.hardware().dmacr().write(|w| w.cyclic_enable().set_bit());
        self.is_cyclic = true;
    }

    /// Disable the cyclic mode of this channel
    pub fn cyclic_disable(&mut self) {
        self.hardware().dmacr().write(|w| w.cyclic_enable().clear_bit());
        self.is_cyclic = false;
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
        })
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
    }


    /// Update the current buffer descriptor of the channel 
    fn update_cur_bd(&self) {
        let addr = self.head_desc_addr();
        let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
        let addr_msb = (addr >> 32) as _;
        unsafe {
            self.hardware().curdesc().write(|w|  w.curdesc_ptr().bits(addr_lsb));
            self.hardware().curdesc_msb().write(|w|  w.curdesc_ptr().bits(addr_msb));
        }
    }

    /// Update the tail buffer descriptor of the channel 
    fn update_tail_bd(&self) {
        let addr = self.tail_desc_addr();
        let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
        let addr_msb = (addr >> 32) as _;
        unsafe {
            self.hardware().taildesc().write(|w| w.taildesc_ptr().bits(addr_lsb));
            self.hardware().taildesc_msb().write(|w| w.taildesc_ptr().bits(addr_msb));
        }
    }

    /// the pointer of head buffer descriptor in the work group
    fn head_desc_addr(&self) -> usize {
        &self.ring[self.hw_head].desc as *const _ as usize
    }

    /// the pointer of tail buffer descriptor in the work group
    fn tail_desc_addr(&self) -> usize {
        &self.ring[self.hw_tail].desc as *const _ as usize
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