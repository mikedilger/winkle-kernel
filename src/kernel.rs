
#![no_std]
#![no_main]

mod target;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> !
{
    loop { }
}
