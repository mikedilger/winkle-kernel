
use core::fmt::{Write, Error};
use bit_field::BitField;
use crate::register::{AtomicRegisterI32RW, AtomicRegisterI32RO,
                      AtomicRegisterU32RW, AtomicRegisterU32RO};
use crate::device::uart::Uart;

/* This is the SiFive style UART */
pub struct SifiveUart {
    base_address: usize
}

#[allow(dead_code)] // this device may not be chosen for the target machine
impl SifiveUart {
    #[inline(always)]
    pub const unsafe fn new(base_address: usize) -> Self {
        SifiveUart {
            base_address: base_address
        }
    }

    #[inline(always)]
    unsafe fn txdata(&self) -> AtomicRegisterI32RW {
        AtomicRegisterI32RW::new(self.base_address)
    }

    #[inline(always)]
    unsafe fn rxdata(&self) -> AtomicRegisterI32RO {
        AtomicRegisterI32RO::new(self.base_address + 0x04)
    }

    #[inline(always)]
    unsafe fn txctrl(&self) -> AtomicRegisterU32RW {
        AtomicRegisterU32RW::new(self.base_address + 0x08)
    }

    #[inline(always)]
    unsafe fn rxctrl(&self) -> AtomicRegisterU32RW {
        AtomicRegisterU32RW::new(self.base_address + 0x0C)
    }

    #[inline(always)]
    unsafe fn ie(&self) -> AtomicRegisterU32RW {
        AtomicRegisterU32RW::new(self.base_address + 0x10)
    }

    #[inline(always)]
    unsafe fn ip(&self) -> AtomicRegisterU32RO {
        AtomicRegisterU32RO::new(self.base_address + 0x14)
    }

    #[inline(always)]
    unsafe fn div(&self) -> AtomicRegisterU32RW {
        AtomicRegisterU32RW::new(self.base_address + 0x18)
    }
}

impl Uart for SifiveUart {
    fn put(&self, c: u8) {
        loop {
            let v = unsafe { self.txdata() }.fetch_or(c as i32);
            if v != 0 { return; }
        }
    }

    fn get_maybe(&self) -> Option<u8> {
        let v =  unsafe { self.rxdata() }.fetch();
        if v < 0 {
            return None
        } else {
            return Some(v.get_bits(0..=7) as u8)
        }
    }
}

impl Write for SifiveUart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for c in s.bytes() {
            self.put(c);
        }

        Ok(())
    }
}
