
pub mod sifive;
pub mod uart16550;

use core::fmt::Write;

pub trait Uart: Write {
    fn put(&self, c: u8);
    fn get_maybe(&self) -> Option<u8>;
}
