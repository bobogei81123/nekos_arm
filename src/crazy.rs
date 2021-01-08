use core::cell::UnsafeCell;

pub struct CrazyCell<T> {
    inner: UnsafeCell<T>
}

unsafe impl<T> Sync for CrazyCell<T> {}
unsafe impl<T> Send for CrazyCell<T> {}

impl<T> CrazyCell<T> {
    pub const fn new(value: T) -> CrazyCell<T> {
        CrazyCell {
            inner: UnsafeCell::new(value)
        }
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }

    pub const fn get(&self) -> *mut T {
        self.inner.get()
    }
}
