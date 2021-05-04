
#[cfg(all(target_arch="riscv64"))]
pub mod rv64i;
#[cfg(all(target_arch="riscv64"))]
pub use rv64i::*;

#[cfg(all(not(target_arch="riscv64")))]
compile_error!("Winkle does not support the specified target architecture.");

/*
 * Each architecture needs to define the following:
 *
 *   pub extern "C" fn abort() -> !
 *
 *   AtomicPtr for i32, u32, i64, u64, isize and usize
 *      fn new(usize)
 *      impl AtomicCell
 *
 *   pub fn fence()
 */
