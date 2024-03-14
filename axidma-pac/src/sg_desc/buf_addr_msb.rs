#[doc = "Register `buf_addr_msb` reader"]
pub type R = crate::R<BufAddrMsbSpec>;
#[doc = "Register `buf_addr_msb` writer"]
pub type W = crate::W<BufAddrMsbSpec>;
#[doc = "Field `buf_addr` reader - Provides the MSB 32 bits of the location of the data to transfer."]
pub type BufAddrR = crate::FieldReader<u32>;
#[doc = "Field `buf_addr` writer - Provides the MSB 32 bits of the location of the data to transfer."]
pub type BufAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the MSB 32 bits of the location of the data to transfer."]
    #[inline(always)]
    pub fn buf_addr(&self) -> BufAddrR {
        BufAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Provides the MSB 32 bits of the location of the data to transfer."]
    #[inline(always)]
    #[must_use]
    pub fn buf_addr(&mut self) -> BufAddrW<BufAddrMsbSpec> {
        BufAddrW::new(self, 0)
    }
}
#[doc = "Upper 32 bits of Buffer Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_addr_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_addr_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufAddrMsbSpec;
impl crate::RegisterSpec for BufAddrMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_addr_msb::R`](R) reader structure"]
impl crate::Readable for BufAddrMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`buf_addr_msb::W`](W) writer structure"]
impl crate::Writable for BufAddrMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_addr_msb to value 0"]
impl crate::Resettable for BufAddrMsbSpec {
    const RESET_VALUE: u32 = 0;
}
