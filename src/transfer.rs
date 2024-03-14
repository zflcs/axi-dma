use core::{hint, ops::Deref, pin::Pin, sync::atomic::{self, Ordering}};
use alloc::sync::Arc;

use crate::{errno::AxiDMAErr, AxiDma};


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
    pub fn wait(mut self) -> Result<Pin<B>, AxiDMAErr> {
        self.dma.rx_wait();
        atomic::compiler_fence(Ordering::SeqCst);
        self.dma.rx_from_hw()?;
        Ok(self.buffer.take().unwrap_or_else(|| unsafe { hint::unreachable_unchecked() }))
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
    pub fn wait(mut self) -> Result<Pin<B>, AxiDMAErr> {
        self.dma.tx_wait();
        atomic::compiler_fence(Ordering::SeqCst);
        self.dma.tx_from_hw()?;
        Ok(self.buffer.take().unwrap_or_else(|| unsafe { hint::unreachable_unchecked() }))

    }
}