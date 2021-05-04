
global_asm!(include_str!("boot.S"));

#[allow(dead_code)]
pub const UART0_ADDR: usize = 0x1001_0000;
