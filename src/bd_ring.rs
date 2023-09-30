use crate::io_fence;

use super::bd::AxiDmaBlockDesc;
use alloc::{boxed::Box, collections::VecDeque};
use core::{
    ops::Deref,
    pin::Pin,
    sync::atomic::{compiler_fence, fence, Ordering::SeqCst},
};

pub(super) struct AxiDmaBdRingConfig {
    #[allow(unused)]
    pub chan_base_addr: usize,
    #[allow(unused)]
    pub is_rx_chan: bool,
    pub has_sts_cntrl_strm: bool,
    pub has_dre: bool,
    pub data_width: usize,
    #[allow(unused)]
    pub addr_ext: bool,
    pub max_transfer_len: usize,
}

pub(super) struct AxiDmaBdRing {
    config: AxiDmaBdRingConfig,

    pub(super) is_halted: bool,

    ring: VecDeque<Pin<Box<AxiDmaBlockDesc>>>,

    bd_head: usize,
    bd_tail: usize,
    bd_restart: usize,

    free_cnt: usize,
    pub(super) pending_cnt: usize,
    pub(super) submit_cnt: usize,
    done_cnt: usize,
    all_cnt: usize,
    #[allow(unused)]
    cyclic: usize,
}

impl AxiDmaBdRing {
    pub fn new(config: AxiDmaBdRingConfig) -> Self {
        Self {
            config,
            is_halted: true,
            ring: VecDeque::new(),
            bd_head: 0,
            bd_tail: 0,
            bd_restart: 0,
            free_cnt: 0,
            pending_cnt: 0,
            submit_cnt: 0,
            done_cnt: 0,
            all_cnt: 0,
            cyclic: 0,
        }
    }

    #[allow(unused)]
    pub fn snaphot_curr_bd(&self) {
        todo!();
    }

    pub fn start(&mut self) -> Result<(), ()> {
        todo!()
    }

    // pub fn create(&mut self, phys_addr: usize, virt_addr: usize, align: usize, bd_count: usize) {
    pub fn create(&mut self, bd_count: usize) {
        trace!("bd_ring::create: creating ring with {} BD", bd_count);
        self.all_cnt = 0;
        self.free_cnt = 0;
        self.pending_cnt = 0;
        self.submit_cnt = 0;
        self.done_cnt = 0;
        self.ring.clear();

        self.ring.reserve(bd_count);
        for _ in 0..bd_count {
            let bd = Box::pin(AxiDmaBlockDesc::new(
                self.config.has_sts_cntrl_strm,
                self.config.has_dre,
                self.config.data_width as _,
            ));
            self.ring.push_back(bd);
        }
        // link bd chain
        for i in 0..bd_count {
            let next_addr = &self.ring[(i + 1) % bd_count].desc as *const _ as usize;
            self.ring[i].set_next_desc_addr(next_addr);
            // trace!("bd_ring::create: bd: {}, next_addr: 0x{:x}", i, next_addr);
        }

        self.is_halted = true;
        self.all_cnt = bd_count;
        self.free_cnt = bd_count;
        self.bd_head = 0;
        self.bd_tail = 0;
        self.bd_restart = 0;
    }

    pub fn submit<B>(&mut self, buffer: &Pin<B>) 
    where
        B: Deref,
        B::Target: AsRef<[u8]> + 'static,
    {
        let buf = (**buffer).as_ref();
        let start = self.bd_restart;
        let mut buf_len = buf.len();
        let mut buf_head = 0;
        let mut bd_len = self.config.max_transfer_len;
        let bd_cnt = (buf_len + bd_len - 1) / bd_len;
        if bd_cnt > self.free_cnt {
            error!("bd_ring::rx_submit: too many BD required!, bd_cnt {}, free_cnt {}", bd_cnt, self.free_cnt);
        }
        trace!(
            "bd_ring::rx_submit: buf_len: {}, bd_cnt: {}, restart: {}",
            buf_len, bd_cnt, self.bd_restart
        );
        for _ in 0..bd_cnt {
            let bd = &self.ring[self.bd_restart];
            bd.clear();
            if buf_len < bd_len {
                bd_len = buf_len;
            }
            bd.set_buf(&buf[buf_head..buf_head + bd_len]);
            let peek_len = 16.min(bd_len);
            trace!(
                "bd_ring::rx_submit: peek buf[{}..{}]: {:x?}",
                buf_head,
                buf_head + peek_len,
                &buf[buf_head..buf_head + peek_len]
            );
            buf_head += bd_len;
            buf_len -= bd_len;
            self.bd_restart += 1;
            if self.bd_restart == self.all_cnt {
                self.bd_restart = 0;
            }
        }
        self.bd_tail = if self.bd_restart == 0 {
            self.ring.len() - 1
        } else {
            self.bd_restart - 1
        };
        self.ring[start]
            .desc
            .control
            .modify(|_, w| w.sof().set_bit());
        self.ring[self.bd_tail]
            .desc
            .control
            .modify(|_, w| w.eof().set_bit());

        self.free_cnt -= bd_cnt;
        self.pending_cnt += bd_cnt;
        trace!(
            "bd_ring::rx_submit: done, restart: {}, head: {}, tail: {}, free: {}, pending: {}",
            self.bd_restart, self.bd_head, self.bd_tail, self.free_cnt, self.pending_cnt
        );
    }


    pub fn head_desc_addr(&self) -> usize {
        &self.ring[self.bd_head].desc as *const _ as usize
    }

    pub fn tail_desc_addr(&self) -> usize {
        &self.ring[self.bd_tail].desc as *const _ as usize
    }

    pub fn from_hw(&mut self) {
        let mut bd_cnt = 0;
        let mut partial_cnt = 0;
        let mut cur_bd = self.bd_head;
        trace!(
            "bd_ring::tx_from_hw: head: {}, tail: {}",
            self.bd_head, self.bd_tail
        );
        compiler_fence(SeqCst);
        fence(SeqCst);
        io_fence();

        loop {
            let bd = &self.ring[cur_bd];
            // unsafe { ebreak() };
            let status = bd.desc.status.read();
            if status.cmplt().is_false() {
                // unsafe { ebreak() };
                trace!("bd_ring::tx_from_hw: Uncompleted BD found at {}", cur_bd);
                bd.dump();
                break;
            }
            bd_cnt += 1;
            let ctrl = bd.desc.control.read();
            if ctrl.eof().is_true() || status.rxeof().is_true() {
                trace!("bd_ring::tx_from_hw: EOF found at {}", cur_bd);
                partial_cnt = 0;
            } else {
                partial_cnt += 1;
            }
            if cur_bd == self.bd_tail {
                break;
            }
            cur_bd += 1;
            if cur_bd == self.all_cnt {
                cur_bd = 0;
            }
        }
        trace!(
            "bd_ring::tx_from_hw: bd_cnt: {}, partial: {}",
            bd_cnt, partial_cnt
        );
        bd_cnt -= partial_cnt;
        if bd_cnt > 0 {
            self.submit_cnt -= bd_cnt;
            self.free_cnt += bd_cnt;
            self.bd_head = (self.bd_head + bd_cnt) % self.all_cnt;
            trace!("bd_ring::tx_from_hw: free_cnt: {}", self.free_cnt);
        } else {
            warn!("bd_ring::tx_from_hw: no completed BD!");
        }
    }
}