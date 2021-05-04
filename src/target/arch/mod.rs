
#[cfg(all(target_arch="riscv64"))]
pub mod rv64i;
#[cfg(all(target_arch="riscv64"))]
pub use rv64i::*;

#[cfg(all(not(target_arch="riscv64")))]
compile_error!("Winkle does not support the specified target architecture.");
