#[doc = "Register `mm2s_length` reader"]
pub type R = crate::R<Mm2sLengthSpec>;
#[doc = "Register `mm2s_length` writer"]
pub type W = crate::W<Mm2sLengthSpec>;
#[doc = "Field `length` reader - Indicates the number of bytes to transfer for the MM2S channel."]
pub type LengthR = crate::FieldReader<u32>;
#[doc = "Field `length` writer - Indicates the number of bytes to transfer for the MM2S channel."]
pub type LengthW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - Indicates the number of bytes to transfer for the MM2S channel."]
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Indicates the number of bytes to transfer for the MM2S channel."]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LengthW<Mm2sLengthSpec> {
        LengthW::new(self, 0)
    }
}
#[doc = "MM2S Transfer Length (Bytes)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mm2s_length::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mm2s_length::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mm2sLengthSpec;
impl crate::RegisterSpec for Mm2sLengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm2s_length::R`](R) reader structure"]
impl crate::Readable for Mm2sLengthSpec {}
#[doc = "`write(|w| ..)` method takes [`mm2s_length::W`](W) writer structure"]
impl crate::Writable for Mm2sLengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mm2s_length to value 0"]
impl crate::Resettable for Mm2sLengthSpec {
    const RESET_VALUE: u32 = 0;
}
