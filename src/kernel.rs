
#![no_std]
#![no_main]
#![feature(asm, llvm_asm, global_asm)]
#![feature(panic_info_message)]

mod atomic;
mod device;
mod register;
mod spinlock;
mod target;

use target::CONSOLE;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> !
{
    if let Some(msg) = info.message() {
        if let Some(m) = msg.as_str() {
            kdebug(m.as_bytes());
        }
    }
    crate::target::abort()
}


#[no_mangle]
extern "C" fn kernel_start() {
    // This initializes the kernel.

    kdebug(b"Testing\n");

    panic!("Not yet implemented.\n");
}

#[cfg(debug_assertions)]
#[inline]
fn kdebug(msg: &[u8]) {
    for c in b"KDEBUG: " {
        unsafe {
            (crate::target::UART0_ADDR as *mut u8)
                .write_volatile(*c);
        }
    }
    for c in msg {
        unsafe {
            (crate::target::UART0_ADDR as *mut u8)
                .write_volatile(*c);
        }
    }
}

#[cfg(not(debug_assertions))]
#[inline]
fn kdebug(msg: &[u8]) { }
