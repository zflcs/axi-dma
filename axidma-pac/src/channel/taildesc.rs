#[doc = "Register `taildesc` reader"]
pub type R = crate::R<TaildescSpec>;
#[doc = "Register `taildesc` writer"]
pub type W = crate::W<TaildescSpec>;
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
    pub fn taildesc_ptr(&mut self) -> TaildescPtrW<TaildescSpec> {
        TaildescPtrW::new(self, 6)
    }
}
#[doc = "Tail Descriptor Pointer. Lower 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taildesc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taildesc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaildescSpec;
impl crate::RegisterSpec for TaildescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`taildesc::R`](R) reader structure"]
impl crate::Readable for TaildescSpec {}
#[doc = "`write(|w| ..)` method takes [`taildesc::W`](W) writer structure"]
impl crate::Writable for TaildescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets taildesc to value 0"]
impl crate::Resettable for TaildescSpec {
    const RESET_VALUE: u32 = 0;
}
