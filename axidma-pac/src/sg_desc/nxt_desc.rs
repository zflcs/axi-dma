#[doc = "Register `nxt_desc` reader"]
pub type R = crate::R<NxtDescSpec>;
#[doc = "Register `nxt_desc` writer"]
pub type W = crate::W<NxtDescSpec>;
#[doc = "Field `nxt_desc_ptr` reader - Indicates the lower order pointer pointing to the first word of the next descriptor"]
pub type NxtDescPtrR = crate::FieldReader<u32>;
#[doc = "Field `nxt_desc_ptr` writer - Indicates the lower order pointer pointing to the first word of the next descriptor"]
pub type NxtDescPtrW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 6:31 - Indicates the lower order pointer pointing to the first word of the next descriptor"]
    #[inline(always)]
    pub fn nxt_desc_ptr(&self) -> NxtDescPtrR {
        NxtDescPtrR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 6:31 - Indicates the lower order pointer pointing to the first word of the next descriptor"]
    #[inline(always)]
    #[must_use]
    pub fn nxt_desc_ptr(&mut self) -> NxtDescPtrW<NxtDescSpec> {
        NxtDescPtrW::new(self, 6)
    }
}
#[doc = "Next Descriptor Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nxt_desc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nxt_desc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NxtDescSpec;
impl crate::RegisterSpec for NxtDescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nxt_desc::R`](R) reader structure"]
impl crate::Readable for NxtDescSpec {}
#[doc = "`write(|w| ..)` method takes [`nxt_desc::W`](W) writer structure"]
impl crate::Writable for NxtDescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets nxt_desc to value 0"]
impl crate::Resettable for NxtDescSpec {
    const RESET_VALUE: u32 = 0;
}
