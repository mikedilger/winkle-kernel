
#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {
	unsafe {
	    llvm_asm!("wfi"::::"volatile"); // Wait for interrupt
	}
    }
}
