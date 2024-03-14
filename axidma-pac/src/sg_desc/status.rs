#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `tfer_bytes` reader - This value indicates the amount of data received and stored in the buffer described by this descriptor. This might or might not match the buffer length."]
pub type TferBytesR = crate::FieldReader<u32>;
#[doc = "End of Frame. Flag indicating buffer holds the last part of packet. This bit is set by AXI DMA to indicate to the sw/user that the buffer associated with this descriptor contains the end of the packet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxeof {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Rxeof> for bool {
    #[inline(always)]
    fn from(variant: Rxeof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rxeof` reader - End of Frame. Flag indicating buffer holds the last part of packet. This bit is set by AXI DMA to indicate to the sw/user that the buffer associated with this descriptor contains the end of the packet."]
pub type RxeofR = crate::BitReader<Rxeof>;
impl RxeofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxeof {
        match self.bits {
            false => Rxeof::False,
            true => Rxeof::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Rxeof::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Rxeof::True
    }
}
#[doc = "Start of Frame. Flag indicating buffer holds first part of packet. This bit is set by AXI DMA to indicate to the sw/user that the buffer associated with this descriptor contains the start of the packet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxsof {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Rxsof> for bool {
    #[inline(always)]
    fn from(variant: Rxsof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rxsof` reader - Start of Frame. Flag indicating buffer holds first part of packet. This bit is set by AXI DMA to indicate to the sw/user that the buffer associated with this descriptor contains the start of the packet."]
pub type RxsofR = crate::BitReader<Rxsof>;
impl RxsofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxsof {
        match self.bits {
            false => Rxsof::False,
            true => Rxsof::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Rxsof::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Rxsof::True
    }
}
#[doc = "DMA Internal Error. Internal Error detected by primary AXI DataMover. This error can occur if a 0 length bytes to transfer is fed to the AXI DataMover.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaIntErr {
    #[doc = "0: `0`"]
    NoErr = 0,
    #[doc = "1: `1`"]
    Detected = 1,
}
impl From<DmaIntErr> for bool {
    #[inline(always)]
    fn from(variant: DmaIntErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dma_int_err` reader - DMA Internal Error. Internal Error detected by primary AXI DataMover. This error can occur if a 0 length bytes to transfer is fed to the AXI DataMover."]
pub type DmaIntErrR = crate::BitReader<DmaIntErr>;
impl DmaIntErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaIntErr {
        match self.bits {
            false => DmaIntErr::NoErr,
            true => DmaIntErr::Detected,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == DmaIntErr::NoErr
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DmaIntErr::Detected
    }
}
#[doc = "DMA Slave Error. Slave Error detected by primary AXI DataMover. This error occurs if the slave read from the Memory Map interface issues a Slave Error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaSlvErr {
    #[doc = "0: `0`"]
    NoErr = 0,
    #[doc = "1: `1`"]
    Detected = 1,
}
impl From<DmaSlvErr> for bool {
    #[inline(always)]
    fn from(variant: DmaSlvErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dma_slv_err` reader - DMA Slave Error. Slave Error detected by primary AXI DataMover. This error occurs if the slave read from the Memory Map interface issues a Slave Error."]
pub type DmaSlvErrR = crate::BitReader<DmaSlvErr>;
impl DmaSlvErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaSlvErr {
        match self.bits {
            false => DmaSlvErr::NoErr,
            true => DmaSlvErr::Detected,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == DmaSlvErr::NoErr
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DmaSlvErr::Detected
    }
}
#[doc = "DMA Decode Error. Decode Error detected by primary AXI DataMover. This error occurs if the Descriptor Buffer Address points to an invalid address.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDecErr {
    #[doc = "0: `0`"]
    NoErr = 0,
    #[doc = "1: `1`"]
    Detected = 1,
}
impl From<DmaDecErr> for bool {
    #[inline(always)]
    fn from(variant: DmaDecErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dma_dec_err` reader - DMA Decode Error. Decode Error detected by primary AXI DataMover. This error occurs if the Descriptor Buffer Address points to an invalid address."]
pub type DmaDecErrR = crate::BitReader<DmaDecErr>;
impl DmaDecErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaDecErr {
        match self.bits {
            false => DmaDecErr::NoErr,
            true => DmaDecErr::Detected,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == DmaDecErr::NoErr
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DmaDecErr::Detected
    }
}
#[doc = "Completed. This indicates to the software that the DMA Engine has completed the transfer as described by the associated descriptor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplt {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Cmplt> for bool {
    #[inline(always)]
    fn from(variant: Cmplt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cmplt` reader - Completed. This indicates to the software that the DMA Engine has completed the transfer as described by the associated descriptor."]
pub type CmpltR = crate::BitReader<Cmplt>;
impl CmpltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmplt {
        match self.bits {
            false => Cmplt::False,
            true => Cmplt::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Cmplt::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Cmplt::True
    }
}
impl R {
    #[doc = "Bits 0:25 - This value indicates the amount of data received and stored in the buffer described by this descriptor. This might or might not match the buffer length."]
    #[inline(always)]
    pub fn tfer_bytes(&self) -> TferBytesR {
        TferBytesR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 26 - End of Frame. Flag indicating buffer holds the last part of packet. This bit is set by AXI DMA to indicate to the sw/user that the buffer associated with this descriptor contains the end of the packet."]
    #[inline(always)]
    pub fn rxeof(&self) -> RxeofR {
        RxeofR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Start of Frame. Flag indicating buffer holds first part of packet. This bit is set by AXI DMA to indicate to the sw/user that the buffer associated with this descriptor contains the start of the packet."]
    #[inline(always)]
    pub fn rxsof(&self) -> RxsofR {
        RxsofR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Internal Error. Internal Error detected by primary AXI DataMover. This error can occur if a 0 length bytes to transfer is fed to the AXI DataMover."]
    #[inline(always)]
    pub fn dma_int_err(&self) -> DmaIntErrR {
        DmaIntErrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA Slave Error. Slave Error detected by primary AXI DataMover. This error occurs if the slave read from the Memory Map interface issues a Slave Error."]
    #[inline(always)]
    pub fn dma_slv_err(&self) -> DmaSlvErrR {
        DmaSlvErrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Decode Error. Decode Error detected by primary AXI DataMover. This error occurs if the Descriptor Buffer Address points to an invalid address."]
    #[inline(always)]
    pub fn dma_dec_err(&self) -> DmaDecErrR {
        DmaDecErrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Completed. This indicates to the software that the DMA Engine has completed the transfer as described by the associated descriptor."]
    #[inline(always)]
    pub fn cmplt(&self) -> CmpltR {
        CmpltR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status of BD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
