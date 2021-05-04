
// Extension specific code
#[cfg(target_feature = "a")]
mod ext_a;
#[cfg(target_feature = "a")]
pub use ext_a::*;
#[cfg(not(target_feature = "a"))]
compile_error!("rv64i is only currently supported if the Atomic extension is available.");

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {
	unsafe {
	    llvm_asm!("wfi"::::"volatile"); // Wait for interrupt
	}
    }
}

mod ordering;
pub use ordering::*;
