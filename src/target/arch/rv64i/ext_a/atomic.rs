
use crate::atomic::Atomic;
use core::marker::Sync;
use core::sync::atomic::Ordering;

/// An AtomicPtr is just a memory address that is accessed with atomic operations.
/// Unlike core::sync::Atomic types, this can be used with unowned data (e.g. hardware registers)
/// as well as owned data.
pub struct AtomicPtr<T> {
    ptr: *mut T
}

macro_rules! impl_atomic_ptr {
    // $w should be 'w' for 32 bit or 'd' for 64 bit
    // $u shuld be 'u' for unsigned or '' for signed
    ($typ:ty, $w:expr, $u:expr) => (

        impl AtomicPtr<$typ> {
            #[allow(dead_code)]
            #[inline(always)]
            pub const unsafe fn new(ptr: *mut $typ) -> AtomicPtr<$typ> {
                AtomicPtr {
                    ptr: ptr
                }
            }

            #[allow(dead_code)]
            #[inline(always)]
            pub const fn new_address(addr: usize) -> AtomicPtr<$typ> {
                AtomicPtr {
                    ptr: addr as *mut $typ
                }
            }
        }

        impl Atomic for AtomicPtr<$typ> {
            type T = $typ;

            #[inline(always)]
            unsafe fn as_mut_ptr(&self) -> *mut Self::T {
                self.ptr
            }

            #[inline(always)]
            fn store(&self, t: Self::T) {
                unsafe {
                    // There is no need to be atomic here, it's just a store.
                    // llvm_asm!(concat!("amoswap.",$w," zero, $0, ($1)") :: "r"(t), "r"(self.ptr) :: "volatile");
                    *self.ptr = t;
                }
            }

            #[inline(always)]
            fn store_acq(&self, t: Self::T) {
                unsafe {
                    llvm_asm!(concat!("amoswap.",$w,".aq zero, $0, ($1)") :: "r"(t), "r"(self.ptr) :: "volatile");
                }
                core::sync::atomic::compiler_fence(Ordering::Acquire);
            }

            #[inline(always)]
            fn store_rel(&self, t: Self::T) {
                core::sync::atomic::compiler_fence(Ordering::Release);
                unsafe {
                    llvm_asm!(concat!("amoswap.",$w,".rl zero, $0, ($1)") :: "r"(t), "r"(self.ptr) :: "volatile");
                }
            }

            #[inline(always)]
            fn store_seqcst(&self, t: Self::T) {
                core::sync::atomic::compiler_fence(Ordering::Release);
                unsafe {
                    llvm_asm!(concat!("amoswap.",$w,".aqrl zero, $0, ($1)") :: "r"(t), "r"(self.ptr) :: "volatile");
                }
                core::sync::atomic::compiler_fence(Ordering::Acquire);
            }

            #[inline(always)]
            fn fetch(&self) -> Self::T {
                unsafe {
                    // There is no need to be atomic here, it's just a load
                    // llvm_asm!(concat!("lr.",$w,".aq $0, ($1)") : "=r"(output) : "r"(self.ptr) :: "volatile");
                    *self.ptr
                }
            }

            #[inline(always)]
            fn fetch_seqcst(&self) -> Self::T {
                core::sync::atomic::compiler_fence(Ordering::Release);
                let mut output: Self::T;
                unsafe {
                    llvm_asm!(concat!("lr.",$w,".aqrl $0, ($1)") : "=r"(output) : "r"(self.ptr) :: "volatile");
                }
                core::sync::atomic::compiler_fence(Ordering::Acquire);
                output
            }

            #[inline(always)]
            fn swap(&self, t: Self::T) -> Self::T {
                let mut output: Self::T;
                unsafe {
                    llvm_asm!(concat!("amoswap.",$w," $0, $1, ($2)") : "=r"(output) : "r"(t), "r"(self.ptr) :: "volatile");
                }
                output
            }

            #[inline(always)]
            fn swap_seqcst(&self, t: Self::T) -> Self::T {
                core::sync::atomic::compiler_fence(Ordering::Release);
                let mut output: Self::T;
                unsafe {
                    llvm_asm!(concat!("amoswap.",$w,".aqrl $0, $1, ($2)") : "=r"(output) : "r"(t), "r"(self.ptr) :: "volatile");
                }
                core::sync::atomic::compiler_fence(Ordering::Acquire);
                output
            }

            #[inline(always)]
            fn compare_and_swap(&self, compare_to: Self::T, t: Self::T) -> Self::T {
                core::sync::atomic::compiler_fence(Ordering::Release);
                let mut output: Self::T;
                unsafe {
                    // Note: risc-v guarantees eventual success and forward progress
                    // (avoiding livelock) as this sequence meets the constraints for
                    // said guarantee
                    llvm_asm!(concat!("1:
                       lr.",$w,".aqrl $0, ($1)
                       bne $0, $2, 2f
                       sc.",$w,".aqrl t0, $3, ($1)
                       bnez t0, 1b
                       2: ")
                              : "=&r"(output)                      // $0=output
                              : "r"(self.ptr), "r"(compare_to), "r"(t) // $1=cell, $2=compare_to, $3=t
                              : "t0"
                              : "volatile");
                }
                core::sync::atomic::compiler_fence(Ordering::Acquire);
                output
            }

            #[inline(always)]
            fn fetch_add(&self, t: Self::T) -> Self::T {
                let mut output: Self::T;
                unsafe {
                    llvm_asm!(concat!("amoadd.",$w," $0, $1, ($2)") : "=r"(output) : "r"(t), "r"(self.ptr) :: "volatile");
                }
                output
            }

            #[inline(always)]
            fn fetch_sub(&self, t: Self::T) -> Self::T {
                let mut output: Self::T;
                unsafe {
                    llvm_asm!(concat!("amosub.",$w," $0, $1, ($2)") : "=r"(output) : "r"(t), "r"(self.ptr) :: "volatile");
                }
                output
            }

            #[inline(always)]
            fn fetch_and(&self, t: Self::T) -> Self::T {
                let mut output: Self::T;
                unsafe {
                    llvm_asm!(concat!("amoand.",$w," $0, $1, ($2)") : "=r"(output) : "r"(t), "r"(self.ptr) :: "volatile");
                }
                output
            }

            #[inline(always)]
            fn fetch_or(&self, t: Self::T) -> Self::T {
                let mut output: Self::T;
                unsafe {
                    llvm_asm!(concat!("amoor.",$w," $0, $1, ($2)") : "=r"(output) : "r"(t), "r"(self.ptr) :: "volatile");
                }
                output
            }

            #[inline(always)]
            fn fetch_xor(&self, t: Self::T) -> Self::T {
                let mut output: Self::T;
                unsafe {
                    llvm_asm!(concat!("amoxor.",$w," $0, $1, ($2)") : "=r"(output) : "r"(t), "r"(self.ptr) :: "volatile");
                }
                output
            }

            #[inline(always)]
            fn fetch_max(&self, t: Self::T) -> Self::T {
                let mut output: Self::T;
                unsafe {
                    llvm_asm!(concat!("amomax",$u,".",$w," $0, $1, ($2)") : "=r"(output) : "r"(t), "r"(self.ptr) :: "volatile");
                }
                output
            }

            #[inline(always)]
            fn fetch_min(&self, t: Self::T) -> Self::T {
                let mut output: Self::T;
                unsafe {
                    llvm_asm!(concat!("amomin",$u,".",$w," $0, $1, ($2)") : "=r"(output) : "r"(t), "r"(self.ptr) :: "volatile");
                }
                output
            }
        }

        #[allow(dead_code)]
        impl AtomicPtr<$typ> {
            /// This function ANDs the value with `and` and then ORs it with `or`, but atomically
            /// This is useful for setting a subset of bits without disturbing other bits.
            #[inline(always)]
            pub fn fetch_and_or(&self, and: $typ, or: $typ) -> $typ {
                let mut output: $typ;
                unsafe {
                    // Note: risc-v guarantees eventual success and forward progress
                    // (avoiding livelock) as this sequence meets the constraints for
                    // said guarantee
                    llvm_asm!(concat!("1:
                       lr.",$w,".aqrl $0, ($1)
                       andi $0, $0, $2
                       ori $0, $0, $3
                       sc.",$w,".aqrl t0, $0, ($1)
                       bnez t0, 1b")
                              : "=&r"(output) // $0 is output, = means write, & means clobbered before all inputs used, r means register
                              : "r"(self.ptr), "r"(and), "r"(or) // $1=pointer to mem, $2=AND, $3=OR
                              : "t0"
                              : "volatile");
                }
                output
            }
        }

        unsafe impl Send for AtomicPtr<$typ> {}
        unsafe impl Sync for AtomicPtr<$typ> {}
    );
}

impl_atomic_ptr!(isize, "w", "");
impl_atomic_ptr!(usize, "w", "u");
impl_atomic_ptr!(i32, "w", "");
impl_atomic_ptr!(u32, "w", "u");
impl_atomic_ptr!(i64, "d", "");
impl_atomic_ptr!(u64, "d", "u");
