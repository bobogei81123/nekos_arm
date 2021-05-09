use crate::thread::Thread;

use cortex_a::regs::{RegisterReadWrite, TPIDR_EL1};

global_asm!(include_str!("routine.s"));

#[repr(C)]
#[derive(Default, Debug)]
pub struct ThreadContext {
    x19: u64,
    x20: u64,
    x21: u64,
    x22: u64,
    x23: u64,
    x24: u64,
    x25: u64,
    x26: u64,
    x27: u64,
    x28: u64,
    x29: u64,
    lr: u64,
    sp: u64,
}

impl ThreadContext {
    pub fn new(func: fn(), stack: *const u8) -> Self {
        Self {
            lr: func as u64,
            sp: stack as u64,
            ..Self::default()
        }
    }
}

pub fn current_thread() -> *mut () {
    TPIDR_EL1.get() as *mut _
}

pub fn thread_switch(from: *mut Thread, to: *mut Thread) {
    extern "C" {
        fn __context_switch(from: *mut ThreadContext, to: *mut ThreadContext);
    }

    unsafe {
        TPIDR_EL1.set(to as u64);
        __context_switch(&mut (*from).context as *mut _, &mut (*to).context as *mut _);
    }
}
