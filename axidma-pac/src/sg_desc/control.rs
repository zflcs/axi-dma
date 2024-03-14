#[doc = "Register `control` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `control` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `buf_len` reader - Indicates the amount of space in bytes of the stream."]
pub type BufLenR = crate::FieldReader<u32>;
#[doc = "Field `buf_len` writer - Indicates the amount of space in bytes of the stream."]
pub type BufLenW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "End of Frame. Flag indicating the last buffer to be processed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eof {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Eof> for bool {
    #[inline(always)]
    fn from(variant: Eof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `eof` reader - End of Frame. Flag indicating the last buffer to be processed."]
pub type EofR = crate::BitReader<Eof>;
impl EofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eof {
        match self.bits {
            false => Eof::False,
            true => Eof::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Eof::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Eof::True
    }
}
#[doc = "Field `eof` writer - End of Frame. Flag indicating the last buffer to be processed."]
pub type EofW<'a, REG> = crate::BitWriter<'a, REG, Eof>;
impl<'a, REG> EofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Eof::False)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Eof::True)
    }
}
#[doc = "Start of Frame. Flag indicating the first buffer to be processed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sof {
    #[doc = "0: `0`"]
    False = 0,
    #[doc = "1: `1`"]
    True = 1,
}
impl From<Sof> for bool {
    #[inline(always)]
    fn from(variant: Sof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sof` reader - Start of Frame. Flag indicating the first buffer to be processed."]
pub type SofR = crate::BitReader<Sof>;
impl SofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sof {
        match self.bits {
            false => Sof::False,
            true => Sof::True,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Sof::False
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Sof::True
    }
}
#[doc = "Field `sof` writer - Start of Frame. Flag indicating the first buffer to be processed."]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG, Sof>;
impl<'a, REG> SofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Sof::False)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Sof::True)
    }
}
impl R {
    #[doc = "Bits 0:25 - Indicates the amount of space in bytes of the stream."]
    #[inline(always)]
    pub fn buf_len(&self) -> BufLenR {
        BufLenR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 26 - End of Frame. Flag indicating the last buffer to be processed."]
    #[inline(always)]
    pub fn eof(&self) -> EofR {
        EofR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Start of Frame. Flag indicating the first buffer to be processed."]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - Indicates the amount of space in bytes of the stream."]
    #[inline(always)]
    #[must_use]
    pub fn buf_len(&mut self) -> BufLenW<ControlSpec> {
        BufLenW::new(self, 0)
    }
    #[doc = "Bit 26 - End of Frame. Flag indicating the last buffer to be processed."]
    #[inline(always)]
    #[must_use]
    pub fn eof(&mut self) -> EofW<ControlSpec> {
        EofW::new(self, 26)
    }
    #[doc = "Bit 27 - Start of Frame. Flag indicating the first buffer to be processed."]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SofW<ControlSpec> {
        SofW::new(self, 27)
    }
}
#[doc = "Control of BD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets control to value 0"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0;
}
