pub mod boot;
pub mod exception;
pub mod fdt;
pub mod thread;

pub fn system_off() -> ! {
    extern "Rust" {
        fn __system_off() -> !;
    }

    unsafe { __system_off() }
}
