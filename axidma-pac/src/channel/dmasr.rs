#[doc = "Register `dmasr` reader"]
pub type R = crate::R<DmasrSpec>;
#[doc = "Register `dmasr` writer"]
pub type W = crate::W<DmasrSpec>;
#[doc = "DMA Channel Halted. Indicates the run/stop state of the DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Halted {
    #[doc = "0: `0`"]
    Running = 0,
    #[doc = "1: `1`"]
    Halted = 1,
}
impl From<Halted> for bool {
    #[inline(always)]
    fn from(variant: Halted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `halted` reader - DMA Channel Halted. Indicates the run/stop state of the DMA channel."]
pub type HaltedR = crate::BitReader<Halted>;
impl HaltedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Halted {
        match self.bits {
            false => Halted::Running,
            true => Halted::Halted,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == Halted::Running
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_halted(&self) -> bool {
        *self == Halted::Halted
    }
}
#[doc = "Field `halted` writer - DMA Channel Halted. Indicates the run/stop state of the DMA channel."]
pub type HaltedW<'a, REG> = crate::BitWriter<'a, REG, Halted>;
impl<'a, REG> HaltedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn running(self) -> &'a mut crate::W<REG> {
        self.variant(Halted::Running)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn halted(self) -> &'a mut crate::W<REG> {
        self.variant(Halted::Halted)
    }
}
#[doc = "DMA Channel Idle. Indicates the state of AXI DMA operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: `0`"]
    NotIdle = 0,
    #[doc = "1: `1`"]
    Idle = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `idle` reader - DMA Channel Idle. Indicates the state of AXI DMA operations."]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            false => Idle::NotIdle,
            true => Idle::Idle,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_idle(&self) -> bool {
        *self == Idle::NotIdle
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Idle::Idle
    }
}
#[doc = "Field `idle` writer - DMA Channel Idle. Indicates the state of AXI DMA operations."]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG, Idle>;
impl<'a, REG> IdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_idle(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::NotIdle)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::Idle)
    }
}
#[doc = "Scatter Gather Engine Included. DMASR.SGIncld = 1 indicates the Scatter Gather engine is included and the AXI DMA is configured for Scatter Gather mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SgIncld {
    #[doc = "0: `0`"]
    SgDisabled = 0,
    #[doc = "1: `1`"]
    SgEnabled = 1,
}
impl From<SgIncld> for bool {
    #[inline(always)]
    fn from(variant: SgIncld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sg_incld` reader - Scatter Gather Engine Included. DMASR.SGIncld = 1 indicates the Scatter Gather engine is included and the AXI DMA is configured for Scatter Gather mode."]
pub type SgIncldR = crate::BitReader<SgIncld>;
impl SgIncldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SgIncld {
        match self.bits {
            false => SgIncld::SgDisabled,
            true => SgIncld::SgEnabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_sg_disabled(&self) -> bool {
        *self == SgIncld::SgDisabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sg_enabled(&self) -> bool {
        *self == SgIncld::SgEnabled
    }
}
#[doc = "Field `sg_incld` writer - Scatter Gather Engine Included. DMASR.SGIncld = 1 indicates the Scatter Gather engine is included and the AXI DMA is configured for Scatter Gather mode."]
pub type SgIncldW<'a, REG> = crate::BitWriter<'a, REG, SgIncld>;
impl<'a, REG> SgIncldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sg_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SgIncld::SgDisabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sg_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SgIncld::SgEnabled)
    }
}
#[doc = "DMA Internal Error. Internal error occurs if the buffer length specified in the fetched descriptor is set to 0.\n\nValue on reset: 0"]
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
#[doc = "Field `dma_int_err` reader - DMA Internal Error. Internal error occurs if the buffer length specified in the fetched descriptor is set to 0."]
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
#[doc = "Field `dma_int_err` writer - DMA Internal Error. Internal error occurs if the buffer length specified in the fetched descriptor is set to 0."]
pub type DmaIntErrW<'a, REG> = crate::BitWriter<'a, REG, DmaIntErr>;
impl<'a, REG> DmaIntErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut crate::W<REG> {
        self.variant(DmaIntErr::NoErr)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(DmaIntErr::Detected)
    }
}
#[doc = "DMA Slave Error. This error occurs if the slave read from the Memory Map interface issues a Slave Error.\n\nValue on reset: 0"]
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
#[doc = "Field `dma_slv_err` reader - DMA Slave Error. This error occurs if the slave read from the Memory Map interface issues a Slave Error."]
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
#[doc = "Field `dma_slv_err` writer - DMA Slave Error. This error occurs if the slave read from the Memory Map interface issues a Slave Error."]
pub type DmaSlvErrW<'a, REG> = crate::BitWriter<'a, REG, DmaSlvErr>;
impl<'a, REG> DmaSlvErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut crate::W<REG> {
        self.variant(DmaSlvErr::NoErr)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(DmaSlvErr::Detected)
    }
}
#[doc = "DMA Decode Error. This error occurs if the address request points to an invalid address.\n\nValue on reset: 0"]
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
#[doc = "Field `dma_dec_err` reader - DMA Decode Error. This error occurs if the address request points to an invalid address."]
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
#[doc = "Field `dma_dec_err` writer - DMA Decode Error. This error occurs if the address request points to an invalid address."]
pub type DmaDecErrW<'a, REG> = crate::BitWriter<'a, REG, DmaDecErr>;
impl<'a, REG> DmaDecErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDecErr::NoErr)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDecErr::Detected)
    }
}
#[doc = "Scatter Gather Internal Error. This error occurs if a descriptor with the “Complete bit” already set is fetched.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SgIntErr {
    #[doc = "0: `0`"]
    NoErr = 0,
    #[doc = "1: `1`"]
    Detected = 1,
}
impl From<SgIntErr> for bool {
    #[inline(always)]
    fn from(variant: SgIntErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sg_int_err` reader - Scatter Gather Internal Error. This error occurs if a descriptor with the “Complete bit” already set is fetched."]
pub type SgIntErrR = crate::BitReader<SgIntErr>;
impl SgIntErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SgIntErr {
        match self.bits {
            false => SgIntErr::NoErr,
            true => SgIntErr::Detected,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == SgIntErr::NoErr
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == SgIntErr::Detected
    }
}
#[doc = "Field `sg_int_err` writer - Scatter Gather Internal Error. This error occurs if a descriptor with the “Complete bit” already set is fetched."]
pub type SgIntErrW<'a, REG> = crate::BitWriter<'a, REG, SgIntErr>;
impl<'a, REG> SgIntErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut crate::W<REG> {
        self.variant(SgIntErr::NoErr)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(SgIntErr::Detected)
    }
}
#[doc = "Scatter Gather Slave Error. This error occurs if the slave read from on the Memory Map interface issues a Slave error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SgSlvErr {
    #[doc = "0: `0`"]
    NoErr = 0,
    #[doc = "1: `1`"]
    Detected = 1,
}
impl From<SgSlvErr> for bool {
    #[inline(always)]
    fn from(variant: SgSlvErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sg_slv_err` reader - Scatter Gather Slave Error. This error occurs if the slave read from on the Memory Map interface issues a Slave error."]
pub type SgSlvErrR = crate::BitReader<SgSlvErr>;
impl SgSlvErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SgSlvErr {
        match self.bits {
            false => SgSlvErr::NoErr,
            true => SgSlvErr::Detected,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == SgSlvErr::NoErr
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == SgSlvErr::Detected
    }
}
#[doc = "Field `sg_slv_err` writer - Scatter Gather Slave Error. This error occurs if the slave read from on the Memory Map interface issues a Slave error."]
pub type SgSlvErrW<'a, REG> = crate::BitWriter<'a, REG, SgSlvErr>;
impl<'a, REG> SgSlvErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut crate::W<REG> {
        self.variant(SgSlvErr::NoErr)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(SgSlvErr::Detected)
    }
}
#[doc = "Scatter Gather Decode Error. This error occurs if CURDESC_PTR and/or NXTDESC_PTR points to an invalid address.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SgDecErr {
    #[doc = "0: `0`"]
    NoErr = 0,
    #[doc = "1: `1`"]
    Detected = 1,
}
impl From<SgDecErr> for bool {
    #[inline(always)]
    fn from(variant: SgDecErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sg_dec_err` reader - Scatter Gather Decode Error. This error occurs if CURDESC_PTR and/or NXTDESC_PTR points to an invalid address."]
pub type SgDecErrR = crate::BitReader<SgDecErr>;
impl SgDecErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SgDecErr {
        match self.bits {
            false => SgDecErr::NoErr,
            true => SgDecErr::Detected,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == SgDecErr::NoErr
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == SgDecErr::Detected
    }
}
#[doc = "Field `sg_dec_err` writer - Scatter Gather Decode Error. This error occurs if CURDESC_PTR and/or NXTDESC_PTR points to an invalid address."]
pub type SgDecErrW<'a, REG> = crate::BitWriter<'a, REG, SgDecErr>;
impl<'a, REG> SgDecErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut crate::W<REG> {
        self.variant(SgDecErr::NoErr)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(SgDecErr::Detected)
    }
}
#[doc = "Interrupt on Complete (IOC)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IocIrq {
    #[doc = "0: `0`"]
    NoIntr = 0,
    #[doc = "1: `1`"]
    Detected = 1,
}
impl From<IocIrq> for bool {
    #[inline(always)]
    fn from(variant: IocIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ioc_irq` reader - Interrupt on Complete (IOC)"]
pub type IocIrqR = crate::BitReader<IocIrq>;
impl IocIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IocIrq {
        match self.bits {
            false => IocIrq::NoIntr,
            true => IocIrq::Detected,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == IocIrq::NoIntr
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == IocIrq::Detected
    }
}
#[doc = "Field `ioc_irq` writer - Interrupt on Complete (IOC)"]
pub type IocIrqW<'a, REG> = crate::BitWriter<'a, REG, IocIrq>;
impl<'a, REG> IocIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_intr(self) -> &'a mut crate::W<REG> {
        self.variant(IocIrq::NoIntr)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(IocIrq::Detected)
    }
}
#[doc = "Interrupt on Delay Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DlyIrq {
    #[doc = "0: `0`"]
    NoIntr = 0,
    #[doc = "1: `1`"]
    Detected = 1,
}
impl From<DlyIrq> for bool {
    #[inline(always)]
    fn from(variant: DlyIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dly_irq` reader - Interrupt on Delay Timer"]
pub type DlyIrqR = crate::BitReader<DlyIrq>;
impl DlyIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DlyIrq {
        match self.bits {
            false => DlyIrq::NoIntr,
            true => DlyIrq::Detected,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == DlyIrq::NoIntr
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DlyIrq::Detected
    }
}
#[doc = "Field `dly_irq` writer - Interrupt on Delay Timer"]
pub type DlyIrqW<'a, REG> = crate::BitWriter<'a, REG, DlyIrq>;
impl<'a, REG> DlyIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_intr(self) -> &'a mut crate::W<REG> {
        self.variant(DlyIrq::NoIntr)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(DlyIrq::Detected)
    }
}
#[doc = "Interrupt on Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrIrq {
    #[doc = "0: `0`"]
    NoIntr = 0,
    #[doc = "1: `1`"]
    Detected = 1,
}
impl From<ErrIrq> for bool {
    #[inline(always)]
    fn from(variant: ErrIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `err_irq` reader - Interrupt on Error"]
pub type ErrIrqR = crate::BitReader<ErrIrq>;
impl ErrIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErrIrq {
        match self.bits {
            false => ErrIrq::NoIntr,
            true => ErrIrq::Detected,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == ErrIrq::NoIntr
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == ErrIrq::Detected
    }
}
#[doc = "Field `err_irq` writer - Interrupt on Error"]
pub type ErrIrqW<'a, REG> = crate::BitWriter<'a, REG, ErrIrq>;
impl<'a, REG> ErrIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_intr(self) -> &'a mut crate::W<REG> {
        self.variant(ErrIrq::NoIntr)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(ErrIrq::Detected)
    }
}
#[doc = "Field `irq_threshold_sts` reader - Interrupt Threshold Status. Indicates current interrupt threshold value."]
pub type IrqThresholdStsR = crate::FieldReader;
#[doc = "Field `irq_threshold_sts` writer - Interrupt Threshold Status. Indicates current interrupt threshold value."]
pub type IrqThresholdStsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `irq_delay_sts` reader - Interrupt Delay Time Status. Indicates current interrupt delay time value."]
pub type IrqDelayStsR = crate::FieldReader;
#[doc = "Field `irq_delay_sts` writer - Interrupt Delay Time Status. Indicates current interrupt delay time value."]
pub type IrqDelayStsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - DMA Channel Halted. Indicates the run/stop state of the DMA channel."]
    #[inline(always)]
    pub fn halted(&self) -> HaltedR {
        HaltedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel Idle. Indicates the state of AXI DMA operations."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Scatter Gather Engine Included. DMASR.SGIncld = 1 indicates the Scatter Gather engine is included and the AXI DMA is configured for Scatter Gather mode."]
    #[inline(always)]
    pub fn sg_incld(&self) -> SgIncldR {
        SgIncldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Internal Error. Internal error occurs if the buffer length specified in the fetched descriptor is set to 0."]
    #[inline(always)]
    pub fn dma_int_err(&self) -> DmaIntErrR {
        DmaIntErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Slave Error. This error occurs if the slave read from the Memory Map interface issues a Slave Error."]
    #[inline(always)]
    pub fn dma_slv_err(&self) -> DmaSlvErrR {
        DmaSlvErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Decode Error. This error occurs if the address request points to an invalid address."]
    #[inline(always)]
    pub fn dma_dec_err(&self) -> DmaDecErrR {
        DmaDecErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Scatter Gather Internal Error. This error occurs if a descriptor with the “Complete bit” already set is fetched."]
    #[inline(always)]
    pub fn sg_int_err(&self) -> SgIntErrR {
        SgIntErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scatter Gather Slave Error. This error occurs if the slave read from on the Memory Map interface issues a Slave error."]
    #[inline(always)]
    pub fn sg_slv_err(&self) -> SgSlvErrR {
        SgSlvErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Scatter Gather Decode Error. This error occurs if CURDESC_PTR and/or NXTDESC_PTR points to an invalid address."]
    #[inline(always)]
    pub fn sg_dec_err(&self) -> SgDecErrR {
        SgDecErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt on Complete (IOC)"]
    #[inline(always)]
    pub fn ioc_irq(&self) -> IocIrqR {
        IocIrqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt on Delay Timer"]
    #[inline(always)]
    pub fn dly_irq(&self) -> DlyIrqR {
        DlyIrqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt on Error"]
    #[inline(always)]
    pub fn err_irq(&self) -> ErrIrqR {
        ErrIrqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Status. Indicates current interrupt threshold value."]
    #[inline(always)]
    pub fn irq_threshold_sts(&self) -> IrqThresholdStsR {
        IrqThresholdStsR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt Delay Time Status. Indicates current interrupt delay time value."]
    #[inline(always)]
    pub fn irq_delay_sts(&self) -> IrqDelayStsR {
        IrqDelayStsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel Halted. Indicates the run/stop state of the DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn halted(&mut self) -> HaltedW<DmasrSpec> {
        HaltedW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Channel Idle. Indicates the state of AXI DMA operations."]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<DmasrSpec> {
        IdleW::new(self, 1)
    }
    #[doc = "Bit 3 - Scatter Gather Engine Included. DMASR.SGIncld = 1 indicates the Scatter Gather engine is included and the AXI DMA is configured for Scatter Gather mode."]
    #[inline(always)]
    #[must_use]
    pub fn sg_incld(&mut self) -> SgIncldW<DmasrSpec> {
        SgIncldW::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Internal Error. Internal error occurs if the buffer length specified in the fetched descriptor is set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn dma_int_err(&mut self) -> DmaIntErrW<DmasrSpec> {
        DmaIntErrW::new(self, 4)
    }
    #[doc = "Bit 5 - DMA Slave Error. This error occurs if the slave read from the Memory Map interface issues a Slave Error."]
    #[inline(always)]
    #[must_use]
    pub fn dma_slv_err(&mut self) -> DmaSlvErrW<DmasrSpec> {
        DmaSlvErrW::new(self, 5)
    }
    #[doc = "Bit 6 - DMA Decode Error. This error occurs if the address request points to an invalid address."]
    #[inline(always)]
    #[must_use]
    pub fn dma_dec_err(&mut self) -> DmaDecErrW<DmasrSpec> {
        DmaDecErrW::new(self, 6)
    }
    #[doc = "Bit 8 - Scatter Gather Internal Error. This error occurs if a descriptor with the “Complete bit” already set is fetched."]
    #[inline(always)]
    #[must_use]
    pub fn sg_int_err(&mut self) -> SgIntErrW<DmasrSpec> {
        SgIntErrW::new(self, 8)
    }
    #[doc = "Bit 9 - Scatter Gather Slave Error. This error occurs if the slave read from on the Memory Map interface issues a Slave error."]
    #[inline(always)]
    #[must_use]
    pub fn sg_slv_err(&mut self) -> SgSlvErrW<DmasrSpec> {
        SgSlvErrW::new(self, 9)
    }
    #[doc = "Bit 10 - Scatter Gather Decode Error. This error occurs if CURDESC_PTR and/or NXTDESC_PTR points to an invalid address."]
    #[inline(always)]
    #[must_use]
    pub fn sg_dec_err(&mut self) -> SgDecErrW<DmasrSpec> {
        SgDecErrW::new(self, 10)
    }
    #[doc = "Bit 12 - Interrupt on Complete (IOC)"]
    #[inline(always)]
    #[must_use]
    pub fn ioc_irq(&mut self) -> IocIrqW<DmasrSpec> {
        IocIrqW::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt on Delay Timer"]
    #[inline(always)]
    #[must_use]
    pub fn dly_irq(&mut self) -> DlyIrqW<DmasrSpec> {
        DlyIrqW::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt on Error"]
    #[inline(always)]
    #[must_use]
    pub fn err_irq(&mut self) -> ErrIrqW<DmasrSpec> {
        ErrIrqW::new(self, 14)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Status. Indicates current interrupt threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn irq_threshold_sts(&mut self) -> IrqThresholdStsW<DmasrSpec> {
        IrqThresholdStsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt Delay Time Status. Indicates current interrupt delay time value."]
    #[inline(always)]
    #[must_use]
    pub fn irq_delay_sts(&mut self) -> IrqDelayStsW<DmasrSpec> {
        IrqDelayStsW::new(self, 24)
    }
}
#[doc = "DMA Channel Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmasrSpec;
impl crate::RegisterSpec for DmasrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasr::R`](R) reader structure"]
impl crate::Readable for DmasrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmasr::W`](W) writer structure"]
impl crate::Writable for DmasrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dmasr to value 0"]
impl crate::Resettable for DmasrSpec {
    const RESET_VALUE: u32 = 0;
}
