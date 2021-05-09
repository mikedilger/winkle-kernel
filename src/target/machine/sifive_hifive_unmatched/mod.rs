
global_asm!(include_str!("boot.S"));

use crate::device::uart::sifive::SifiveUart;

#[allow(dead_code)]
pub const UART0_ADDR: usize = 0x1001_0000;
pub static mut CONSOLE: SifiveUart = unsafe { SifiveUart::new(UART0_ADDR) };

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
