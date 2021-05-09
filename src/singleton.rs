use core::cell::UnsafeCell;

pub struct Singleton<T> {
    inner: UnsafeCell<Option<T>>,
}

impl<T> Singleton<T> {
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }

    pub const fn new_with(val: T) -> Self {
        Self {
            inner: UnsafeCell::new(Some(val)),
        }
    }

    pub unsafe fn init(&self, val: T) {
        let opt = unsafe { &mut *self.inner.get() };
        match opt {
            Some(_) => {
                panic!("singleton instance {:p} was initialized twice", self);
            }
            None => {
                *opt = Some(val);
            }
        }
    }

    pub fn get(&self) -> &T {
        let opt = unsafe { &*self.inner.get() };
        opt.as_ref()
            .expect("singleton instance {:p} was not initialized")
    }

    pub unsafe fn get_mut(&self) -> &mut T {
        let opt = unsafe { &mut *self.inner.get() };
        opt.as_mut()
            .expect("singleton instance {:p} was not initialized")
    }
}

unsafe impl<T: Sync> Sync for Singleton<T> {}
