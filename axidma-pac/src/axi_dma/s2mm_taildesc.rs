#[doc = "Register `s2mm_taildesc` reader"]
pub type R = crate::R<S2mmTaildescSpec>;
#[doc = "Register `s2mm_taildesc` writer"]
pub type W = crate::W<S2mmTaildescSpec>;
#[doc = "Field `taildesc_ptr` reader - Indicates the pause pointer in a descriptor chain."]
pub type TaildescPtrR = crate::FieldReader<u32>;
#[doc = "Field `taildesc_ptr` writer - Indicates the pause pointer in a descriptor chain."]
pub type TaildescPtrW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 6:31 - Indicates the pause pointer in a descriptor chain."]
    #[inline(always)]
    pub fn taildesc_ptr(&self) -> TaildescPtrR {
        TaildescPtrR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 6:31 - Indicates the pause pointer in a descriptor chain."]
    #[inline(always)]
    #[must_use]
    pub fn taildesc_ptr(&mut self) -> TaildescPtrW<S2mmTaildescSpec> {
        TaildescPtrW::new(self, 6)
    }
}
#[doc = "S2MM Tail Descriptor Pointer. Lower 32 address bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2mm_taildesc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2mm_taildesc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2mmTaildescSpec;
impl crate::RegisterSpec for S2mmTaildescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2mm_taildesc::R`](R) reader structure"]
impl crate::Readable for S2mmTaildescSpec {}
#[doc = "`write(|w| ..)` method takes [`s2mm_taildesc::W`](W) writer structure"]
impl crate::Writable for S2mmTaildescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets s2mm_taildesc to value 0"]
impl crate::Resettable for S2mmTaildescSpec {
    const RESET_VALUE: u32 = 0;
}
