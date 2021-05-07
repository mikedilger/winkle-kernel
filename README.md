# winkle-kernel
Winkle will be an experimental embedded operating system for RISC-V machines, written in Rust, structured as a non-minimal microkernel, with real-time functionality.

For quite a while nothing groundbreaking will be happening here. Once I get over the hump I intend to play around with a few things:

* Using L1 cache as tightly-integrated memory, and if pages in that can be allocated into userspace, using that for message passing (if not, using it for kernel structures)
* Using lock-free data structures in shared memory for async message passing, along the same lines as linux io_uring does between userspace and the kernel.
* Capability based access control

WARNING:  I will be force-pushing to this repository rewriting history from time to time while it is early days. At some point I will stop doing this, and when I do, I will remove this warning from the README file.


## Installation

See [INSTALL](https://github.com/mikedilger/winkle-kernel/blob/main/INSTALL.md)

## Scope and Features

### Scope
This operating system is intended to be general purpose in nature.

This operating system is intended to support real-time processes along side non-real-time processes.

This operating system is aimed at the embedded space, but not with any intent to exclude other machines. It's just that I wont be spending my time writing drivers for trackpads and GPUs and such.

This operating system is aimed at RISC-V hardware, but not with any intent to exclude other architectures. It's just that I will be designing around and writing code for that ISA and those machines.

This operating system is written in Rust, but not with any intent to exclude other languages. It's just that I won't be writing much in other languages.

This operating system will be structured as a microkernel, but won't necessarily adhere to strict minimality. Whether something ends up in the microkernel or not will be judged on the merits.

This operating system will be using capability based access control.

This operating system will be isolating namespaces from an early point, with a view towards secure containers.

POSIX compatibility is a practical necessity, but I won't let it constrain the possibilities for the microkernel itself; rather, POSIX compatibility (and libc) will likely be a shim.

### Completed Features
This will be extended as features are developed.

## The Name
A winkle (or periwinkle) is a sea snail. Snails are slow, so this is a modest name for an operating system aiming to be fast. Winkle is (was?) also a slang verb in Britain for something that requires a lot of effort to achieve, such as this operating system.

## License
MIT.  See [LICENSE](https://github.com/mikedilger/winkle-kernel/blob/main/LICENSE)

## Contribution
Any contribution intentionally submitted for inclusion in the work by you shall be bound by the terms of the MIT license.
