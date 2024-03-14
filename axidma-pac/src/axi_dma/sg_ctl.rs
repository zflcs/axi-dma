#[doc = "Register `sg_ctl` reader"]
pub type R = crate::R<SgCtlSpec>;
#[doc = "Register `sg_ctl` writer"]
pub type W = crate::W<SgCtlSpec>;
#[doc = "Field `sg_cache` reader - Scatter/Gather Cache Control. Values written in this register reflect on the m_axi_sg_arcache and m_axi_sg_awcache signals of the M_AXI_SG interface."]
pub type SgCacheR = crate::FieldReader;
#[doc = "Field `sg_cache` writer - Scatter/Gather Cache Control. Values written in this register reflect on the m_axi_sg_arcache and m_axi_sg_awcache signals of the M_AXI_SG interface."]
pub type SgCacheW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sg_user` reader - Scatter/Gather User Control. Values written in this register reflect on the m_axi_sg_aruser and m_axi_sg_awuser signals of the M_AXI_SG interface."]
pub type SgUserR = crate::FieldReader;
#[doc = "Field `sg_user` writer - Scatter/Gather User Control. Values written in this register reflect on the m_axi_sg_aruser and m_axi_sg_awuser signals of the M_AXI_SG interface."]
pub type SgUserW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Scatter/Gather Cache Control. Values written in this register reflect on the m_axi_sg_arcache and m_axi_sg_awcache signals of the M_AXI_SG interface."]
    #[inline(always)]
    pub fn sg_cache(&self) -> SgCacheR {
        SgCacheR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Scatter/Gather User Control. Values written in this register reflect on the m_axi_sg_aruser and m_axi_sg_awuser signals of the M_AXI_SG interface."]
    #[inline(always)]
    pub fn sg_user(&self) -> SgUserR {
        SgUserR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Scatter/Gather Cache Control. Values written in this register reflect on the m_axi_sg_arcache and m_axi_sg_awcache signals of the M_AXI_SG interface."]
    #[inline(always)]
    #[must_use]
    pub fn sg_cache(&mut self) -> SgCacheW<SgCtlSpec> {
        SgCacheW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Scatter/Gather User Control. Values written in this register reflect on the m_axi_sg_aruser and m_axi_sg_awuser signals of the M_AXI_SG interface."]
    #[inline(always)]
    #[must_use]
    pub fn sg_user(&mut self) -> SgUserW<SgCtlSpec> {
        SgUserW::new(self, 8)
    }
}
#[doc = "Scatter/Gather User and Cache\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sg_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sg_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SgCtlSpec;
impl crate::RegisterSpec for SgCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sg_ctl::R`](R) reader structure"]
impl crate::Readable for SgCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`sg_ctl::W`](W) writer structure"]
impl crate::Writable for SgCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sg_ctl to value 0"]
impl crate::Resettable for SgCtlSpec {
    const RESET_VALUE: u32 = 0;
}
