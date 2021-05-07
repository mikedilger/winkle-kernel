
use crate::target::AtomicPtr;
use core::cell::UnsafeCell;
use core::marker::Sync;

pub trait Atomic {
    type T;

    /// This gets a mutable pointer to the internal value
    /// Such a pointer is unsafe.
    unsafe fn as_mut_ptr(&self) -> *mut Self::T;

    /// Store a value atomically.
    fn store(&self, t: Self::T);

    /// Store a value atomically.  This has acquire semantics
    fn store_acq(&self, t: Self::T);

    /// Store a value atomically.  This has release semantics
    fn store_rel(&self, t: Self::T);

    /// Store a value atomically.  This has sequentially consistent semantics
    fn store_seqcst(&self, t: Self::T);

    /// Fetch a value atomically.  This has acquire semantics
    fn fetch(&self) -> Self::T;

    /// Fetch a value atomically.  This has sequentially consistent semantics
    fn fetch_seqcst(&self) -> Self::T;

    /// Swap values atomically.  This has no ordering protections
    fn swap(&self, t: Self::T) -> Self::T;

    /// Swap values atomically.  This has sequentially consistent semantics
    fn swap_seqcst(&self, t: Self::T) -> Self::T;

    /// Compares to `compare_to` and if equal replaces with `t`.
    /// Returns the previous value, so if return value == compare_to it swapped,
    /// otherwise the compare failed and nothing happened.
    /// This has sequentially consistent semantics
    fn compare_and_swap(&self, compare_to: Self::T, t: Self::T) -> Self::T;

    /// Add the value t, and return the prior value.
    /// This has no reordering semantics
    fn fetch_add(&self, t: Self::T) -> Self::T;

    /// Subtract the value t, and return the prior value.
    /// This has no reordering semantics
    fn fetch_sub(&self, t: Self::T) -> Self::T;

    /// And with the value t, and return the prior value.
    /// This has no reordering semantics
    fn fetch_and(&self, t: Self::T) -> Self::T;

    /// Or with the value t, and return the prior value.
    /// This has no reordering semantics
    fn fetch_or(&self, t: Self::T) -> Self::T;

    /// Xor with the value t, and return the prior value.
    /// This has no reordering semantics
    fn fetch_xor(&self, t: Self::T) -> Self::T;

    /// Set the value to the maximum of t and itself, and return the prior value.
    /// This has no reordering semantics
    fn fetch_max(&self, t: Self::T) -> Self::T;

    /// Set the value to the minimum of t and itself, and return the prior value.
    /// This has no reordering semantics
    fn fetch_min(&self, t: Self::T) -> Self::T;
}

