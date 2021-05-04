
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

#[inline(always)]
#[allow(dead_code)]
#[allow(unused_assignments)]
pub fn cpu_number() -> u32 {
    let mut hart_id: u32 = 0;
    unsafe { llvm_asm!("csrr $0, mhartid" : "=r"(hart_id) ::: "volatile"); }
    hart_id
}
