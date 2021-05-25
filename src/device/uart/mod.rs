
pub mod sifive;
pub mod uart16550;

use core::fmt::Write;

pub trait Uart: Write {
    fn put(&self, c: u8);
    fn get_maybe(&self) -> Option<u8>;
    fn set_line_settings(&self,
                         parity: UartParity,
                         data_bits: u8,
                         stop_bits: u8);
    fn set_baud_rate(&self, baud_hz: u32, uart_clock_hz: u32);
    fn get_baud_rate(&self, uart_clock_hz: u32) -> u32;
}

#[allow(dead_code)]
pub enum UartParity {
    None,
    Even,
    EvenSticky,
    Odd,
    OddSticky
}
