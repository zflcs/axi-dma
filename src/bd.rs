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

use axidma_pac::sg_desc::RegisterBlock;
use crate::AxiDMAResult;
use crate::AxiDMAErr;

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

    /// Clear the BD
    pub fn clear(&self) {
        self.desc.buf_addr().reset();
        self.desc.buf_addr_msb().reset();
        self.desc.control().reset();
        for app in self.desc.app_iter() {
            app.reset();
        }
    }

    /// Set the sof field of control for a BD.
    pub fn set_sof(&self) {
        self.desc.control().modify(|_, w| w.sof().set_bit())
    }

    /// Clear the sof field of control for a BD.
    #[allow(unused)]
    pub fn clear_sof(&self) {
        self.desc.control().modify(|_, w| w.sof().clear_bit())
    }

    /// Check the BD is sof
    #[allow(unused)]
    pub fn is_sof(&self) -> bool {
        self.desc.control().read().sof().is_true()
    }

    /// Set the eof field of control for a BD.
    pub fn set_eof(&self) {
        self.desc.control().modify(|_, w| w.eof().set_bit())
    }

    /// Clear the eof field of control for a BD.
    #[allow(unused)]
    pub fn clear_eof(&self) {
        self.desc.control().modify(|_, w| w.eof().clear_bit())
    }

    /// Check the BD is eof
    #[allow(unused)]
    pub fn is_eof(&self) -> bool {
        self.desc.control().read().eof().is_true()
    }

    /// Check the BD is dma_int_err
    #[allow(unused)]
    pub fn is_dma_int_err(&self) -> bool {
        self.desc.status().read().dma_int_err().is_detected()
    }

    /// Check the BD is dma_slv_err
    #[allow(unused)]
    pub fn is_dma_slv_err(&self) -> bool {
        self.desc.status().read().dma_slv_err().is_detected()
    }

    /// Check the BD is dma_dec_err
    #[allow(unused)]
    pub fn is_dma_dec_err(&self) -> bool {
        self.desc.status().read().dma_dec_err().is_detected()
    }

    /// Set the length field for the given BD. 
    /// Length has to be non-zero and less than (1 << 26 - 1).
    /// 
    /// For TX channels, the value passed in should be the number of bytes to
    /// transmit from the TX buffer associated with the given BD.
    /// 
    /// For RX channels, the value passed in should be the size of the RX buffer
    /// associated with the given BD in bytes. This is to notify the RX channel
    /// the capability of the RX buffer to avoid buffer overflow.
    /// 
    /// The actual receive length can be equal or smaller than the specified length.
    /// The actual transfer length will be updated by the hardware in the
    /// `status.tfer_bytes` fields in the BD.
    #[allow(unused)]
    pub fn set_length(&self, length: usize) -> AxiDMAResult {
        if length <= 0 || length > (1 << 26 - 1) {
            log::error!("invalid length {}", length);
            return Err(AxiDMAErr::InValidParam);
        }
        self.desc.control().write(|w| unsafe { w.buf_len().bits(length as _) });
        Ok(())
    }

    /// Retrieve the length field value from the given BD. The returned value is
    /// the same as what was written with XAxiDma_BdSetLength(). Note that in the
    /// this value does not reflect the real length of received data.
    /// See the comments of XAxiDma_BdSetLength() for more details. To obtain the
    /// actual transfer length, use XAxiDma_BdGetActualLength().
    #[allow(unused)]
    pub fn get_length(&self) -> usize {
        self.desc.control().read().buf_len().bits() as _
    }

    /// Get the actual transfer length of a BD. The BD has completed in hw.
    #[allow(unused)]
    pub fn get_actual_length(&self) -> usize {
        self.desc.status().read().tfer_bytes().bits() as _
    }

    /// Set the ID field of the given BD. The ID is an arbitrary piece of data the
    /// application can associate with a specific BD.
    #[allow(unused)]
    pub fn set_id(&mut self, id: u32) {
        self.sw_id = id;
    }

    /// Retrieve the ID field of the given BD previously set with XAxiDma_BdSetId.
    #[allow(unused)]
    pub fn get_id(&self) -> u32 {
        self.sw_id
    }

    /// Set the BD's buffer address.
    /// Note: this function not support dre.
    pub fn set_buf(&self, buf: &[u8]) -> AxiDMAResult {
        let addr = buf.as_ptr() as usize;
        if addr & 0x3 != 0 {
            log::error!("buf is not aligned with 4 byte {:#X}", addr);
            return Err(AxiDMAErr::InValidParam);
        }
        let addr_lsb = (addr & 0xFFFF_FFFF) as _;
        let addr_msb = (addr >> 32) as _;
        log::trace!("bd::set_buf: addr: {:x}, len: {}", addr, buf.len());
        unsafe {
            self.desc.buf_addr().write(|w| w.buf_addr().bits(addr_lsb));
            self.desc.buf_addr_msb().write(|w| w.buf_addr().bits(addr_msb));
            self.set_length(buf.len())?;
        }
        Ok(())
    }

    /// Get the BD's buffer address
    #[allow(unused)]
    pub fn get_buf_addr(&self) -> usize {
        let addr_lsb = self.desc.buf_addr().read().buf_addr().bits() as usize;
        let addr_msb = self.desc.buf_addr_msb().read().buf_addr().bits() as usize;
        (addr_msb << 32) | addr_lsb
    }

    /// Check whether a BD has completed in hardware. This BD has been submitted
    /// to hardware. The application can use this function to poll for the
    /// completion of the BD.
    #[allow(unused)]
    pub fn is_hw_completed(&self) -> bool {
        self.desc.status().read().cmplt().is_true()
    }

    /// Set the APP word at the specified APP word offset for a BD.
    /// This function can be used only when DMA is in SG mode
    #[allow(unused)]
    pub fn set_app_word(&self, appidx: usize, value: usize) -> AxiDMAResult {
        if !self.has_sts_cntrl {
            log::error!("no status control stream in hardware build, cannot set app word");
            return Err(AxiDMAErr::InValidParam);
        }
        if appidx > 4 {
            log::error!("invalid app index {}", appidx);
            return Err(AxiDMAErr::InValidParam);
        }
        self.desc.app(appidx).write(|w| unsafe { w.bits(value as _) });
        Ok(())
    }
    
    /// Get the APP word at the specified APP word offset for a BD.
    /// This function can be used only when DMA is in SG mode
    #[allow(unused)]
    pub fn get_app_word(&self, appidx: usize) -> Result<usize, AxiDMAErr> {
        if !self.has_sts_cntrl {
            log::error!("no status control stream in hardware build, cannot set app word");
            return Err(AxiDMAErr::InValidParam);
        }
        if appidx > 4 {
            log::error!("invalid app index {}", appidx);
            return Err(AxiDMAErr::InValidParam);
        }
        Ok(self.desc.app(appidx).read().bits() as _)
    }

    /// Dump the fields of a BD.
    pub fn dump(&self) {
        let d = &self.desc;
        log::trace!(
            "NXT_DESC_MSB: 0x{:x}, NXT_DESC: 0x{:x}",
            d.nxt_desc_msb().read().bits(),
            d.nxt_desc().read().bits()
        );
        log::trace!(
            "BUF_ADDR_MSB: 0x{:x}, BUF_ADDR: 0x{:x}",
            d.buf_addr_msb().read().bits(),
            d.buf_addr().read().bits()
        );
        log::trace!(
            "CONTROL: 0x{:x}, STATUS: 0x{:x}",
            d.control().read().bits(),
            d.status().read().bits()
        );
    }

}