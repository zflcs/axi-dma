#[doc = "Register `s2mm_dmacr` reader"]
pub type R = crate::R<S2mmDmacrSpec>;
#[doc = "Register `s2mm_dmacr` writer"]
pub type W = crate::W<S2mmDmacrSpec>;
#[doc = "Run / Stop control for controlling running and stopping of the DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunStop {
    #[doc = "0: `0`"]
    Stop = 0,
    #[doc = "1: `1`"]
    Run = 1,
}
impl From<RunStop> for bool {
    #[inline(always)]
    fn from(variant: RunStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `run_stop` reader - Run / Stop control for controlling running and stopping of the DMA channel."]
pub type RunStopR = crate::BitReader<RunStop>;
impl RunStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RunStop {
        match self.bits {
            false => RunStop::Stop,
            true => RunStop::Run,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RunStop::Stop
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == RunStop::Run
    }
}
#[doc = "Field `run_stop` writer - Run / Stop control for controlling running and stopping of the DMA channel."]
pub type RunStopW<'a, REG> = crate::BitWriter<'a, REG, RunStop>;
impl<'a, REG> RunStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(RunStop::Stop)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(RunStop::Run)
    }
}
#[doc = "Soft reset for resetting the AXI DMA core\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "0: `0`"]
    Normal = 0,
    #[doc = "1: `1`"]
    Reset = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reset` reader - Soft reset for resetting the AXI DMA core"]
pub type ResetR = crate::BitReader<Reset>;
impl ResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reset {
        match self.bits {
            false => Reset::Normal,
            true => Reset::Reset,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Reset::Normal
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Reset::Reset
    }
}
#[doc = "Field `reset` writer - Soft reset for resetting the AXI DMA core"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Normal)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Reset)
    }
}
#[doc = "Keyhole Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyhole {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<Keyhole> for bool {
    #[inline(always)]
    fn from(variant: Keyhole) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `keyhole` reader - Keyhole Read"]
pub type KeyholeR = crate::BitReader<Keyhole>;
impl KeyholeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Keyhole {
        match self.bits {
            false => Keyhole::Disable,
            true => Keyhole::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Keyhole::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Keyhole::Enable
    }
}
#[doc = "Field `keyhole` writer - Keyhole Read"]
pub type KeyholeW<'a, REG> = crate::BitWriter<'a, REG, Keyhole>;
impl<'a, REG> KeyholeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Keyhole::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Keyhole::Enable)
    }
}
#[doc = "When set to 1, the DMA operates in Cyclic Buffer Descriptor (BD) mode without any user intervention\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CyclicBufferDescriptor {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<CyclicBufferDescriptor> for bool {
    #[inline(always)]
    fn from(variant: CyclicBufferDescriptor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cyclic_buffer_descriptor` reader - When set to 1, the DMA operates in Cyclic Buffer Descriptor (BD) mode without any user intervention"]
