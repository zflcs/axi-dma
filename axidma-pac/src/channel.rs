#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmacr: Dmacr,
    dmasr: Dmasr,
    curdesc: Curdesc,
    curdesc_msb: CurdescMsb,
    taildesc: Taildesc,
    taildesc_msb: TaildescMsb,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Channel Control register"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &Dmacr {
        &self.dmacr
    }
    #[doc = "0x04 - DMA Channel Status register"]
    #[inline(always)]
    pub const fn dmasr(&self) -> &Dmasr {
        &self.dmasr
    }
    #[doc = "0x08 - Current Descriptor Pointer. Lower 32 bits of the address."]
    #[inline(always)]
    pub const fn curdesc(&self) -> &Curdesc {
        &self.curdesc
    }
    #[doc = "0x0c - Current Descriptor Pointer. Upper 32 bits of the address."]
    #[inline(always)]
    pub const fn curdesc_msb(&self) -> &CurdescMsb {
        &self.curdesc_msb
    }
    #[doc = "0x10 - Tail Descriptor Pointer. Lower 32 bits of the address."]
    #[inline(always)]
    pub const fn taildesc(&self) -> &Taildesc {
        &self.taildesc
    }
    #[doc = "0x14 - Tail Descriptor Pointer. Upper 32 bits of the address."]
    #[inline(always)]
    pub const fn taildesc_msb(&self) -> &TaildescMsb {
        &self.taildesc_msb
    }
}
#[doc = "dmacr (rw) register accessor: DMA Channel Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacr`]
module"]
#[doc(alias = "dmacr")]
pub type Dmacr = crate::Reg<dmacr::DmacrSpec>;
#[doc = "DMA Channel Control register"]
pub mod dmacr;
#[doc = "dmasr (rw) register accessor: DMA Channel Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasr`]
module"]
#[doc(alias = "dmasr")]
pub type Dmasr = crate::Reg<dmasr::DmasrSpec>;
#[doc = "DMA Channel Status register"]
pub mod dmasr;
#[doc = "curdesc (rw) register accessor: Current Descriptor Pointer. Lower 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curdesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`curdesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curdesc`]
module"]
#[doc(alias = "curdesc")]
pub type Curdesc = crate::Reg<curdesc::CurdescSpec>;
#[doc = "Current Descriptor Pointer. Lower 32 bits of the address."]
pub mod curdesc;
#[doc = "curdesc_msb (rw) register accessor: Current Descriptor Pointer. Upper 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curdesc_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`curdesc_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curdesc_msb`]
module"]
#[doc(alias = "curdesc_msb")]
pub type CurdescMsb = crate::Reg<curdesc_msb::CurdescMsbSpec>;
#[doc = "Current Descriptor Pointer. Upper 32 bits of the address."]
pub mod curdesc_msb;
#[doc = "taildesc (rw) register accessor: Tail Descriptor Pointer. Lower 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taildesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taildesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taildesc`]
module"]
#[doc(alias = "taildesc")]
pub type Taildesc = crate::Reg<taildesc::TaildescSpec>;
#[doc = "Tail Descriptor Pointer. Lower 32 bits of the address."]
pub mod taildesc;
#[doc = "taildesc_msb (rw) register accessor: Tail Descriptor Pointer. Upper 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taildesc_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taildesc_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taildesc_msb`]
module"]
#[doc(alias = "taildesc_msb")]
pub type TaildescMsb = crate::Reg<taildesc_msb::TaildescMsbSpec>;
#[doc = "Tail Descriptor Pointer. Upper 32 bits of the address."]
pub mod taildesc_msb;
