#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(try_reserve)]
#![feature(allocator_api)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::panic::PanicInfo;

#[doc(inline)]
pub use core;

use thread::SCHEDULER;

#[doc(inline)]
pub extern crate alloc;

use alloc::boxed::Box;

use crate::{sync::SpinMutex, thread::Thread};

mod singleton;
mod heap;
mod serial;
mod sync;
mod thread;
mod utils;

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
mod arch;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    arch::system_off();
}

fn init_bss() {
    println!("Initialize bss");

    extern "Rust" {
        static __EXT_BSS_START: ();
        static __EXT_BSS_END_INCLUSIVE: ();
    }

    unsafe {
        let start = &__EXT_BSS_START as *const () as *mut u8;
        let end = &__EXT_BSS_START as *const () as *mut u8;
        let buf = core::slice::from_raw_parts_mut(start, end.offset_from(start) as usize);
        for byte in buf {
            *byte = 0;
        }
    }
}

fn thread1() {
    for _ in 0..10 {
        println!("Hello from thread #1");
        Thread::yield_current();
    }
}

fn thread2() {
    for _ in 0..9 {
        println!("Hello from thread #2");
        Thread::yield_current();
    }

    println!("Hello from thread #2");
    arch::system_off()
}

fn init() {
    println!("Hello from init");
    let thread1 = Box::leak(Box::new(Thread::new(thread1))) as *mut _;
    let thread2 = Box::leak(Box::new(Thread::new(thread2))) as *mut _;

    {
        let mut scheduler = SCHEDULER.get().lock();
        scheduler.add(thread1);
        scheduler.add(thread2);
    }

    loop {
        Thread::yield_current();
    }
}

fn idle() {
    loop {
        cortex_a::asm::wfi();
        Thread::yield_current();
    }
}

#[no_mangle]
pub extern "C" fn main() {
    println!("Hello!");

    init_bss();

    unsafe {
        arch::fdt::fdt_init();
    }

    unsafe {
        arch::exception::handling_init();
    }

    let (start, size) = arch::fdt::fdt_get_memory();
    heap::heap_init(start, size);

    println!("OK");

    let idle_thread = Box::leak(Box::try_new(Thread::new(idle)).unwrap()) as *mut _;
    let init_thread = Box::leak(Box::try_new(Thread::new(init)).unwrap()) as *mut _;

    unsafe {
        use crate::thread::Scheduler;

        SCHEDULER.init(SpinMutex::new(Scheduler::new(idle_thread)));
        Thread::start_with(init_thread);
    }
}
