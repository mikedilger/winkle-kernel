
use core::sync::atomic;
use core::sync::atomic::Ordering;

#[allow(dead_code)]
#[inline(always)]
pub fn fence() {
    atomic::compiler_fence(Ordering::SeqCst);
    unsafe { asm!("fence"); }
}
