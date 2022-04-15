use core::{ptr, slice};

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BE<T>(T);

impl From<u32> for BE<u32> {
    fn from(x: u32) -> Self {
        BE(x.to_be())
    }
}

impl From<BE<u32>> for u32 {
    fn from(x: BE<u32>) -> Self {
        u32::from_be(x.0)
    }
}

impl From<u64> for BE<u64> {
    fn from(x: u64) -> Self {
        BE(x.to_be())
    }
}

impl From<BE<u64>> for u64 {
    fn from(x: BE<u64>) -> Self {
        u64::from_be(x.0)
    }
}

pub trait PointerExt<T> {
    unsafe fn read(self) -> T;
    unsafe fn next(&mut self) -> T;
}

impl<T> PointerExt<T> for *const T {
    unsafe fn read(self) -> T {
        unsafe { ptr::read(self) }
    }

    unsafe fn next(&mut self) -> T {
        let ptr = *self;
        unsafe {
            *self = self.add(1);
            ptr::read(ptr)
        }
    }
}

pub unsafe fn to_cstr<'a>(ptr: *const u8) -> &'a [u8] {
    let mut end = ptr;
    unsafe {
        while end.next() != 0 {}
        let len = end.offset_from(ptr) as usize;
        slice::from_raw_parts(ptr, len - 1)
    }
}

pub unsafe fn align_to(ptr: *const u8, align: usize) -> *const u8 {
    align_up(ptr as _, align) as _
}

/// Returns the value of `addr` aligned up to `align`.
pub fn align_up(addr: usize, align: usize) -> usize {
    assert!(align.is_power_of_two(), "`align` must be a power of 2");

    (addr + align - 1) & !(align - 1)
}

/// Returns the value of `addr` aligned down to `align`.
pub fn align_down(addr: usize, align: usize) -> usize {
    assert!(align.is_power_of_two(), "`align` must be a power of 2");

    addr & !(align - 1)
}

#[macro_export]
macro_rules! dbg {
    () => {
        $crate::println!("[{}:{}]", file!(), line!());
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::println!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
