use core::ptr::NonNull;

/// A raw buffer struct.
pub struct BufPtr {
    // The pointer to the buffer.
    buf_ptr: NonNull<u8>,
    len: usize,
}

impl BufPtr {
    /// Create a new [`NetBufPtr`].
    pub fn new(buf_ptr: NonNull<u8>, len: usize) -> Self {
        Self { buf_ptr, len }
    }

    /// Return [`BufPtr`] buffer ptr.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.buf_ptr.as_ptr()
    }

    /// Return [`BufPtr`] buffer mut ptr.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        unsafe { self.buf_ptr.as_mut() }
    }

    /// Return [`BufPtr`] buffer len.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return [`NetBufPtr`] buffer as &[u8].
    #[inline]
    pub fn packet(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.buf_ptr.as_ptr() as *const u8, self.len) }
    }

    /// Return [`NetBufPtr`] buffer as &mut [u8].
    #[inline]
    pub fn packet_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.buf_ptr.as_ptr(), self.len) }
    }
}
