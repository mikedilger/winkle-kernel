
// FIXME
global_asm!(include_str!("boot.S"));

use crate::device::uart::uart16550::Uart16550;

#[allow(dead_code)]
pub const UART0_ADDR: usize = 0x1000_0000;
pub static mut CONSOLE: Uart16550 = unsafe { Uart16550::new(UART0_ADDR) };

#[inline(always)]
pub fn pause() {
    unsafe {
        // LLVM does not support (yet?) Zihintpause feature.  Once it does,
        // we may need a new target json file that enables this processor feature
        // e.g. "features": "+64bit,+m,+a,+c,+zihintpause",
	// llvm_asm!("pause"::::"volatile");

        // Until then, we just issue a FENCE
        asm!("fence");
    }
}

pub fn init() {
}

pub fn display_machine_information() {
    println!("Build: QEMU virt (riscv64)");
}
