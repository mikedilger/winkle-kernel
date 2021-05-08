
// FIXME
global_asm!(include_str!("../sifive_hifive_unmatched/boot.S"));

use crate::device::uart::uart16550::Uart16550;

#[allow(dead_code)]
pub const UART0_ADDR: usize = 0x1000_0000;
pub static mut CONSOLE: Uart16550 = unsafe { Uart16550::new(UART0_ADDR) };
