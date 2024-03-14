#[doc = "Register `curdesc` reader"]
pub type R = crate::R<CurdescSpec>;
#[doc = "Register `curdesc` writer"]
pub type W = crate::W<CurdescSpec>;
#[doc = "Field `curdesc_ptr` reader - Indicates the pointer of the current descriptor being worked on."]
pub type CurdescPtrR = crate::FieldReader<u32>;
#[doc = "Field `curdesc_ptr` writer - Indicates the pointer of the current descriptor being worked on."]
pub type CurdescPtrW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 6:31 - Indicates the pointer of the current descriptor being worked on."]
    #[inline(always)]
    pub fn curdesc_ptr(&self) -> CurdescPtrR {
        CurdescPtrR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 6:31 - Indicates the pointer of the current descriptor being worked on."]
    #[inline(always)]
    #[must_use]
    pub fn curdesc_ptr(&mut self) -> CurdescPtrW<CurdescSpec> {
        CurdescPtrW::new(self, 6)
    }
}
#[doc = "Current Descriptor Pointer. Lower 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curdesc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`curdesc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurdescSpec;
impl crate::RegisterSpec for CurdescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curdesc::R`](R) reader structure"]
impl crate::Readable for CurdescSpec {}
#[doc = "`write(|w| ..)` method takes [`curdesc::W`](W) writer structure"]
impl crate::Writable for CurdescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets curdesc to value 0"]
impl crate::Resettable for CurdescSpec {
    const RESET_VALUE: u32 = 0;
}
