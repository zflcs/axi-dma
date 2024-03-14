#[doc = "Register `s2mm_da` reader"]
pub type R = crate::R<S2mmDaSpec>;
#[doc = "Register `s2mm_da` writer"]
pub type W = crate::W<S2mmDaSpec>;
#[doc = "Field `dst_addr` reader - Indicates the destination address the AXI DMA writes to transfer data from AXI4-Stream on S2MM Channel."]
pub type DstAddrR = crate::FieldReader<u32>;
#[doc = "Field `dst_addr` writer - Indicates the destination address the AXI DMA writes to transfer data from AXI4-Stream on S2MM Channel."]
pub type DstAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates the destination address the AXI DMA writes to transfer data from AXI4-Stream on S2MM Channel."]
    #[inline(always)]
    pub fn dst_addr(&self) -> DstAddrR {
        DstAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates the destination address the AXI DMA writes to transfer data from AXI4-Stream on S2MM Channel."]
    #[inline(always)]
    #[must_use]
    pub fn dst_addr(&mut self) -> DstAddrW<S2mmDaSpec> {
        DstAddrW::new(self, 0)
    }
}
#[doc = "S2MM Destination Address. Lower 32 bit address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2mm_da::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2mm_da::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2mmDaSpec;
impl crate::RegisterSpec for S2mmDaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2mm_da::R`](R) reader structure"]
impl crate::Readable for S2mmDaSpec {}
#[doc = "`write(|w| ..)` method takes [`s2mm_da::W`](W) writer structure"]
impl crate::Writable for S2mmDaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets s2mm_da to value 0"]
impl crate::Resettable for S2mmDaSpec {
    const RESET_VALUE: u32 = 0;
}
