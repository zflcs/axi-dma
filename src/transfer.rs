use core::{hint, ops::Deref, pin::Pin, sync::atomic::{self, Ordering}};
use alloc::sync::Arc;

use crate::{errno::AxiDMAErr, channel::AxiDMAChannel};

/// The transfer structure of a transaction. It is combined with the AxiDMA channel.
/// It also combined the Rust memory mode, when the buffer has been transported to the hardware,
/// while this transaction has not been finished. The target buffer must be pinned and cannot be droppped.
/// The target buffer cannot be taken from this transfer structure until the transaction is finished. 
/// Then it can be dropped or used for other function.
pub struct Transfer<B> 
where
        B: Deref,
        B::Target: AsRef<[u8]> + 'static,
{
    // NOTE: always `Some` variant
    buffer: Option<Pin<B>>,
    // The channel related with the transfer
    channel: Arc<AxiDMAChannel>,
    /// Completed flag
    #[cfg(feature = "async")]
    flag: bool,
}

impl<B> Transfer<B> 
where
    B: Deref,
    B::Target: AsRef<[u8]>,
{

    /// Create a new tranfer
    pub fn new(buf: Pin<B>, channel: Arc<AxiDMAChannel>) -> Self {
        Self { 
            buffer: Some(buf), 
            channel,
            #[cfg(feature = "async")]
            flag: false
        }
    }

    /// Blocks until the transfer is done and returns the buffer, the 
    pub fn wait(mut self) -> Result<Pin<B>, AxiDMAErr> {
        self.channel.wait();
        atomic::compiler_fence(Ordering::SeqCst);
        self.channel.from_hw()?;
        // Deal the interrupt
        self.channel.intr_handler()?;
        Ok(self.buffer.take().unwrap_or_else(|| unsafe { hint::unreachable_unchecked() }))
    }
}

#[cfg(feature = "async")]
impl<B> Unpin for Transfer<B> 
where
    B: Deref,
    B::Target: AsRef<[u8]> + 'static
{}

#[cfg(feature = "async")]
use core::{future::Future, task::{Poll, Context}};

#[cfg(feature = "async")]
impl<B> Future for Transfer<B> 
where
    B: Deref,
    B::Target: AsRef<[u8]>,
{
    type Output = Pin<B>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        
        if !self.flag {
            if !self.channel.check_cmplt() {
                let waker = cx.waker();
                self.channel.ctrl.lock().wakers.push_back(waker.clone());
                log::trace!("async transfer pending");
                self.flag = true;
                return Poll::Pending;
            } 
        }
        log::trace!("async transfer ready");
        atomic::compiler_fence(Ordering::SeqCst);
        let buf = self.buffer.take().unwrap_or_else(|| unsafe { hint::unreachable_unchecked() });
        let _ = self.channel.from_hw();
        return Poll::Ready(buf);
    }
}