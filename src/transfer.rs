use core::{ops::Deref, pin::Pin, sync::atomic::{self, Ordering}, hint};
use alloc::sync::Arc;

use crate::AxiDma;

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
        self.buffer.take().unwrap_or_else(|| unsafe { hint::unreachable_unchecked() })

    }
}