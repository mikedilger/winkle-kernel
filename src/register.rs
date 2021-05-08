
use crate::target::AtomicPtr;
use crate::atomic::Atomic;

macro_rules! impl_atomic_register_ro {
    ($typ:ident, $inner:ty) => (
        pub struct $typ(AtomicPtr<$inner>);

        impl $typ {
            #[allow(dead_code)]
            #[inline(always)]
            pub const unsafe fn new(addr: usize) -> $typ {
                $typ(AtomicPtr::<$inner>::new_address(addr))
            }

            #[allow(dead_code)]
            pub fn fetch(&self) -> $inner {
                self.0.fetch()
            }

            #[allow(dead_code)]
            pub fn fetch_seqcst(&self) -> $inner {
                self.0.fetch_seqcst()
            }
        }
    );
}

impl_atomic_register_ro!(AtomicRegisterU32RO, u32);
impl_atomic_register_ro!(AtomicRegisterI32RO, i32);
impl_atomic_register_ro!(AtomicRegisterU64RO, u64);
impl_atomic_register_ro!(AtomicRegisterI64RO, i64);

macro_rules! impl_atomic_register_wo {
    ($typ:ident, $inner:ty) => (
        pub struct $typ(AtomicPtr<$inner>);

        impl $typ {
            #[allow(dead_code)]
            #[inline(always)]
            pub const unsafe fn new(addr: usize) -> $typ {
                $typ(AtomicPtr::<$inner>::new_address(addr))
            }

            #[allow(dead_code)]
            pub fn store(&self, t: $inner) {
                self.0.store(t)
            }

            #[allow(dead_code)]
            pub fn store_acq(&self, t: $inner) {
                self.0.store_acq(t)
            }

            #[allow(dead_code)]
            pub fn store_rel(&self, t: $inner) {
                self.0.store_rel(t)
            }

            #[allow(dead_code)]
            pub fn store_seqcst(&self, t: $inner) {
                self.0.store_seqcst(t)
            }
        }
    );
}

impl_atomic_register_wo!(AtomicRegisterU32WO, u32);
impl_atomic_register_wo!(AtomicRegisterI32WO, i32);
impl_atomic_register_wo!(AtomicRegisterU64WO, u64);
impl_atomic_register_wo!(AtomicRegisterI64WO, i64);

macro_rules! impl_atomic_register_rw {
    ($typ:ident, $inner:ty) => (
        pub struct $typ(AtomicPtr<$inner>);

        impl $typ {
            #[allow(dead_code)]
            #[inline(always)]
            pub const unsafe fn new(addr: usize) -> $typ {
                $typ(AtomicPtr::<$inner>::new_address(addr))
            }

            #[allow(dead_code)]
            pub fn store(&self, t: $inner) {
                self.0.store(t)
            }

            #[allow(dead_code)]
            pub fn store_acq(&self, t: $inner) {
                self.0.store_acq(t)
            }

            #[allow(dead_code)]
            pub fn store_rel(&self, t: $inner) {
                self.0.store_rel(t)
            }

            #[allow(dead_code)]
            pub fn store_seqcst(&self, t: $inner) {
                self.0.store_seqcst(t)
            }

            #[allow(dead_code)]
            pub fn fetch(&self) -> $inner {
                self.0.fetch()
            }

            #[allow(dead_code)]
            pub fn fetch_seqcst(&self) -> $inner {
                self.0.fetch_seqcst()
            }

            #[allow(dead_code)]
            pub fn swap(&self, t: $inner) -> $inner {
                self.0.swap(t)
            }

            #[allow(dead_code)]
            pub fn swap_seqcst(&self, t: $inner) -> $inner {
                self.0.swap_seqcst(t)
            }

            #[allow(dead_code)]
            pub fn compare_and_swap(&self, compare: $inner, t: $inner) -> $inner {
                self.0.compare_and_swap(compare, t)
            }

            #[allow(dead_code)]
            pub fn fetch_add(&self, t: $inner) -> $inner {
                self.0.fetch_add(t)
            }

            #[allow(dead_code)]
            pub fn fetch_sub(&self, t: $inner) -> $inner {
                self.0.fetch_sub(t)
            }

            #[allow(dead_code)]
            pub fn fetch_and(&self, t: $inner) -> $inner {
                self.0.fetch_and(t)
            }

            #[allow(dead_code)]
            pub fn fetch_or(&self, t: $inner) -> $inner {
                self.0.fetch_or(t)
            }

            #[allow(dead_code)]
            pub fn fetch_xor(&self, t: $inner) -> $inner {
                self.0.fetch_xor(t)
            }

            #[allow(dead_code)]
            pub fn fetch_max(&self, t: $inner) -> $inner {
                self.0.fetch_max(t)
            }

            #[allow(dead_code)]
            pub fn fetch_min(&self, t: $inner) -> $inner {
                self.0.fetch_min(t)
            }
        }
    );
}

