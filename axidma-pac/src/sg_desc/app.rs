#[doc = "Register `app[%s]` reader"]
pub type R = crate::R<AppSpec>;
#[doc = "Register `app[%s]` writer"]
pub type W = crate::W<AppSpec>;
#[doc = "Field `app` reader - Specifies user-specific application data."]
pub type AppR = crate::FieldReader<u32>;
#[doc = "Field `app` writer - Specifies user-specific application data."]
pub type AppW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies user-specific application data."]
    #[inline(always)]
    pub fn app(&self) -> AppR {
        AppR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies user-specific application data."]
    #[inline(always)]
    #[must_use]
    pub fn app(&mut self) -> AppW<AppSpec> {
        AppW::new(self, 0)
    }
}
#[doc = "User Application Field \\[%s\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppSpec;
impl crate::RegisterSpec for AppSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app::R`](R) reader structure"]
impl crate::Readable for AppSpec {}
#[doc = "`write(|w| ..)` method takes [`app::W`](W) writer structure"]
impl crate::Writable for AppSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets app[%s]
to value 0"]
impl crate::Resettable for AppSpec {
    const RESET_VALUE: u32 = 0;
}
