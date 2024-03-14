#[doc = "Register `mm2s_taildesc_msb` reader"]
pub type R = crate::R<Mm2sTaildescMsbSpec>;
#[doc = "Register `mm2s_taildesc_msb` writer"]
pub type W = crate::W<Mm2sTaildescMsbSpec>;
#[doc = "Field `taildesc_ptr` reader - Indicates the pause pointer in a descriptor chain."]
pub type TaildescPtrR = crate::FieldReader<u32>;
#[doc = "Field `taildesc_ptr` writer - Indicates the pause pointer in a descriptor chain."]
pub type TaildescPtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates the pause pointer in a descriptor chain."]
    #[inline(always)]
    pub fn taildesc_ptr(&self) -> TaildescPtrR {
        TaildescPtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates the pause pointer in a descriptor chain."]
    #[inline(always)]
    #[must_use]
    pub fn taildesc_ptr(&mut self) -> TaildescPtrW<Mm2sTaildescMsbSpec> {
        TaildescPtrW::new(self, 0)
    }
}
#[doc = "MM2S Tail Descriptor Pointer. Upper 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mm2s_taildesc_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mm2s_taildesc_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mm2sTaildescMsbSpec;
impl crate::RegisterSpec for Mm2sTaildescMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm2s_taildesc_msb::R`](R) reader structure"]
impl crate::Readable for Mm2sTaildescMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`mm2s_taildesc_msb::W`](W) writer structure"]
impl crate::Writable for Mm2sTaildescMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mm2s_taildesc_msb to value 0"]
impl crate::Resettable for Mm2sTaildescMsbSpec {
    const RESET_VALUE: u32 = 0;
}
