use alloc::collections::VecDeque;
use alloc::{boxed::Box, vec::Vec};

use crate::{arch, singleton::Singleton, sync::SpinMutex};

#[repr(C)]
pub struct Thread {
    stack: Box<[u8]>,
    pub context: arch::thread::ThreadContext,
}

const STACK_SIZE: usize = 1024 * 1024;

impl Thread {
    pub fn new(func: fn()) -> Self {
        let mut stack = Vec::new();
        stack
            .try_reserve_exact(STACK_SIZE)
            .expect("could not allocate stack");
        stack.resize(STACK_SIZE, 0);
        let stack = stack.into_boxed_slice();
        let stack_end = stack.as_ptr_range().end;

        let context = arch::thread::ThreadContext::new(func, stack_end);

        Self { stack, context }
    }

    pub fn current() -> &'static Thread {
        unsafe { &*(arch::thread::current_thread() as *const Thread) }
    }

    pub fn yield_current() {
        // TODO: handle idle thread
        let current_thread = Thread::current() as *const _ as *mut _;
        let next_thread;
        {
            // TODO: multiple CPU case?
            let mut scheduler = SCHEDULER.get().lock();
            next_thread = scheduler.schedule();
            scheduler.queue.push_back(current_thread);
        }

        Thread::switch(current_thread, next_thread);
    }

    pub fn switch(from: *mut Thread, to: *mut Thread) {
        arch::thread::thread_switch(from, to);
    }

    pub fn start_with(init: *mut Thread) -> ! {
        let mut placeholder = Thread {
            stack: Box::new([]),
            context: arch::thread::ThreadContext::default(),
        };

        Thread::switch(&mut placeholder as *mut Thread, init);
        unreachable!();
    }
}

pub static SCHEDULER: Singleton<SpinMutex<Scheduler>> = Singleton::new();

pub struct Scheduler {
    queue: VecDeque<*mut Thread>,
    idle_thread: *mut Thread,
}

impl Scheduler {
    pub fn new(idle_thread: *mut Thread) -> Self {
        Self {
            queue: VecDeque::new(),
            idle_thread,
        }
    }

    pub fn add(&mut self, thread: *mut Thread) {
        self.queue.push_back(thread);
    }

    pub fn schedule(&mut self) -> *mut Thread {
        let next_thread = self.queue.pop_front().unwrap_or(self.idle_thread);
        next_thread
    }
}

// TODO: remove this when scheduler is per CPU
unsafe impl Send for Scheduler {}
