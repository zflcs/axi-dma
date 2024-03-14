#[doc = "Register `mm2s_curdesc_msb` reader"]
pub type R = crate::R<Mm2sCurdescMsbSpec>;
#[doc = "Register `mm2s_curdesc_msb` writer"]
pub type W = crate::W<Mm2sCurdescMsbSpec>;
#[doc = "Field `curdesc_ptr` reader - Indicates the pointer of the current descriptor being worked on."]
pub type CurdescPtrR = crate::FieldReader<u32>;
#[doc = "Field `curdesc_ptr` writer - Indicates the pointer of the current descriptor being worked on."]
pub type CurdescPtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates the pointer of the current descriptor being worked on."]
    #[inline(always)]
    pub fn curdesc_ptr(&self) -> CurdescPtrR {
        CurdescPtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates the pointer of the current descriptor being worked on."]
    #[inline(always)]
    #[must_use]
    pub fn curdesc_ptr(&mut self) -> CurdescPtrW<Mm2sCurdescMsbSpec> {
        CurdescPtrW::new(self, 0)
    }
}
#[doc = "MM2S Current Descriptor Pointer. Upper 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mm2s_curdesc_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mm2s_curdesc_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mm2sCurdescMsbSpec;
impl crate::RegisterSpec for Mm2sCurdescMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm2s_curdesc_msb::R`](R) reader structure"]
impl crate::Readable for Mm2sCurdescMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`mm2s_curdesc_msb::W`](W) writer structure"]
impl crate::Writable for Mm2sCurdescMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mm2s_curdesc_msb to value 0"]
impl crate::Resettable for Mm2sCurdescMsbSpec {
    const RESET_VALUE: u32 = 0;
}
