#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mm2s_dmacr: Mm2sDmacr,
    mm2s_dmasr: Mm2sDmasr,
    mm2s_curdesc: Mm2sCurdesc,
    mm2s_curdesc_msb: Mm2sCurdescMsb,
    mm2s_taildesc: Mm2sTaildesc,
    mm2s_taildesc_msb: Mm2sTaildescMsb,
    mm2s_sa: Mm2sSa,
    mm2s_sa_msb: Mm2sSaMsb,
    _reserved8: [u8; 0x08],
    mm2s_length: Mm2sLength,
    sg_ctl: SgCtl,
    s2mm_dmacr: S2mmDmacr,
    s2mm_dmasr: S2mmDmasr,
    s2mm_curdesc: S2mmCurdesc,
    s2mm_curdesc_msb: S2mmCurdescMsb,
    s2mm_taildesc: S2mmTaildesc,
    s2mm_taildesc_msb: S2mmTaildescMsb,
    s2mm_da: S2mmDa,
    s2mm_da_msb: S2mmDaMsb,
    _reserved18: [u8; 0x08],
    s2mm_length: S2mmLength,
}
impl RegisterBlock {
    #[doc = "0x00 - MM2S DMA Control register"]
    #[inline(always)]
    pub const fn mm2s_dmacr(&self) -> &Mm2sDmacr {
        &self.mm2s_dmacr
    }
    #[doc = "0x04 - MM2S DMA Status register"]
    #[inline(always)]
    pub const fn mm2s_dmasr(&self) -> &Mm2sDmasr {
        &self.mm2s_dmasr
    }
    #[doc = "0x08 - MM2S Current Descriptor Pointer. Lower 32 bits of the address."]
    #[inline(always)]
    pub const fn mm2s_curdesc(&self) -> &Mm2sCurdesc {
        &self.mm2s_curdesc
    }
    #[doc = "0x0c - MM2S Current Descriptor Pointer. Upper 32 bits of the address."]
    #[inline(always)]
    pub const fn mm2s_curdesc_msb(&self) -> &Mm2sCurdescMsb {
        &self.mm2s_curdesc_msb
    }
    #[doc = "0x10 - MM2S Tail Descriptor Pointer. Lower 32 bits of the address."]
    #[inline(always)]
    pub const fn mm2s_taildesc(&self) -> &Mm2sTaildesc {
        &self.mm2s_taildesc
    }
    #[doc = "0x14 - MM2S Tail Descriptor Pointer. Upper 32 bits of the address."]
    #[inline(always)]
    pub const fn mm2s_taildesc_msb(&self) -> &Mm2sTaildescMsb {
        &self.mm2s_taildesc_msb
    }
    #[doc = "0x18 - MM2S Source Address. Lower 32 bits of the address."]
    #[inline(always)]
    pub const fn mm2s_sa(&self) -> &Mm2sSa {
        &self.mm2s_sa
    }
    #[doc = "0x1c - MM2S Source Address. Upper 32 bits of the address."]
    #[inline(always)]
    pub const fn mm2s_sa_msb(&self) -> &Mm2sSaMsb {
        &self.mm2s_sa_msb
    }
    #[doc = "0x28 - MM2S Transfer Length (Bytes)"]
    #[inline(always)]
    pub const fn mm2s_length(&self) -> &Mm2sLength {
        &self.mm2s_length
    }
    #[doc = "0x2c - Scatter/Gather User and Cache"]
    #[inline(always)]
    pub const fn sg_ctl(&self) -> &SgCtl {
        &self.sg_ctl
    }
    #[doc = "0x30 - S2MM DMA Control register"]
    #[inline(always)]
    pub const fn s2mm_dmacr(&self) -> &S2mmDmacr {
        &self.s2mm_dmacr
    }
    #[doc = "0x34 - S2MM DMA Status register"]
    #[inline(always)]
    pub const fn s2mm_dmasr(&self) -> &S2mmDmasr {
        &self.s2mm_dmasr
    }
    #[doc = "0x38 - S2MM Current Descriptor Pointer. Lower 32 address bits."]
    #[inline(always)]
    pub const fn s2mm_curdesc(&self) -> &S2mmCurdesc {
        &self.s2mm_curdesc
    }
    #[doc = "0x3c - S2MM Current Descriptor Pointer. Upper 32 address bits."]
    #[inline(always)]
    pub const fn s2mm_curdesc_msb(&self) -> &S2mmCurdescMsb {
        &self.s2mm_curdesc_msb
    }
    #[doc = "0x40 - S2MM Tail Descriptor Pointer. Lower 32 address bits."]
    #[inline(always)]
    pub const fn s2mm_taildesc(&self) -> &S2mmTaildesc {
        &self.s2mm_taildesc
    }
    #[doc = "0x44 - S2MM Tail Descriptor Pointer. Upper 32 address bits."]
    #[inline(always)]
    pub const fn s2mm_taildesc_msb(&self) -> &S2mmTaildescMsb {
        &self.s2mm_taildesc_msb
    }
    #[doc = "0x48 - S2MM Destination Address. Lower 32 bit address"]
    #[inline(always)]
    pub const fn s2mm_da(&self) -> &S2mmDa {
        &self.s2mm_da
    }
    #[doc = "0x4c - S2MM Destination Address. Upper 32 bit address."]
    #[inline(always)]
    pub const fn s2mm_da_msb(&self) -> &S2mmDaMsb {
        &self.s2mm_da_msb
    }
    #[doc = "0x58 - S2MM Buffer Length (Bytes)"]
    #[inline(always)]
    pub const fn s2mm_length(&self) -> &S2mmLength {
        &self.s2mm_length
    }
}
#[doc = "mm2s_dmacr (rw) register accessor: MM2S DMA Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mm2s_dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mm2s_dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm2s_dmacr`]
module"]
#[doc(alias = "mm2s_dmacr")]
pub type Mm2sDmacr = crate::Reg<mm2s_dmacr::Mm2sDmacrSpec>;
#[doc = "MM2S DMA Control register"]
pub mod mm2s_dmacr;
#[doc = "mm2s_dmasr (rw) register accessor: MM2S DMA Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mm2s_dmasr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mm2s_dmasr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm2s_dmasr`]
module"]
#[doc(alias = "mm2s_dmasr")]
pub type Mm2sDmasr = crate::Reg<mm2s_dmasr::Mm2sDmasrSpec>;
#[doc = "MM2S DMA Status register"]
pub mod mm2s_dmasr;
#[doc = "mm2s_curdesc (rw) register accessor: MM2S Current Descriptor Pointer. Lower 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mm2s_curdesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mm2s_curdesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm2s_curdesc`]
module"]
#[doc(alias = "mm2s_curdesc")]
pub type Mm2sCurdesc = crate::Reg<mm2s_curdesc::Mm2sCurdescSpec>;
#[doc = "MM2S Current Descriptor Pointer. Lower 32 bits of the address."]
pub mod mm2s_curdesc;
#[doc = "mm2s_curdesc_msb (rw) register accessor: MM2S Current Descriptor Pointer. Upper 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mm2s_curdesc_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mm2s_curdesc_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm2s_curdesc_msb`]
module"]
#[doc(alias = "mm2s_curdesc_msb")]
pub type Mm2sCurdescMsb = crate::Reg<mm2s_curdesc_msb::Mm2sCurdescMsbSpec>;
#[doc = "MM2S Current Descriptor Pointer. Upper 32 bits of the address."]
pub mod mm2s_curdesc_msb;
#[doc = "mm2s_taildesc (rw) register accessor: MM2S Tail Descriptor Pointer. Lower 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mm2s_taildesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mm2s_taildesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm2s_taildesc`]
module"]
#[doc(alias = "mm2s_taildesc")]
pub type Mm2sTaildesc = crate::Reg<mm2s_taildesc::Mm2sTaildescSpec>;
#[doc = "MM2S Tail Descriptor Pointer. Lower 32 bits of the address."]
pub mod mm2s_taildesc;
#[doc = "mm2s_taildesc_msb (rw) register accessor: MM2S Tail Descriptor Pointer. Upper 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mm2s_taildesc_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mm2s_taildesc_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm2s_taildesc_msb`]
module"]
#[doc(alias = "mm2s_taildesc_msb")]
pub type Mm2sTaildescMsb = crate::Reg<mm2s_taildesc_msb::Mm2sTaildescMsbSpec>;
#[doc = "MM2S Tail Descriptor Pointer. Upper 32 bits of the address."]
pub mod mm2s_taildesc_msb;
#[doc = "mm2s_sa (rw) register accessor: MM2S Source Address. Lower 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mm2s_sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mm2s_sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm2s_sa`]
module"]
#[doc(alias = "mm2s_sa")]
pub type Mm2sSa = crate::Reg<mm2s_sa::Mm2sSaSpec>;
#[doc = "MM2S Source Address. Lower 32 bits of the address."]
pub mod mm2s_sa;
#[doc = "mm2s_sa_msb (rw) register accessor: MM2S Source Address. Upper 32 bits of the address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mm2s_sa_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mm2s_sa_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm2s_sa_msb`]
module"]
#[doc(alias = "mm2s_sa_msb")]
pub type Mm2sSaMsb = crate::Reg<mm2s_sa_msb::Mm2sSaMsbSpec>;
#[doc = "MM2S Source Address. Upper 32 bits of the address."]
pub mod mm2s_sa_msb;
#[doc = "mm2s_length (rw) register accessor: MM2S Transfer Length (Bytes)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mm2s_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mm2s_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm2s_length`]
module"]
#[doc(alias = "mm2s_length")]
pub type Mm2sLength = crate::Reg<mm2s_length::Mm2sLengthSpec>;
#[doc = "MM2S Transfer Length (Bytes)"]
pub mod mm2s_length;
#[doc = "sg_ctl (rw) register accessor: Scatter/Gather User and Cache\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sg_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sg_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sg_ctl`]
module"]
#[doc(alias = "sg_ctl")]
pub type SgCtl = crate::Reg<sg_ctl::SgCtlSpec>;
#[doc = "Scatter/Gather User and Cache"]
pub mod sg_ctl;
#[doc = "s2mm_dmacr (rw) register accessor: S2MM DMA Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2mm_dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2mm_dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2mm_dmacr`]
module"]
#[doc(alias = "s2mm_dmacr")]
pub type S2mmDmacr = crate::Reg<s2mm_dmacr::S2mmDmacrSpec>;
#[doc = "S2MM DMA Control register"]
pub mod s2mm_dmacr;
#[doc = "s2mm_dmasr (rw) register accessor: S2MM DMA Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2mm_dmasr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2mm_dmasr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2mm_dmasr`]
module"]
#[doc(alias = "s2mm_dmasr")]
pub type S2mmDmasr = crate::Reg<s2mm_dmasr::S2mmDmasrSpec>;
#[doc = "S2MM DMA Status register"]
pub mod s2mm_dmasr;
#[doc = "s2mm_curdesc (rw) register accessor: S2MM Current Descriptor Pointer. Lower 32 address bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2mm_curdesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2mm_curdesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2mm_curdesc`]
module"]
#[doc(alias = "s2mm_curdesc")]
pub type S2mmCurdesc = crate::Reg<s2mm_curdesc::S2mmCurdescSpec>;
#[doc = "S2MM Current Descriptor Pointer. Lower 32 address bits."]
pub mod s2mm_curdesc;
#[doc = "s2mm_curdesc_msb (rw) register accessor: S2MM Current Descriptor Pointer. Upper 32 address bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2mm_curdesc_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2mm_curdesc_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2mm_curdesc_msb`]
module"]
#[doc(alias = "s2mm_curdesc_msb")]
pub type S2mmCurdescMsb = crate::Reg<s2mm_curdesc_msb::S2mmCurdescMsbSpec>;
#[doc = "S2MM Current Descriptor Pointer. Upper 32 address bits."]
pub mod s2mm_curdesc_msb;
#[doc = "s2mm_taildesc (rw) register accessor: S2MM Tail Descriptor Pointer. Lower 32 address bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2mm_taildesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2mm_taildesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2mm_taildesc`]
module"]
#[doc(alias = "s2mm_taildesc")]
pub type S2mmTaildesc = crate::Reg<s2mm_taildesc::S2mmTaildescSpec>;
#[doc = "S2MM Tail Descriptor Pointer. Lower 32 address bits."]
pub mod s2mm_taildesc;
#[doc = "s2mm_taildesc_msb (rw) register accessor: S2MM Tail Descriptor Pointer. Upper 32 address bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2mm_taildesc_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2mm_taildesc_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2mm_taildesc_msb`]
module"]
#[doc(alias = "s2mm_taildesc_msb")]
pub type S2mmTaildescMsb = crate::Reg<s2mm_taildesc_msb::S2mmTaildescMsbSpec>;
#[doc = "S2MM Tail Descriptor Pointer. Upper 32 address bits."]
pub mod s2mm_taildesc_msb;
#[doc = "s2mm_da (rw) register accessor: S2MM Destination Address. Lower 32 bit address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2mm_da::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2mm_da::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2mm_da`]
module"]
#[doc(alias = "s2mm_da")]
pub type S2mmDa = crate::Reg<s2mm_da::S2mmDaSpec>;
#[doc = "S2MM Destination Address. Lower 32 bit address"]
pub mod s2mm_da;
#[doc = "s2mm_da_msb (rw) register accessor: S2MM Destination Address. Upper 32 bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2mm_da_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2mm_da_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2mm_da_msb`]
module"]
#[doc(alias = "s2mm_da_msb")]
pub type S2mmDaMsb = crate::Reg<s2mm_da_msb::S2mmDaMsbSpec>;
#[doc = "S2MM Destination Address. Upper 32 bit address."]
pub mod s2mm_da_msb;
#[doc = "s2mm_length (rw) register accessor: S2MM Buffer Length (Bytes)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2mm_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2mm_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2mm_length`]
module"]
#[doc(alias = "s2mm_length")]
pub type S2mmLength = crate::Reg<s2mm_length::S2mmLengthSpec>;
#[doc = "S2MM Buffer Length (Bytes)"]
pub mod s2mm_length;
