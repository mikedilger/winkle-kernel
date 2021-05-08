
global_asm!(include_str!("boot.S"));

use crate::device::uart::sifive::SifiveUart;

#[allow(dead_code)]
pub const UART0_ADDR: usize = 0x1001_0000;
pub static mut CONSOLE: SifiveUart = unsafe { SifiveUart::new(UART0_ADDR) };
