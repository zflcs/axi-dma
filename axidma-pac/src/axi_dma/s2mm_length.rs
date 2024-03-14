#[doc = "Register `s2mm_length` reader"]
pub type R = crate::R<S2mmLengthSpec>;
#[doc = "Register `s2mm_length` writer"]
pub type W = crate::W<S2mmLengthSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtlLtInitializeToTx {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<CtlLtInitializeToTx> for bool {
    #[inline(always)]
    fn from(variant: CtlLtInitializeToTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ctl_lt_initialize_to_tx` reader - "]
pub type CtlLtInitializeToTxR = crate::BitReader<CtlLtInitializeToTx>;
impl CtlLtInitializeToTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtlLtInitializeToTx {
        match self.bits {
            false => CtlLtInitializeToTx::Disable,
            true => CtlLtInitializeToTx::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CtlLtInitializeToTx::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CtlLtInitializeToTx::Enable
    }
}
#[doc = "Field `ctl_lt_initialize_to_tx` writer - "]
pub type CtlLtInitializeToTxW<'a, REG> = crate::BitWriter<'a, REG, CtlLtInitializeToTx>;
impl<'a, REG> CtlLtInitializeToTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CtlLtInitializeToTx::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CtlLtInitializeToTx::Enable)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctl_lt_initialize_to_tx(&self) -> CtlLtInitializeToTxR {
        CtlLtInitializeToTxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_lt_initialize_to_tx(&mut self) -> CtlLtInitializeToTxW<S2mmLengthSpec> {
        CtlLtInitializeToTxW::new(self, 0)
    }
}
#[doc = "S2MM Buffer Length (Bytes)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2mm_length::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2mm_length::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2mmLengthSpec;
impl crate::RegisterSpec for S2mmLengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2mm_length::R`](R) reader structure"]
impl crate::Readable for S2mmLengthSpec {}
#[doc = "`write(|w| ..)` method takes [`s2mm_length::W`](W) writer structure"]
impl crate::Writable for S2mmLengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets s2mm_length to value 0"]
impl crate::Resettable for S2mmLengthSpec {
    const RESET_VALUE: u32 = 0;
}
