
#![no_std]
#![no_main]
#![feature(asm, llvm_asm, global_asm)]

mod target;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> !
{
    crate::target::abort()
}

#[no_mangle]
extern "C" fn kernel_start() {
    // This initializes the kernel.

    panic!("Not yet implemented.\n");
}