impl_atomic_register_rw!(AtomicRegisterU32RW, u32);
impl_atomic_register_rw!(AtomicRegisterI32RW, i32);
impl_atomic_register_rw!(AtomicRegisterU64RW, u64);
impl_atomic_register_rw!(AtomicRegisterI64RW, i64);


// TODO/FIXME:
//   RISC-V cannot do atomic operations smaller than 32 bits
//   To be truly multiplatform, we shouldn't presume that register ops
//   less than 32 bits can't be atomic.  But for now, that's exactly
//   what we do.  In the future, each arch should set a flag to signal
//   what sizes can and cannot use AtomicPtr


macro_rules! impl_register_ro {
    ($typ:ident, $inner:ty) => (
        pub struct $typ(*mut $inner);

        impl $typ {
            #[allow(dead_code)]
            #[inline(always)]
            pub const unsafe fn new(addr: usize) -> $typ {
                $typ(addr as *mut $inner)
            }

            #[allow(dead_code)]
            pub fn fetch(&self) -> $inner {
                unsafe {
                    core::ptr::read_volatile(self.0)
                }
            }
        }
    );
}

impl_register_ro!(RegisterU16RO, u16);
impl_register_ro!(RegisterI16RO, i16);
impl_register_ro!(RegisterU8RO, u8);
impl_register_ro!(RegisterI8RO, i8);

macro_rules! impl_register_wo {
    ($typ:ident, $inner:ty) => (
        pub struct $typ(*mut $inner);

        impl $typ {
            #[allow(dead_code)]
            #[inline(always)]
            pub const unsafe fn new(addr: usize) -> $typ {
                $typ(addr as *mut $inner)
            }

            #[allow(dead_code)]
            pub fn store(&self, t: $inner) {
                unsafe {
                    core::ptr::write_volatile(self.0, t)
                }
            }
        }
    );
}

impl_register_wo!(RegisterU16WO, u16);
impl_register_wo!(RegisterI16WO, i16);
impl_register_wo!(RegisterU8WO, u8);
impl_register_wo!(RegisterI8WO, i8);

macro_rules! impl_register_rw {
    ($typ:ident, $inner:ty) => (
        pub struct $typ(*mut $inner);

        impl $typ {
            #[allow(dead_code)]
            #[inline(always)]
            pub const unsafe fn new(addr: usize) -> $typ {
                $typ(addr as *mut $inner)
            }

            #[allow(dead_code)]
            pub fn fetch(&self) -> $inner {
                unsafe {
                    core::ptr::read_volatile(self.0)
                }
            }

            #[allow(dead_code)]
            pub fn store(&self, t: $inner) {
                unsafe {
                    core::ptr::write_volatile(self.0, t)
                }
            }
        }
    );
}

impl_register_rw!(RegisterU16RW, u16);
impl_register_rw!(RegisterI16RW, i16);
impl_register_rw!(RegisterU8RW, u8);
impl_register_rw!(RegisterI8RW, i8);
