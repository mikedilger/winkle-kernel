
#![no_std]
#![no_main]
#![feature(asm, llvm_asm, global_asm)]
#![feature(panic_info_message)]

include!("macros.rs");

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

    // Initialize the hardware
    target::init();

    // Initialize the CONSOLE
    use device::uart::{Uart, UartParity};
    unsafe { CONSOLE.set_line_settings(UartParity::None, 8, 1) };
    // For now we leave the baud rate as the default, or whatever target::init()
    // selects.

    // Print machine-level information
    target::display_machine_information();

    // Print a few more things and finish up, as we don't have a useable
    // operating system yet.
    println!("Hello World!\n");

    panic!("Cannot Continue - Operating System is not yet implemented.\n");
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
