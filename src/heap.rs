use crate::{allocator::{PAGE_ALLOCATOR, PAGE_SIZE}, println, sync::SpinMutex, utils::{self, align_down, align_up}};
use core::{
    alloc::{GlobalAlloc, Layout},
    mem, ptr,
};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new();

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn heap_init(size: usize) {
    let size = align_down(size, PAGE_SIZE);
    let mut page_allocator = PAGE_ALLOCATOR.get().lock();
    unsafe {
        let heap_start = page_allocator.get_n_pages(size / PAGE_SIZE).expect("Can't allocate for heap");
        let heap_end = heap_start + size;
        ALLOCATOR.init(heap_start, heap_end - heap_start)
    }
}

struct Allocator {
    inner: SpinMutex<AllocatorInner>,
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // perform layout adjustments
        let (size, align) = size_align(layout);
        let mut inner = self.inner.lock();

        if let Some((region, alloc_start)) = inner.find_region(size, align) {
            let alloc_end = alloc_start.checked_add(size).expect("overflow");
            let excess_size = region.end_addr() - alloc_end;
            if excess_size > 0 {
                unsafe {
                    inner.add_free_region(alloc_end, excess_size);
                }
            }
            alloc_start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // perform layout adjustments
        let (size, _) = size_align(layout);

        unsafe {
            self.inner.lock().add_free_region(ptr as usize, size);
        }
    }
}

impl Allocator {
    /// Creates an empty LinkedListAllocator.
    pub const fn new() -> Self {
        Self {
            inner: SpinMutex::new(AllocatorInner {
                head: ListNode::new(0),
            }),
        }
    }

    /// Initialize the allocator with the given heap bounds.
    ///
    /// This function is unsafe because the caller must guarantee that the given
    /// heap bounds are valid and that the heap is unused. This method must be
    /// called only once.
    pub unsafe fn init(&self, heap_start: usize, heap_size: usize) {
        println!(
            "Heap init start = 0x{:x}, size = {}MB",
            heap_start,
            heap_size / 1024 / 1024
        );
        unsafe {
            self.inner.lock().add_free_region(heap_start, heap_size);
        }
    }
}

/// Adjust the given layout so that the resulting allocated memory
/// region is also capable of storing a `ListNode`.
///
/// Returns the adjusted size and alignment as a (size, align) tuple.
fn size_align(layout: Layout) -> (usize, usize) {
    let layout = layout
        .align_to(mem::align_of::<ListNode>())
        .expect("adjusting alignment failed")
        .pad_to_align();
    let size = layout.size().max(mem::size_of::<ListNode>());
    (size, layout.align())
}

struct AllocatorInner {
    head: ListNode,
}

impl AllocatorInner {
    pub const fn new() -> Self {
        Self {
            head: ListNode::new(0),
        }
    }

    /// Adds the given memory region to the front of the list.
    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        // ensure that the freed region is capable of holding ListNode
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());

        // create a new list node and append it at the start of the list
        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        unsafe {
            node_ptr.write(node);
            self.head.next = Some(&mut *node_ptr)
        }
    }

    /// Looks for a free region with the given size and alignment and removes
    /// it from the list.
    ///
    /// Returns a tuple of the list node and the start address of the allocation.
    fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        // reference to current list node, updated for each iteration
        let mut current = &mut self.head;
        // look for a large enough memory region in linked list
        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
                // region suitable for allocation -> remove node from list
                let next = region.next.take();
                let ret = Some((current.next.take().unwrap(), alloc_start));
                current.next = next;
                return ret;
            } else {
                // region not suitable -> continue with next region
                current = current.next.as_mut().unwrap();
            }
        }

        // no suitable region found
        None
    }

    /// Try to use the given region for an allocation with given size and
    /// alignment.
    ///
    /// Returns the allocation start address on success.
    fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        if alloc_end > region.end_addr() {
            // region too small
            return Err(());
        }

        let excess_size = region.end_addr() - alloc_end;
        if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
            // rest of region too small to hold a ListNode (required because the
            // allocation splits the region in a used and a free part)
            return Err(());
        }

        // region suitable for allocation
        Ok(alloc_start)
    }
}

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}
