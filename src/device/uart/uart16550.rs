
use core::fmt::{Write, Error};
use crate::spinlock::Spinlock;
use crate::register::{RegisterU8RO, RegisterU8WO, RegisterU8RW, RegisterU16RW};
use crate::device::uart::Uart;
use bit_field::BitField;

pub struct Uart16550 {
    inner: Spinlock<InnerUart16550>
}

impl Uart16550 {
    #[allow(dead_code)]
    pub const unsafe fn new(base_address: usize) -> Self {
        Uart16550 {
            inner: Spinlock::new(InnerUart16550::new(base_address))
        }
    }
}

impl Uart for Uart16550 {
    fn put(&self, c: u8) {
        let inner_guard = self.inner.lock();
        inner_guard.put(c);
    }

    fn get_maybe(&self) -> Option<u8> {
        let inner_guard = self.inner.lock();
        inner_guard.get_maybe()
    }
}

impl Write for Uart16550 {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for c in s.bytes() {
            self.put(c);
        }

        Ok(())
    }
}

pub struct InnerUart16550 {
    rbr: RegisterU8RO,
    thr: RegisterU8WO,
    #[allow(dead_code)]
    ier: RegisterU8RW,
    #[allow(dead_code)]
    iir: RegisterU8RO,
    #[allow(dead_code)]
    fcr: RegisterU8WO,
    #[allow(dead_code)]
    lcr: RegisterU8RW,
    #[allow(dead_code)]
    mcr: RegisterU8RW,
    lsr: RegisterU8RO,
    #[allow(dead_code)]
    msr: RegisterU8RO,
    #[allow(dead_code)]
    dlr: RegisterU16RW
}

impl InnerUart16550 {
    #[allow(dead_code)]
    pub const fn new(base_address: usize) -> Self {
        unsafe {
            InnerUart16550 {
                rbr: RegisterU8RO::new(base_address),
                thr: RegisterU8WO::new(base_address),
                ier: RegisterU8RW::new(base_address + 0x1),
                iir: RegisterU8RO::new(base_address + 0x2),
                fcr: RegisterU8WO::new(base_address + 0x2),
                lcr: RegisterU8RW::new(base_address + 0x3),
                mcr: RegisterU8RW::new(base_address + 0x4),
                lsr: RegisterU8RO::new(base_address + 0x5),
                msr: RegisterU8RO::new(base_address + 0x6),
                dlr: RegisterU16RW::new(base_address),
            }
        }
    }

    pub fn put(&self, c: u8) {
        loop {
            if self.lsr.fetch().get_bit(5) { break; }
        }
        self.thr.store(c);
    }

    #[allow(dead_code)]
    pub fn get_maybe(&self) -> Option<u8> {
        if self.lsr.fetch().get_bit(0) {
            None
        } else {
            Some(self.rbr.fetch())
        }
    }
}

unsafe impl Sync for Uart16550 {}
unsafe impl Send for Uart16550 {}

