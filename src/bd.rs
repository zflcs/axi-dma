//! Buffer descriptor (BD) management API.
//!
//! ### Buffer Descriptors
//!
//! A BD defines a DMA transaction (see "Transaction" section in lib.rs).
//! All accesses to a BD go through this set of API.
//!
//! ### Actual Transfer Length
//!
//! The actual transfer length for receive could be smaller than the requested
//! transfer length. The hardware sets the actual transfer length in the
//! completed BD. The API to retrieve the actual transfer length is
//! XAxiDma_GetActualLength().
//!
//! ### User IP words
//!
//! There are 5 user IP words in each BD.
//!
//! If hardware does not have the StsCntrl stream built in, then these words
//! are not usable. Retrieving these words get a NULL pointer and setting
//! these words results an error.
//!
//! ### Performance
//!
//! BDs are typically in a non-cached memory space. Reducing the number of
//! I/O operations to BDs can improve overall performance of the DMA channel.
//!
//!
//! Not Completed funtions:
//!     - [ ] Multiple Channel
//!     - [ ] Micro Mode

use crate::AxiDMAErr;
use crate::AxiDMAResult;
use crate::BufPtr;
use axidma_pac::sg_desc::RegisterBlock;

/// The AxiDmaBD is the type for a buffer descriptor (BD).
#[repr(C, align(64))]
pub struct AxiDmaBD {
    pub desc: RegisterBlock,
    pub sw_id: u32,
    pub has_sts_cntrl: bool,
    pub has_dre: bool,
    pub word_len: u32,
}

impl AxiDmaBD {
    ///
    pub fn new(has_sts_cntrl: bool, has_dre: bool, word_len: u32) -> Self {
        Self {
            desc: RegisterBlock::default(),
            sw_id: 0,
            has_sts_cntrl,
            has_dre,
            word_len,
        }
    }
    /// Clear the BD
    pub fn clear(&self) {
        self.desc.buf_addr().reset();
        self.desc.buf_addr_msb().reset();
        self.desc.control().reset();
        for app in self.desc.app_iter() {
            app.reset();
        }
    }
    /// Link the next BD
    pub fn set_next_desc_addr(&self, addr: usize) {
        let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
        let addr_msb = (addr >> 32) as _;
        unsafe {
            self.desc
                .nxt_desc()
                .write(|w| w.nxt_desc_ptr().bits(addr_lsb));
            self.desc
                .nxt_desc_msb()
                .write(|w| w.nxt_desc_ptr().bits(addr_msb));
        }
    }

    /// Set the BD's buffer address.
    /// Note: this function not support dre.
    pub fn set_buf(&self, buf: &BufPtr) -> AxiDMAResult {
        let addr = buf.as_ptr() as usize;
        if addr & 0x3 != 0 {
            log::error!("buf is not aligned with 4 byte {:#X}", addr);
            return Err(AxiDMAErr::InValidParam);
        }
        let addr_lsb = (addr & 0xFFFF_FFFF) as _;
        let addr_msb = (addr >> 32) as _;
        trace!("bd::set_buf: addr: {:x}, len: {}", addr, buf.len());
        unsafe {
            self.desc.buf_addr().write(|w| w.buf_addr().bits(addr_lsb));
            self.desc
                .buf_addr_msb()
                .write(|w| w.buf_addr().bits(addr_msb));
            self.desc
                .control()
                .modify(|_, w| w.buf_len().bits(buf.len() as _));
        }
        Ok(())
    }
    /// Dump the fields of a BD.
    #[allow(unused)]
    pub fn dump(&self) {
        let d = &self.desc;
        info!(
            "NXT_DESC_MSB: 0x{:x}, NXT_DESC: 0x{:x}",
            d.nxt_desc_msb().read().bits(),
            d.nxt_desc().read().bits()
        );
        info!(
            "BUF_ADDR_MSB: 0x{:x}, BUF_ADDR: 0x{:x}",
            d.buf_addr_msb().read().bits(),
            d.buf_addr().read().bits()
        );
        info!(
            "CONTROL: 0x{:x}, STATUS: 0x{:x}",
            d.control().read().bits(),
            d.status().read().bits()
        );
    }
}
