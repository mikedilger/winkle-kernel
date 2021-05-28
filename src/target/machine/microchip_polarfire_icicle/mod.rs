
// FIXME
global_asm!(include_str!("../sifive_hifive_unmatched/boot.S"));

compile_error!("Microchip PolarFire SoC Icicle Kit will be supported soon, but is not yet.");

#[allow(dead_code)]
pub const UART0_ADDR: usize = 0x2000_0000;
// Missing CONSOLE

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

pub fn init() {
}

pub fn display_machine_information() {
    println!("Build: Microchip PolarFire SoC Icicle Kit");
}
