#[cfg(feature = "bsp_qemu")]
#[path = "qemu/mod.rs"]
mod _bsp;

#[cfg(feature = "bsp_pinephone")]
#[path = "pinephone/mod.rs"]
mod _bsp;

pub use _bsp::*;
