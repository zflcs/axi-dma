#[doc = "Register `nxt_desc_msb` reader"]
pub type R = crate::R<NxtDescMsbSpec>;
#[doc = "Register `nxt_desc_msb` writer"]
pub type W = crate::W<NxtDescMsbSpec>;
#[doc = "Field `nxt_desc_ptr` reader - Indicates the MSB 32 bits of the pointer pointing to the first word of the next descriptor"]
pub type NxtDescPtrR = crate::FieldReader<u32>;
#[doc = "Field `nxt_desc_ptr` writer - Indicates the MSB 32 bits of the pointer pointing to the first word of the next descriptor"]
pub type NxtDescPtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates the MSB 32 bits of the pointer pointing to the first word of the next descriptor"]
    #[inline(always)]
    pub fn nxt_desc_ptr(&self) -> NxtDescPtrR {
        NxtDescPtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates the MSB 32 bits of the pointer pointing to the first word of the next descriptor"]
    #[inline(always)]
    #[must_use]
    pub fn nxt_desc_ptr(&mut self) -> NxtDescPtrW<NxtDescMsbSpec> {
        NxtDescPtrW::new(self, 0)
    }
}
#[doc = "Upper 32 bits of Next Descriptor Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nxt_desc_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nxt_desc_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NxtDescMsbSpec;
impl crate::RegisterSpec for NxtDescMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nxt_desc_msb::R`](R) reader structure"]
impl crate::Readable for NxtDescMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`nxt_desc_msb::W`](W) writer structure"]
impl crate::Writable for NxtDescMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets nxt_desc_msb to value 0"]
impl crate::Resettable for NxtDescMsbSpec {
    const RESET_VALUE: u32 = 0;
}
