#![doc = "Peripheral access API for AXIDMA microcontrollers (generated using svd2rust v0.29.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.29.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
mod sg_desc_impl;
#[cfg(feature = "rt")]
extern "C" {}
#[doc(hidden)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    pub _reserved: usize,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[no_mangle]
pub static __EXTERNAL_INTERRUPTS: [Vector; 0] = [];
#[doc = "AXI Direct Memory Access"]
pub struct AXI_DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AXI_DMA {}
impl AXI_DMA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const axi_dma::RegisterBlock = 0x6010_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const axi_dma::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AXI_DMA {
    type Target = axi_dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AXI_DMA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_DMA").finish()
    }
}
#[doc = "AXI Direct Memory Access"]
pub mod axi_dma;
#[doc = "Scatter Gather Descriptor"]
pub struct SG_DESC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SG_DESC {}
impl SG_DESC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sg_desc::RegisterBlock = 0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sg_desc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SG_DESC {
    type Target = sg_desc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SG_DESC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SG_DESC").finish()
    }
}
#[doc = "Scatter Gather Descriptor"]
pub mod sg_desc;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AXI_DMA"]
    pub AXI_DMA: AXI_DMA,
    #[doc = "SG_DESC"]
    pub SG_DESC: SG_DESC,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AXI_DMA: AXI_DMA {
                _marker: PhantomData,
            },
            SG_DESC: SG_DESC {
                _marker: PhantomData,
            },
        }
    }
}
