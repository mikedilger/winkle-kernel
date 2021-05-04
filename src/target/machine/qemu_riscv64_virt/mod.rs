
// FIXME
global_asm!(include_str!("../sifive_hifive_unmatched/boot.S"));

#[allow(dead_code)]
pub const UART0_ADDR: usize = 0x1000_0000;
