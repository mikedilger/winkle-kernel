
use core::fmt::{Write, Error};
use crate::spinlock::Spinlock;
use crate::register::{RegisterU8RO, RegisterU8WO, RegisterU8RW};
use crate::device::uart::{Uart, UartParity};
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

    fn set_line_settings(&self,
                         parity: UartParity,
                         data_bits: u8,
                         stop_bits: u8)
    {
        let inner_guard = self.inner.lock();
        inner_guard.set_line_settings(parity, data_bits, stop_bits);
    }

    fn set_baud_rate(&self, baud_hz: u32, uart_clock_hz: u32) {
        let inner_guard = self.inner.lock();
        inner_guard.set_baud_rate(baud_hz, uart_clock_hz);
    }

    fn get_baud_rate(&self, uart_clock_hz: u32) -> u32 {
        let inner_guard = self.inner.lock();
        inner_guard.get_baud_rate(uart_clock_hz)
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

struct InnerUart16550 {
    base_address: usize,
}

#[allow(dead_code)]
impl InnerUart16550 {
    pub const unsafe fn new(base_address: usize) -> Self {
        InnerUart16550 {
            base_address: base_address
        }
    }

    #[inline(always)]
    unsafe fn rbr(&self) -> RegisterU8RO {
        RegisterU8RO::new(self.base_address)
    }

    #[inline(always)]
    unsafe fn thr(&self) -> RegisterU8WO {
        RegisterU8WO::new(self.base_address)
    }

    #[inline(always)]
    unsafe fn ier(&self) -> RegisterU8RW {
        RegisterU8RW::new(self.base_address + 0x1)
    }

    #[inline(always)]
    unsafe fn iir(&self) -> RegisterU8RO {
        RegisterU8RO::new(self.base_address + 0x2)
    }

    #[inline(always)]
    unsafe fn fcr(&self) -> RegisterU8WO {
        RegisterU8WO::new(self.base_address + 0x2)
    }

    #[inline(always)]
    unsafe fn lcr(&self) -> RegisterU8RW {
        RegisterU8RW::new(self.base_address + 0x3)
    }

    #[inline(always)]
    unsafe fn mcr(&self) -> RegisterU8RW {
        RegisterU8RW::new(self.base_address + 0x4)
    }

    #[inline(always)]
    unsafe fn lsr(&self) -> RegisterU8RO {
        RegisterU8RO::new(self.base_address + 0x5)
    }

    #[inline(always)]
    unsafe fn msr(&self) -> RegisterU8RO {
        RegisterU8RO::new(self.base_address + 0x6)
    }

    #[inline(always)]
    unsafe fn dlr_lsb(&self) -> RegisterU8RW {
        RegisterU8RW::new(self.base_address)
    }

    #[inline(always)]
    unsafe fn dlr_msb(&self) -> RegisterU8RW {
        RegisterU8RW::new(self.base_address + 0x01)
    }

    pub fn put(&self, c: u8) {
        loop {
            if unsafe { self.lsr() }.fetch().get_bit(5) { break; }
        }
        unsafe { self.thr() }.store(c);
    }

    pub fn get_maybe(&self) -> Option<u8> {
        if unsafe { self.lsr() }.fetch().get_bit(0) {
            None
        } else {
            Some(unsafe { self.rbr() }.fetch())
        }
    }

    pub fn set_line_settings(&self, parity: UartParity,
                             mut data_bits: u8,
                             mut stop_bits: u8)
    {
        // Bound values to supported ranges
        if data_bits<5 { data_bits = 5; }
        else if data_bits>8 { data_bits = 8; }
        if stop_bits < 1 { stop_bits = 1; }
        else if stop_bits > 2 { stop_bits = 2; }

        let mut lcr = data_bits;
        if stop_bits==2 { lcr.set_bit(2, true); }
        match parity {
            UartParity::None => lcr.set_bits(3..=5, 0b000),
            UartParity::Even => lcr.set_bits(3..=5, 0b011),
            UartParity::EvenSticky => lcr.set_bits(3..=5, 0b111),
            UartParity::Odd => lcr.set_bits(3..=5, 0b001),
            UartParity::OddSticky => lcr.set_bits(3..=5, 0b101),
        };

        unsafe { self.lcr() }.store(lcr);
    }

    pub fn set_baud_rate(&self, baud_hz: u32, uart_clock_hz: u32)
    {
        // Compute the divisor
        let divisor: u16 = (uart_clock_hz / (baud_hz * 16)) as u16;
        let divisor_lsb: u8 = (divisor & 0x00FF) as u8;
        let divisor_msb: u8 = ((divisor & 0xFF00) >> 8) as u8;

        // Turn on the divisor latch
        let mut lcr = unsafe { self.lcr() }.fetch();
        lcr.set_bit(7, true);
        unsafe { self.lcr() }.store(lcr);

        // Write the divisor
        unsafe { self.dlr_msb() }.store(divisor_msb);
        unsafe { self.dlr_lsb() } .store(divisor_lsb);

        // Turn off the divisor latch
        lcr.set_bit(7, false);
        unsafe { self.lcr() }.store(lcr);
    }

    pub fn get_baud_rate(&self, uart_clock_hz: u32) -> u32
    {
        // Turn on the divisor latch
        let mut lcr = unsafe { self.lcr() }.fetch();
        lcr.set_bit(7, true);
        unsafe { self.lcr() }.store(lcr);

        // Read the divisor
        let msb = unsafe { self.dlr_msb() }.fetch();
        let lsb = unsafe { self.dlr_lsb() }.fetch();

        // Turn off the divisor latch
        lcr.set_bit(7, false);
        unsafe { self.lcr() }.store(lcr);

        // Compute the divisor
        let divisor = ((msb as u32) << 8) | (lsb as u32);

        (uart_clock_hz / divisor) / 16
    }
}

unsafe impl Sync for Uart16550 {}
unsafe impl Send for Uart16550 {}
