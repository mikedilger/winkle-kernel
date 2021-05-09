#[cfg(machine = "qemu-riscv64-virt")]
mod qemu_riscv64_virt;
#[cfg(machine = "qemu-riscv64-virt")]
pub use qemu_riscv64_virt::*;

#[cfg(machine = "sifive-hifive-unleashed")]
mod sifive_hifive_unleashed;
#[cfg(machine = "sifive-hifive-unleashed")]
pub use sifive_hifive_unleashed::*;

#[cfg(machine = "sifive-hifive-unmatched")]
mod sifive_hifive_unmatched;
#[cfg(machine = "sifive-hifive-unmatched")]
pub use sifive_hifive_unmatched::*;

#[cfg(machine = "polarfire-icicle-kit")]
mod polarfire_icicle_kit;
#[cfg(machine = "polarfire-icicle-kit")]
pub use polarfire_icicle_kit::*;

#[cfg(machine = "")]
compile_error!("Winkle requires a specific machine to be defined");

#[cfg(all(
    not(machine = "qemu-riscv64-virt"),
    not(machine = "sifive-hifive-unleashed"),
    not(machine = "sifive-hifive-unmatched"),
    not(machine = "polarfire-icicle-kit")
))]
compile_error!("Winkle does not support the specified machine");

/*
 * Each machine needs to define the following:
 *
 *   The label "_start" where execution begins
 *   const UART0_ADDR: usize
 *   static CONSOLE: T
 *       where T: Uart
 *       and has const new fn
 *   fn pause()
 */

