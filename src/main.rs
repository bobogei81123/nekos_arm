#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(unsafe_cell_get_mut)]

use core::{
    panic::PanicInfo,
    ptr,
};

use cortex_a::regs::*;

mod serial;
mod crazy;
mod exception;
mod cpu;

global_asm!(include_str!("start.s"));

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() {

    unsafe {
        exception::handling_init();
    }

    static ARR: [i64; 64] = [0; 64];

    println!("Current privilege = {}", exception::current_privilege_level());

    println!("AArch64 Bare Metal");

    let x = CNTFRQ_EL0.get();
    println!("Freq = {}\n", x);
    CNTP_TVAL_EL0.set(x);
    CNTP_CTL_EL0.set(1);

    loop {}
}
