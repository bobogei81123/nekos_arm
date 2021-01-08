#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]

use core::{
    panic::PanicInfo,
    ptr,
};

global_asm!(include_str!("start.s"));

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe fn outb(addr: u32, val: u32) {
    ptr::write_volatile(addr as *mut u32, val);
}

unsafe fn inb(addr: u32) -> u32 {
    ptr::read_volatile(addr as *mut u32)
}

fn wait(cycle: usize) {
    for _ in 0..cycle {
        unsafe {
            llvm_asm!("nop" : : : : "volatile");
        }
    }
}

enum Color {
    RED,
    GREEN,
    BLUE,
}

const PD_DATA_REG: u32 = 0x01c2087c;

fn blink(color: Color, times: usize) {
    let val: u32 = match color {
        Color::BLUE => 0x00040000,
        Color::RED => 0x00080000,
        Color::GREEN => 0x00100000,
    };

    for _ in 0..times {
        unsafe { outb(PD_DATA_REG, val); }
        wait(100_000_000);
        unsafe { outb(PD_DATA_REG, 0); }
        wait(100_000_000);
    }

    unsafe { outb(PD_DATA_REG, 0x001c0000); }
}

#[no_mangle]
pub extern "C" fn main() {

    unsafe {
        outb(0x01c20874, 0x77711177);
        blink(Color::GREEN, 5);
    }

    unsafe {
        // GATING
        outb(0x01c2006c, 0x00070000);
        blink(Color::BLUE, 5);

        const UART0_BASE: u32 = 0x01c28000;
        const UART0_DLL: u32 = UART0_BASE + 0x00;
        const UART0_DLH: u32 = UART0_BASE + 0x04;
        const UART0_LCR: u32 = UART0_BASE + 0x0c;
        const UART0_LSR: u32 = UART0_BASE + 0x14;
        const UART0_USR: u32 = UART0_BASE + 0x7c;
        const UART0_THR: u32 = UART0_BASE + 0x00;

        fn wait_UART0() {
           unsafe { while (inb(UART0_USR) & 1) != 0 {} }
        }

        wait_UART0();
        outb(UART0_LCR, 1 << 7);
        blink(Color::RED, 2);

        wait_UART0();
        outb(UART0_DLL, 13);
        blink(Color::BLUE, 2);

        wait_UART0();
        outb(UART0_DLH, 0);
        blink(Color::RED, 2);

        wait_UART0();
        outb(UART0_LCR, 0x03);
        blink(Color::BLUE, 2);

        loop {
            for c in "Hello world from Pinephone!".bytes() {
                while (inb(UART0_LSR) & (1 << 5)) == 0 { }
                outb(UART0_THR, c as u32);
            }
            blink(Color::GREEN, 2);
        }
    }

    //const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    //let out_str = b"AArch64 Bare Metal";
    //for byte in out_str {
        //unsafe {
            //ptr::write_volatile(UART0, *byte);
        //}
    //}
}