pub type CyclicBufferDescriptorR = crate::BitReader<CyclicBufferDescriptor>;
impl CyclicBufferDescriptorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CyclicBufferDescriptor {
        match self.bits {
            false => CyclicBufferDescriptor::Disable,
            true => CyclicBufferDescriptor::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CyclicBufferDescriptor::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CyclicBufferDescriptor::Enable
    }
}
#[doc = "Field `cyclic_buffer_descriptor` writer - When set to 1, the DMA operates in Cyclic Buffer Descriptor (BD) mode without any user intervention"]
pub type CyclicBufferDescriptorW<'a, REG> = crate::BitWriter<'a, REG, CyclicBufferDescriptor>;
impl<'a, REG> CyclicBufferDescriptorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CyclicBufferDescriptor::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CyclicBufferDescriptor::Enable)
    }
}
#[doc = "Interrupt on Complete (IOC) Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IocIrqEn {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<IocIrqEn> for bool {
    #[inline(always)]
    fn from(variant: IocIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ioc_irq_en` reader - Interrupt on Complete (IOC) Interrupt Enable"]
pub type IocIrqEnR = crate::BitReader<IocIrqEn>;
impl IocIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IocIrqEn {
        match self.bits {
            false => IocIrqEn::Disable,
            true => IocIrqEn::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IocIrqEn::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IocIrqEn::Enable
    }
}
#[doc = "Field `ioc_irq_en` writer - Interrupt on Complete (IOC) Interrupt Enable"]
pub type IocIrqEnW<'a, REG> = crate::BitWriter<'a, REG, IocIrqEn>;
impl<'a, REG> IocIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IocIrqEn::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IocIrqEn::Enable)
    }
}
#[doc = "Interrupt on Delay Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DlyIrqEn {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<DlyIrqEn> for bool {
    #[inline(always)]
    fn from(variant: DlyIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dly_irq_en` reader - Interrupt on Delay Timer Interrupt Enable"]
pub type DlyIrqEnR = crate::BitReader<DlyIrqEn>;
impl DlyIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DlyIrqEn {
        match self.bits {
            false => DlyIrqEn::Disable,
            true => DlyIrqEn::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DlyIrqEn::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DlyIrqEn::Enable
    }
}
#[doc = "Field `dly_irq_en` writer - Interrupt on Delay Timer Interrupt Enable"]
pub type DlyIrqEnW<'a, REG> = crate::BitWriter<'a, REG, DlyIrqEn>;
impl<'a, REG> DlyIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DlyIrqEn::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DlyIrqEn::Enable)
    }
}
#[doc = "Interrupt on Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrIrqEn {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<ErrIrqEn> for bool {
    #[inline(always)]
    fn from(variant: ErrIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `err_irq_en` reader - Interrupt on Error Interrupt Enable"]
pub type ErrIrqEnR = crate::BitReader<ErrIrqEn>;
impl ErrIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErrIrqEn {
        match self.bits {
            false => ErrIrqEn::Disable,
            true => ErrIrqEn::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ErrIrqEn::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ErrIrqEn::Enable
    }
}
#[doc = "Field `err_irq_en` writer - Interrupt on Error Interrupt Enable"]
pub type ErrIrqEnW<'a, REG> = crate::BitWriter<'a, REG, ErrIrqEn>;
impl<'a, REG> ErrIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ErrIrqEn::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ErrIrqEn::Enable)
    }
}
#[doc = "Field `irq_threshold` reader - Interrupt Threshold"]
pub type IrqThresholdR = crate::FieldReader;
#[doc = "Field `irq_threshold` writer - Interrupt Threshold"]
pub type IrqThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `irq_delay` reader - Interrupt Delay Time Out"]
pub type IrqDelayR = crate::FieldReader;
#[doc = "Field `irq_delay` writer - Interrupt Delay Time Out"]
pub type IrqDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Run / Stop control for controlling running and stopping of the DMA channel."]
    #[inline(always)]
    pub fn run_stop(&self) -> RunStopR {
        RunStopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Soft reset for resetting the AXI DMA core"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Keyhole Read"]
    #[inline(always)]
    pub fn keyhole(&self) -> KeyholeR {
        KeyholeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set to 1, the DMA operates in Cyclic Buffer Descriptor (BD) mode without any user intervention"]
    #[inline(always)]
    pub fn cyclic_buffer_descriptor(&self) -> CyclicBufferDescriptorR {
        CyclicBufferDescriptorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt on Complete (IOC) Interrupt Enable"]
    #[inline(always)]
    pub fn ioc_irq_en(&self) -> IocIrqEnR {
        IocIrqEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt on Delay Timer Interrupt Enable"]
    #[inline(always)]
    pub fn dly_irq_en(&self) -> DlyIrqEnR {
        DlyIrqEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt on Error Interrupt Enable"]
    #[inline(always)]
    pub fn err_irq_en(&self) -> ErrIrqEnR {
        ErrIrqEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold"]
    #[inline(always)]
    pub fn irq_threshold(&self) -> IrqThresholdR {
        IrqThresholdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt Delay Time Out"]
    #[inline(always)]
    pub fn irq_delay(&self) -> IrqDelayR {
        IrqDelayR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Run / Stop control for controlling running and stopping of the DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn run_stop(&mut self) -> RunStopW<S2mmDmacrSpec> {
        RunStopW::new(self, 0)
    }
    #[doc = "Bit 2 - Soft reset for resetting the AXI DMA core"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<S2mmDmacrSpec> {
        ResetW::new(self, 2)
    }
    #[doc = "Bit 3 - Keyhole Read"]
    #[inline(always)]
    #[must_use]
    pub fn keyhole(&mut self) -> KeyholeW<S2mmDmacrSpec> {
        KeyholeW::new(self, 3)
    }
    #[doc = "Bit 4 - When set to 1, the DMA operates in Cyclic Buffer Descriptor (BD) mode without any user intervention"]
    #[inline(always)]
    #[must_use]
    pub fn cyclic_buffer_descriptor(&mut self) -> CyclicBufferDescriptorW<S2mmDmacrSpec> {
        CyclicBufferDescriptorW::new(self, 4)
    }
    #[doc = "Bit 12 - Interrupt on Complete (IOC) Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ioc_irq_en(&mut self) -> IocIrqEnW<S2mmDmacrSpec> {
        IocIrqEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt on Delay Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dly_irq_en(&mut self) -> DlyIrqEnW<S2mmDmacrSpec> {
        DlyIrqEnW::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt on Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_irq_en(&mut self) -> ErrIrqEnW<S2mmDmacrSpec> {
        ErrIrqEnW::new(self, 14)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn irq_threshold(&mut self) -> IrqThresholdW<S2mmDmacrSpec> {
        IrqThresholdW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt Delay Time Out"]
    #[inline(always)]
    #[must_use]
    pub fn irq_delay(&mut self) -> IrqDelayW<S2mmDmacrSpec> {
        IrqDelayW::new(self, 24)
    }
}
#[doc = "S2MM DMA Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2mm_dmacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2mm_dmacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2mmDmacrSpec;
impl crate::RegisterSpec for S2mmDmacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2mm_dmacr::R`](R) reader structure"]
impl crate::Readable for S2mmDmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`s2mm_dmacr::W`](W) writer structure"]
impl crate::Writable for S2mmDmacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets s2mm_dmacr to value 0"]
impl crate::Resettable for S2mmDmacrSpec {
    const RESET_VALUE: u32 = 0;
}
