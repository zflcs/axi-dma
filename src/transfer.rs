use alloc::sync::Arc;
use core::{
    hint,
    sync::atomic::{self, Ordering},
};

use crate::{channel::AxiDMAChannel, errno::AxiDMAErr, BufPtr};

/// The transfer structure of a transaction. It is combined with the AxiDMA channel.
/// It also combined the Rust memory mode, when the buffer has been transported to the hardware,
/// while this transaction has not been finished. The target buffer must be pinned and cannot be droppped.
/// The target buffer cannot be taken from this transfer structure until the transaction is finished.
/// Then it can be dropped or used for other function.
pub struct Transfer {
    // NOTE: always `Some` variant
    buffer: Option<BufPtr>,
    // The channel related with the transfer
    channel: Arc<AxiDMAChannel>,
    /// Completed flag
    #[cfg(feature = "async")]
    flag: bool,
}

impl Transfer {
    /// Create a new tranfer
    pub fn new(buf: BufPtr, channel: Arc<AxiDMAChannel>) -> Self {
        Self {
            buffer: Some(buf),
            channel,
            #[cfg(feature = "async")]
            flag: false,
        }
    }

    /// Blocks until the transfer is done and returns the buffer, the
    pub fn wait(mut self) -> Result<BufPtr, AxiDMAErr> {
        self.channel.wait();
        atomic::compiler_fence(Ordering::SeqCst);
        self.channel.from_hw()?;
        // Deal the interrupt
        self.channel.intr_handler()?;
        Ok(self
            .buffer
            .take()
            .unwrap_or_else(|| unsafe { hint::unreachable_unchecked() }))
    }
}

#[cfg(feature = "async")]
impl Unpin for Transfer {}

#[cfg(feature = "async")]
use core::{
    future::Future,
    task::{Context, Poll},
};

#[cfg(feature = "async")]
impl Future for Transfer {
    type Output = BufPtr;
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
        let buf = self
            .buffer
            .take()
            .unwrap_or_else(|| unsafe { hint::unreachable_unchecked() });
        let _ = self.channel.from_hw();
        return Poll::Ready(buf);
    }
}
