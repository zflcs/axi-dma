use alloc::sync::Arc;
use core::hint;

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
        self.channel.from_hw()?;
        // Deal the interrupt
        self.channel.intr_handler()?;
        Ok(self
            .buffer
            .take()
            .unwrap_or_else(|| unsafe { hint::unreachable_unchecked() }))
    }

    /// Blocks until the transfer is done and returns the buffer, the
    pub fn recycle(mut self) -> Result<BufPtr, AxiDMAErr> {
        self.channel.from_hw()?;
        Ok(self
            .buffer
            .take()
            .unwrap_or_else(|| unsafe { hint::unreachable_unchecked() }))
    }
}

#[cfg(not(feature = "driver_test"))]
impl Drop for Transfer {
    fn drop(&mut self) {
        let mut bufptr = self.buffer.take().unwrap_or_else(|| unsafe { hint::unreachable_unchecked() });
        let len = bufptr.len();
        let raw_ptr = bufptr.as_mut_ptr();
        let slice = unsafe {
            core::slice::from_raw_parts_mut(raw_ptr, len)
        };
        let _buf = unsafe { alloc::boxed::Box::from_raw(slice) };
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
use core::pin::Pin;

#[cfg(feature = "async")]
impl Future for Transfer {
    type Output = BufPtr;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.flag {
            self.flag = true;
            return Poll::Pending;
        }
        let _ = self.channel.from_hw().unwrap();
        let _ = self.channel.intr_handler().unwrap();
        let buf = self
            .buffer
            .take()
            .unwrap_or_else(|| unsafe { hint::unreachable_unchecked() });
        return Poll::Ready(buf);
    }
}
