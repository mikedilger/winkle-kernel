
use core::fmt::{Write, Error};
use bit_field::BitField;
use crate::register::{AtomicRegisterI32RW, AtomicRegisterI32RO,
                      AtomicRegisterU32RW, AtomicRegisterU32RO};
use crate::device::uart::Uart;

/* This is the SiFive style UART */
pub struct SifiveUart {
    txdata: AtomicRegisterI32RW,
    rxdata: AtomicRegisterI32RO,
    #[allow(dead_code)]
    txctrl: AtomicRegisterU32RW,
    #[allow(dead_code)]
    rxctrl: AtomicRegisterU32RW,
    #[allow(dead_code)]
    ie: AtomicRegisterU32RW,
    #[allow(dead_code)]
    ip: AtomicRegisterU32RO,
    #[allow(dead_code)]
    div: AtomicRegisterU32RW,
}

#[allow(dead_code)] // this device may not be chosen for the target machine
impl SifiveUart {
    pub const unsafe fn new(base_address: usize) -> Self {
        SifiveUart {
            txdata: AtomicRegisterI32RW::new(base_address),
            rxdata: AtomicRegisterI32RO::new(base_address + 0x04),
            txctrl: AtomicRegisterU32RW::new(base_address + 0x08),
            rxctrl: AtomicRegisterU32RW::new(base_address + 0x0C),
            ie: AtomicRegisterU32RW::new(base_address + 0x10),
            ip: AtomicRegisterU32RO::new(base_address + 0x14),
            div: AtomicRegisterU32RW::new(base_address + 0x18),
        }
    }
}

impl Uart for SifiveUart {
    fn put(&self, c: u8) {
        loop {
            let v = self.txdata.fetch_or(c as i32);
            if v != 0 { return; }
        }
    }

    fn get_maybe(&self) -> Option<u8> {
        let v = self.rxdata.fetch();
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
