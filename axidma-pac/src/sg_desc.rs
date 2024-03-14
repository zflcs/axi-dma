#[repr(C)]
#[doc = "Register block"]
#[derive(Default)]
pub struct RegisterBlock {
    nxt_desc: NxtDesc,
    nxt_desc_msb: NxtDescMsb,
    buf_addr: BufAddr,
    buf_addr_msb: BufAddrMsb,
    _reserved4: [u8; 0x08],
    control: Control,
    status: Status,
    app: [App; 5],
}
impl RegisterBlock {
    #[doc = "0x00 - Next Descriptor Pointer"]
    #[inline(always)]
    pub const fn nxt_desc(&self) -> &NxtDesc {
        &self.nxt_desc
    }
    #[doc = "0x04 - Upper 32 bits of Next Descriptor Pointer"]
    #[inline(always)]
    pub const fn nxt_desc_msb(&self) -> &NxtDescMsb {
        &self.nxt_desc_msb
    }
    #[doc = "0x08 - Buffer Address"]
    #[inline(always)]
    pub const fn buf_addr(&self) -> &BufAddr {
        &self.buf_addr
    }
    #[doc = "0x0c - Upper 32 bits of Buffer Address"]
    #[inline(always)]
    pub const fn buf_addr_msb(&self) -> &BufAddrMsb {
        &self.buf_addr_msb
    }
    #[doc = "0x18 - Control of BD"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x1c - Status of BD"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x20..0x34 - User Application Field \\[%s\\]"]
    #[inline(always)]
    pub const fn app(&self, n: usize) -> &App {
        &self.app[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x34 - User Application Field \\[%s\\]"]
    #[inline(always)]
    pub fn app_iter(&self) -> impl Iterator<Item = &App> {
        self.app.iter()
    }
}
#[doc = "nxt_desc (rw) register accessor: Next Descriptor Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nxt_desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nxt_desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nxt_desc`]
module"]
#[doc(alias = "nxt_desc")]
pub type NxtDesc = crate::Reg<nxt_desc::NxtDescSpec>;
#[doc = "Next Descriptor Pointer"]
pub mod nxt_desc;
#[doc = "nxt_desc_msb (rw) register accessor: Upper 32 bits of Next Descriptor Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nxt_desc_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nxt_desc_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nxt_desc_msb`]
module"]
#[doc(alias = "nxt_desc_msb")]
pub type NxtDescMsb = crate::Reg<nxt_desc_msb::NxtDescMsbSpec>;
#[doc = "Upper 32 bits of Next Descriptor Pointer"]
pub mod nxt_desc_msb;
#[doc = "buf_addr (rw) register accessor: Buffer Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_addr`]
module"]
#[doc(alias = "buf_addr")]
pub type BufAddr = crate::Reg<buf_addr::BufAddrSpec>;
#[doc = "Buffer Address"]
pub mod buf_addr;
#[doc = "buf_addr_msb (rw) register accessor: Upper 32 bits of Buffer Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_addr_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_addr_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_addr_msb`]
module"]
#[doc(alias = "buf_addr_msb")]
pub type BufAddrMsb = crate::Reg<buf_addr_msb::BufAddrMsbSpec>;
#[doc = "Upper 32 bits of Buffer Address"]
pub mod buf_addr_msb;
#[doc = "control (rw) register accessor: Control of BD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "control")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Control of BD"]
pub mod control;
#[doc = "status (r) register accessor: Status of BD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status of BD"]
pub mod status;
#[doc = "app (rw) register accessor: User Application Field \\[%s\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app`]
module"]
#[doc(alias = "app")]
pub type App = crate::Reg<app::AppSpec>;
#[doc = "User Application Field \\[%s\\]"]
pub mod app;
