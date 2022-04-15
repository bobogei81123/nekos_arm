#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]
#![feature(const_mut_refs)]
#![feature(custom_test_frameworks)]
#![feature(never_type)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_harness_main"]
#![deny(unsafe_op_in_unsafe_fn)]


mod allocator;
mod heap;
mod serial;
mod singleton;
mod sync;
mod thread;
mod utils;

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
mod arch;

mod bsp;

#[doc(inline)]
pub use core;
#[doc(inline)]
pub extern crate alloc;

use alloc::boxed::Box;
use core::panic::PanicInfo;

use crate::allocator::PAGE_SIZE;
use crate::serial::serial_init;
use crate::thread::{Thread, SCHEDULER};

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
pub extern "C" fn _main() {
    #[cfg(not(test))]
    {
        main();
    }
    #[cfg(test)]
    {
        test_main();
    }
}

#[cfg(test)]
pub fn test_main() {
    init_bss();
    test_harness_main();
    arch::system_off();
}

pub fn main() {
    use spin::mutex::SpinMutex;
    use tock_registers::interfaces::Readable;

    serial_init();

    println!("Hello from {}!", bsp::BOARD_NAME);
    println!(
        "Currently running in EL {}",
        cortex_a::registers::CurrentEL.read(cortex_a::registers::CurrentEL::EL)
    );

    loop {}
    //init_bss();
    //unsafe {
    //arch::fdt::fdt_init();
    //}
    //unsafe {
    //arch::exception::handling_init();
    //}
    //let (start, size) = arch::fdt::fdt_get_memory();
    //let end = start + size;
    //extern "Rust" {
    //static __EXT_STACK_END: ();
    //}
    //let heap_start = utils::align_up(unsafe { &__EXT_STACK_END } as *const () as usize, PAGE_SIZE);
    //unsafe {
    //allocator::page_allocator_init(heap_start, end as usize - heap_start);
    //}
    //println!("OK");

    //heap::heap_init(size as usize / 16);

    //let idle_thread = Box::leak(Box::try_new(Thread::new(idle)).unwrap()) as *mut _;
    //let init_thread = Box::leak(Box::try_new(Thread::new(init)).unwrap()) as *mut _;

    //unsafe {
    //use crate::thread::Scheduler;

    //SCHEDULER.init(SpinMutex::new(Scheduler::new(idle_thread)));
    //Thread::start_with(init_thread);
    //}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn test_works() {
        assert!(true);
    }
}
