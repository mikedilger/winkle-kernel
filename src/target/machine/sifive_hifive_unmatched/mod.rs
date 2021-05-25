
global_asm!(include_str!("boot.S"));

use crate::device::uart::Uart;
use crate::device::uart::sifive::SifiveUart;

mod clock;

pub const UART0_ADDR: usize = 0x1001_0000;
#[allow(dead_code)]
pub const UART1_ADDR: usize = 0x1001_1000;
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

pub fn display_machine_information() {
    println!("Build: SiFive HiFive Unmatched");

    println!("Clock Info:");
    let corefreq = clock::get_core_frequency();
    println!("  Core frequency = {} Hz", corefreq);
    println!("  PLL cltx: {}", if clock::prci_plls::get_cltxpll() { "present" } else { "absent" });
    println!("  PLL gemgxl: {}", if clock::prci_plls::get_gemgxlpll() { "present" } else { "absent" });
    println!("  PLL ddr: {}", if clock::prci_plls::get_ddrpll() { "present" } else { "absent" });
    println!("  PLL hfpclk: {}", if clock::prci_plls::get_hfpclkpll() { "present" } else { "absent" });
    println!("  PLL dvfscore: {}", if clock::prci_plls::get_dvfscorepll() { "present" } else { "absent" });
    println!("  PLL core: {}", if clock::prci_plls::get_corepll() { "present" } else { "absent" });
    let tlclk = clock::get_tlclk();
    println!("  tlclk: {} Hz", tlclk);
    println!("  UART baud: {}", CONSOLE.get_baud_rate(tlclk as u32));
}
