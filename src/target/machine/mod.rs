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

#[cfg(machine = "microchip-polarfire-icicle")]
mod microchip_polarfire_icicle;
#[cfg(machine = "microchip-polarfire-icicle")]
pub use microchip_polarfire_icicle::*;

#[cfg(machine = "qemu-microchip-polarfire-icicle")]
mod qemu_microchip_polarfire_icicle;
#[cfg(machine = "qemu-microchip-polarfire-icicle")]
pub use qemu_microchip_polarfire_icicle::*;

#[cfg(machine = "")]
compile_error!("Winkle requires a specific machine to be defined");

#[cfg(all(
    not(machine = "qemu-riscv64-virt"),
    not(machine = "sifive-hifive-unleashed"),
    not(machine = "sifive-hifive-unmatched"),
    not(machine = "microchip-polarfire-icicle"),
    not(machine = "qemu-microchip-polarfire-icicle")
))]
compile_error!("Winkle does not support the specified machine");

/*
 * Each machine needs to define the following:
 *
 *   The label "_start" where execution begins
 *   fn init() for initializing the hardware
 *   fn display_machine_information() for logging info about the hardware
 *   fn pause() for spinlocks
 *   const UART0_ADDR: usize
 *   static CONSOLE: T
 *       where T: Uart
 *       and has const new fn
 */

