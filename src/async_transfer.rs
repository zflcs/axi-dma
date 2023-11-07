/// This mod provide a async buffer.
/// When the buffer is submitted, it don't need to wait for DMA to transfer complete.
/// While the DMA tranfer complete, the buffer is dropped.

use core::{ops::Deref, pin::Pin, sync::atomic::{self, Ordering, compiler_fence, fence}, hint, future::Future, task::{Context, Poll}};
use alloc::sync::Arc;

use crate::{AxiDma, io_fence};

pub struct AsyncRxTransfer<B> 
where
        B: Deref,
        B::Target: AsRef<[u8]> + 'static,
{
    // NOTE: always `Some` variant
    buffer: Option<Pin<B>>,
    dma: Arc<AxiDma>,
    flag: bool,
}

impl<B> Unpin for AsyncRxTransfer<B> 
where
    B: Deref,
    B::Target: AsRef<[u8]> + 'static
{}

impl<B> Future for AsyncRxTransfer<B> 
where
    B: Deref,
    B::Target: AsRef<[u8]>,
{
    type Output = Pin<B>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.flag {
            let waker = cx.waker();
            self.dma.rx_wakers.lock().push_back(waker.clone());
            self.flag = true;
            Poll::Pending
        } else {
            atomic::compiler_fence(Ordering::SeqCst);
            let buf = self.buffer.take().unwrap_or_else(|| unsafe { hint::unreachable_unchecked() });
            self.dma.rx_intr_handler();
            self.dma.rx_from_hw();
            Poll::Ready(buf)
        }
    }
}

impl<B> AsyncRxTransfer<B> 
where
    B: Deref,
    B::Target: AsRef<[u8]>,
{
    pub fn new(buf: Pin<B>, dma: Arc<AxiDma>) -> Self {
        Self { buffer: Some(buf), dma, flag: false }
    }
}

pub struct AsyncTxTransfer<B> 
where
        B: Deref,
        B::Target: AsRef<[u8]> + 'static,
{
    // NOTE: always `Some` variant
    buffer: Option<Pin<B>>,
    dma: Arc<AxiDma>,
    flag: bool,
}

impl<B> Unpin for AsyncTxTransfer<B> 
where
    B: Deref,
    B::Target: AsRef<[u8]> + 'static
{}

impl<B> Future for AsyncTxTransfer<B> 
where
    B: Deref,
    B::Target: AsRef<[u8]>,
{
    type Output = Pin<B>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.flag {
            let waker = cx.waker();
            self.dma.tx_wakers.lock().push_back(waker.clone());
            log::debug!("async transfer pending");
            self.flag = true;
            Poll::Pending
        } else {
            log::debug!("async transfer ready");
            atomic::compiler_fence(Ordering::SeqCst);
            let buf = self.buffer.take().unwrap_or_else(|| unsafe { hint::unreachable_unchecked() });
            self.dma.tx_intr_handler();
            self.dma.tx_from_hw();
            Poll::Ready(buf)
        }
    }
}

impl<B> AsyncTxTransfer<B> 
where
    B: Deref,
    B::Target: AsRef<[u8]>,
{
    pub fn new(buf: Pin<B>, dma: Arc<AxiDma>) -> Self {
        Self { buffer: Some(buf), dma, flag: false }
    }
}


impl AxiDma {

    pub fn tx_submit<B>(self: &Arc<Self>, buf: Pin<B>) -> Option<AsyncTxTransfer<B>>
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
            Some(AsyncTxTransfer::new(buf, self.clone()))
        } else {
            trace!("axidma::tx_submit: no tx ring!");
            None
        }
    }

    pub fn rx_submit<B>(self: &Arc<Self>, buf: Pin<B>) -> Option<AsyncRxTransfer<B>>
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
            Some(AsyncRxTransfer::new(buf, self.clone()))
        } else {
            trace!("axidma::rx_submit: no rx ring!");
            None
        }
    }

    pub fn tx_complete(self: &Arc<Self>) -> bool {
        let status = self.hardware().mm2s_dmasr.read();
        !(status.ioc_irq().is_no_intr() && status.dly_irq().is_no_intr() && status.err_irq().is_no_intr())
    }


    pub fn rx_complete(self: &Arc<Self>) -> bool {
        let status = self.hardware().s2mm_dmasr.read();
        !(status.ioc_irq().is_no_intr() && status.dly_irq().is_no_intr() && status.err_irq().is_no_intr())
    }

    pub fn tx_intr_handler(self: &Arc<Self>) -> bool {
        let sr = &self.hardware().mm2s_dmasr;
        if sr.read().err_irq().is_detected() {
            // dump regs
            // reset
            error!("axidma_intr: tx err intr detected");
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
}