
// This UART is documented in fu740-c000-manual-v1p2.pdf

use core::fmt::{Write, Error};
use bit_field::BitField;
use crate::register::{AtomicRegisterI32RW, AtomicRegisterI32RO,
                      AtomicRegisterU32RW, AtomicRegisterU32RO};
use crate::device::uart::{Uart, UartParity};

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

    // data_bits is ignored, device only supports 8.
    // parity is ignored, device only supports no parity.
    fn set_line_settings(&self,
                         _parity: UartParity,
                         _data_bits: u8,
                         mut stop_bits: u8)
    {
        if stop_bits<1 { stop_bits=1; }
        else if stop_bits>2 { stop_bits=2; }

        if stop_bits==2 {
            // set bit 1
            unsafe { self.txctrl() }.fetch_or( 0b10_u32 );
        } else {
            // clear bit 1
            unsafe { self.txctrl() }.fetch_and( ! 0b10_u32 );
        }
    }

    fn set_baud_rate(&self, baud_hz: u32, uart_clock_hz: u32) {
        let mut divisor = uart_clock_hz / baud_hz - 1;

        // minimum due to recv oversampling requirement
        if divisor < 16 { divisor = 16; }

        // Bits 16..31 are reserved. We can't atomically set the lower 16
        // bits while preserving the upper 16.  So we read the register,
        // and use compare_and_swap to write the lower 16 only if the upper
        // 16 haven't chnaged out from under us.  If they have, we loop
        // around and try again.
        loop {
            let divreg = unsafe { self.div() }.fetch();
            let newdivreg = (divreg & 0xFFFF_0000) + (divisor & 0x0000_FFFF);
            let cas_result = unsafe { self.div() }.compare_and_swap(divreg, newdivreg);
            if cas_result == divreg { break; }
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
