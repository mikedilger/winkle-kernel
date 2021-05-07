
use core::cell::UnsafeCell;
use core::ops::{Drop, Deref, DerefMut};
use crate::atomic::{Atomic, AtomicBool};

pub struct Spinlock<T: ?Sized> {
    locked: AtomicBool,
    data: UnsafeCell<T>
}

impl<T> Spinlock<T> {
    #[allow(dead_code)]
    pub const fn new(data: T) -> Spinlock<T> {
        Spinlock {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data)
        }
    }

    // If we can call this, no other code has a reference to self, so it
    // wasn't locked
    #[allow(dead_code)]
    pub fn into_inner(self) -> T {
        let Spinlock { data, .. } = self;
        data.into_inner()
    }
}

impl<T: ?Sized> Spinlock<T> {
    /// Lock and return a guard
    #[allow(dead_code)]
    pub fn lock(&self) -> SpinlockGuard<T> {
        let mut previous_value = true;
        while previous_value != false {
            previous_value = self.locked.compare_and_swap(false, true);
        }
        SpinlockGuard {
            spinlock: &self,
            data: unsafe { &mut *self.data.get() }
        }
    }

    /// Break through the lock and get the mutable data anyways.  This is
    /// useful in emergencies (e.g. when panicking so that panic can print
    /// something).
    #[allow(dead_code)]
    pub unsafe fn breaklock(&self) -> &mut T {
        &mut *self.data.get()
    }

    /// Get whether it is locked. Unsafe as the result cannot be relied upon
    /// as it is not synchronzied in any way.
    #[allow(dead_code)]
    pub unsafe fn is_locked(&self) -> bool {
        self.locked.fetch()
    }
}

unsafe impl<T: ?Sized + Send> Sync for Spinlock<T> {}

unsafe impl<T: ?Sized + Send> Send for Spinlock<T> {}


pub struct SpinlockGuard<'a, T: ?Sized + 'a> {
    spinlock: &'a Spinlock<T>,
    data: &'a mut T,
}

impl<'a, T: ?Sized> Deref for SpinlockGuard<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T { &*self.data }
}

impl<'a, T: ?Sized> DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T { &mut *self.data }
}

impl<'a, T: ?Sized> Drop for SpinlockGuard<'a, T> {
    /// The dropping of the SpinlockGuard will release the lock it was created from.
    fn drop(&mut self) {
        if cfg!(debug_assertions) {
            if ! self.spinlock.locked.fetch() {
                panic!("lock dropped, but not locked!!!");
            }
        }
        self.spinlock.locked.store_rel(false);
    }
}
