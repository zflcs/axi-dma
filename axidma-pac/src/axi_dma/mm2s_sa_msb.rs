#[doc = "Register `mm2s_sa_msb` reader"]
pub type R = crate::R<Mm2sSaMsbSpec>;
#[doc = "Register `mm2s_sa_msb` writer"]
pub type W = crate::W<Mm2sSaMsbSpec>;
#[doc = "Field `src_addr` reader - Indicates the MSB 32 bits of the source address AXI DMA reads from totransfer data to AXI4-Stream on the MM2S Channel."]
pub type SrcAddrR = crate::FieldReader<u32>;
#[doc = "Field `src_addr` writer - Indicates the MSB 32 bits of the source address AXI DMA reads from totransfer data to AXI4-Stream on the MM2S Channel."]
pub type SrcAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates the MSB 32 bits of the source address AXI DMA reads from totransfer data to AXI4-Stream on the MM2S Channel."]
    #[inline(always)]
    pub fn src_addr(&self) -> SrcAddrR {
        SrcAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates the MSB 32 bits of the source address AXI DMA reads from totransfer data to AXI4-Stream on the MM2S Channel."]
    #[inline(always)]
    #[must_use]
    pub fn src_addr(&mut self) -> SrcAddrW<Mm2sSaMsbSpec> {
        SrcAddrW::new(self, 0)
    }
}
#[doc = "MM2S Source Address. Upper 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mm2s_sa_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mm2s_sa_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mm2sSaMsbSpec;
impl crate::RegisterSpec for Mm2sSaMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm2s_sa_msb::R`](R) reader structure"]
impl crate::Readable for Mm2sSaMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`mm2s_sa_msb::W`](W) writer structure"]
impl crate::Writable for Mm2sSaMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mm2s_sa_msb to value 0"]
impl crate::Resettable for Mm2sSaMsbSpec {
    const RESET_VALUE: u32 = 0;
}
