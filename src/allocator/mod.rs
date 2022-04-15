use spin::mutex::SpinMutex;

use crate::{
    singleton::Singleton,
    utils::{align_down, align_up},
};

pub const PAGE_SIZE: usize = 4 * (1 << 20); // 4MB

pub struct PageAllocator {
    start: usize,
    num_pages: usize,
    page_used: &'static mut [bool],
}

pub static PAGE_ALLOCATOR: Singleton<SpinMutex<PageAllocator>> = Singleton::new();

struct Page {
    addr: usize,
}

impl PageAllocator {
    pub fn new(start: usize, size: usize) -> Self {
        let end = start + size;

        let start = align_up(start, PAGE_SIZE);
        let end = align_down(end, PAGE_SIZE);
        let num_pages = (end - start) / PAGE_SIZE;
        let num_pages_for_num_pages_bool = (num_pages + PAGE_SIZE - 1) / PAGE_SIZE;
        let page_used = unsafe { core::slice::from_raw_parts_mut(start as *mut bool, num_pages) };
        for i in 0..num_pages_for_num_pages_bool {
            page_used[i] = true;
        }

        Self {
            start,
            num_pages,
            page_used,
        }
    }

    pub fn page_addr(&self, n: usize) -> usize {
        self.start + n * PAGE_SIZE
    }

    pub fn get_page(&mut self) -> Option<Page> {
        for i in 0..self.num_pages {
            if !self.page_used[i] {
                self.page_used[i] = true;
                return Some(Page {
                    addr: self.page_addr(i),
                });
            }
        }

        None
    }

    pub fn get_n_pages(&mut self, n: usize) -> Option<usize> {
        if self.num_pages < n {
            return None;
        }
        let end = self.num_pages - n;
        for i in 0..=end {
            if self.page_used[i..i + n].iter().all(|x| !*x) {
                self.page_used[i..i + n].fill(true);
                return Some(self.page_addr(i));
            }
        }
        None
    }
}

pub unsafe fn page_allocator_init(start: usize, size: usize) {
    unsafe {
        PAGE_ALLOCATOR.init(SpinMutex::new(PageAllocator::new(start, size)));
    }
}