macro_rules! impl_derived_atomic {
    ($name:ident, $typ:ty, using $innertyp:ty) => (
        pub struct $name(UnsafeCell<$innertyp>);

        impl $name {
            #[allow(dead_code)]
            #[inline(always)]
            pub const fn new(t: $typ) -> $name {
                $name(UnsafeCell::new(t as $innertyp))
            }

            #[allow(dead_code)]
            #[inline(always)]
            pub fn into_inner(self) -> $typ {
                self.0.into_inner() as $typ
            }
        }

        impl Atomic for $name {
            type T = $typ;

            #[inline(always)]
            unsafe fn as_mut_ptr(&self) -> *mut Self::T {
                self.0.get() as *mut $typ
            }

            #[inline(always)]
            fn store(&self, t: Self::T) {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.store(t as $innertyp)
            }

            #[inline(always)]
            fn store_acq(&self, t: Self::T) {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.store_acq(t as $innertyp)
            }

            #[inline(always)]
            fn store_rel(&self, t: Self::T) {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.store_rel(t as $innertyp)
            }

            #[inline(always)]
            fn store_seqcst(&self, t: Self::T) {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.store_seqcst(t as $innertyp)
            }

            #[inline(always)]
            fn fetch(&self) -> Self::T {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.fetch() as $typ
            }

            #[inline(always)]
            fn fetch_seqcst(&self) -> Self::T {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.fetch_seqcst() as $typ
            }

            #[inline(always)]
            fn swap(&self, t: Self::T) -> Self::T {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.swap(t as $innertyp) as $typ
            }

            #[inline(always)]
            fn swap_seqcst(&self, t: Self::T) -> Self::T {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.swap_seqcst(t as $innertyp) as $typ
            }

            #[inline(always)]
            fn compare_and_swap(&self, compare_to: Self::T, t: Self::T) -> Self::T {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.compare_and_swap(compare_to as $innertyp, t as $innertyp) as $typ
            }

            #[inline(always)]
            fn fetch_add(&self, t: Self::T) -> Self::T {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.fetch_add(t as $innertyp) as $typ
            }

            #[inline(always)]
            fn fetch_sub(&self, t: Self::T) -> Self::T {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.fetch_sub(t as $innertyp) as $typ
            }

            #[inline(always)]
            fn fetch_and(&self, t: Self::T) -> Self::T {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.fetch_and(t as $innertyp) as $typ
            }

            #[inline(always)]
            fn fetch_or(&self, t: Self::T) -> Self::T {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.fetch_or(t as $innertyp) as $typ
            }

            #[inline(always)]
            fn fetch_xor(&self, t: Self::T) -> Self::T {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.fetch_xor(t as $innertyp) as $typ
            }

            #[inline(always)]
            fn fetch_max(&self, t: Self::T) -> Self::T {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.fetch_max(t as $innertyp) as $typ
            }

            #[inline(always)]
            fn fetch_min(&self, t: Self::T) -> Self::T {
                unsafe {
                    AtomicPtr::<$innertyp>::new(self.0.get())
                }.fetch_min(t as $innertyp) as $typ
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}
    );
}

impl_derived_atomic!(AtomicI8, i8, using i32);
impl_derived_atomic!(AtomicU8, u8, using u32);
impl_derived_atomic!(AtomicI16, i16, using i32);
impl_derived_atomic!(AtomicU16, u16, using u32);
impl_derived_atomic!(AtomicI32, i32, using i32);
impl_derived_atomic!(AtomicU32, u32, using u32);
impl_derived_atomic!(AtomicI64, i64, using i64);
impl_derived_atomic!(AtomicU64, u64, using u64);
impl_derived_atomic!(AtomicISize, isize, using isize);
impl_derived_atomic!(AtomicUSize, usize, using usize);

// Unfortunately rust makes casting into bool an error.
// So we cannot use the above macro, we have to do custom bool-specific code

pub struct AtomicBool(UnsafeCell<u32>);

impl AtomicBool {
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn new(t: bool) -> AtomicBool {
        AtomicBool(UnsafeCell::new(t as u32))
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn into_inner(self) -> bool {
        self.0.into_inner() != 0
    }
}

impl Atomic for AtomicBool {
    type T = bool;

    #[inline(always)]
    unsafe fn as_mut_ptr(&self) -> *mut Self::T {
        self.0.get() as *mut bool
    }

    #[inline(always)]
    fn store(&self, t: Self::T) {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.store(t as u32)
    }

    #[inline(always)]
    fn store_acq(&self, t: Self::T) {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.store_acq(t as u32)
    }

    #[inline(always)]
    fn store_rel(&self, t: Self::T) {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.store_rel(t as u32)
    }

    #[inline(always)]
    fn store_seqcst(&self, t: Self::T) {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.store_seqcst(t as u32)
    }

    #[inline(always)]
    fn fetch(&self) -> Self::T {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.fetch() != 0
    }

    #[inline(always)]
    fn fetch_seqcst(&self) -> Self::T {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.fetch_seqcst() != 0
    }

    #[inline(always)]
    fn swap(&self, t: Self::T) -> Self::T {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.swap(t as u32) != 0
    }

    #[inline(always)]
    fn swap_seqcst(&self, t: Self::T) -> Self::T {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.swap_seqcst(t as u32) != 0
    }

    #[inline(always)]
    fn compare_and_swap(&self, compare_to: Self::T, t: Self::T) -> Self::T {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.compare_and_swap(compare_to as u32, t as u32) != 0
    }

    #[inline(always)]
    fn fetch_add(&self, t: Self::T) -> Self::T {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.fetch_add(t as u32) != 0
    }

    #[inline(always)]
    fn fetch_sub(&self, t: Self::T) -> Self::T {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.fetch_sub(t as u32) != 0
    }

    #[inline(always)]
    fn fetch_and(&self, t: Self::T) -> Self::T {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.fetch_and(t as u32) != 0
    }

    #[inline(always)]
    fn fetch_or(&self, t: Self::T) -> Self::T {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.fetch_or(t as u32) != 0
    }

    #[inline(always)]
    fn fetch_xor(&self, t: Self::T) -> Self::T {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.fetch_xor(t as u32) != 0
    }

    #[inline(always)]
    fn fetch_max(&self, t: Self::T) -> Self::T {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.fetch_max(t as u32) != 0
    }

    #[inline(always)]
    fn fetch_min(&self, t: Self::T) -> Self::T {
        unsafe {
            AtomicPtr::<u32>::new(self.0.get())
        }.fetch_min(t as u32) != 0
    }
}

unsafe impl Send for AtomicBool {}
unsafe impl Sync for AtomicBool {}
