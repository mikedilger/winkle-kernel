# Install

## Hardware Requirements
Initially you'll need one of the following machines, as this is all that will be supported for the near future:

* QEMU (for software simulation only)
* SiFive HiFive Unleashed
* SiFive HiFive Unmatched
* PolarFire SoC Icicle Kit

For other boards to be supported, they will need the following:

* RISC-V rv64ima with all three privilege modes (machine, supervisor and user)

## QEMU
We need an emulator to run the OS on while developing, even if we have the hardware on hand.  We use QEMU for this.  Install QEMU for the qemu-system-riscv64 target according to your operating system's method.

## Rust
If rust is not installed in your home directory via 'rustup', then get it from https://rustup.rs/ and install it.

If rustup is installed, update it:

````sh
    $ rustup self update
    $ rustup update
````
In this directory run:

````sh
    $ rustup override set nightly
````
Add the machine target you are building for. Currently only one is supported:

````sh
    $ rustup target add riscv64imac-unknown-none-elf
````
Note that this target will not constrain the final target that programs running on
the operating system are built for.

Install the cargo binutils:

````sh
    $ cargo install cargo-binutils
````
## Choose a machine
In the `machines/` directory are a set of environment settings for building for various machines. Source one of them in your active shell(s). E.g.:

````sh
    $ source ./machines/riscv64imac-qemu.env
````

