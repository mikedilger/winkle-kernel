
use core::fmt::Write;

pub trait Uart: Write {
    fn put(&self, c: u8);
    fn get_maybe(&self) -> Option<u8>;
}
