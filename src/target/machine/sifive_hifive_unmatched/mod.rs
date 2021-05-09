
global_asm!(include_str!("boot.S"));

use crate::device::uart::sifive::SifiveUart;

#[allow(dead_code)]
pub const UART0_ADDR: usize = 0x1001_0000;
pub static mut CONSOLE: SifiveUart = unsafe { SifiveUart::new(UART0_ADDR) };

#[inline(always)]
pub fn pause() {
    unsafe {
        // PAUSE instruction (not yet in llvm backend)
        llvm_asm!(".word 0x0100000F" : : : : "volatile");
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn cease() {
    unsafe {
        llvm_asm!(".word 0x30500073" : : : "memory" : "volatile");
    }
}
