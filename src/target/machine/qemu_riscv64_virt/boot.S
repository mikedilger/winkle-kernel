// Winkle First-Stage/Berkeley Boot Loader for SiFive FU740-C000 based systems (and similar)

	/* Disable generation of compressed instructions */
.option norvc

.section .text.init
.global _start
_start:
        /* All harts will be running this code in parallel */

	/* It is not valid to obtain the address of any symbol if the GP is not configured */
        /* So the first thing we must do is set the gp.  And we have to disable linker */
        /* relaxation to get this right because _global_pointer itself is a symbol! */
.option push
.option norelax
	la		gp, _global_pointer
.option pop

	/* Configure our machine trap beyond the next instructions. If they cause */
	/* a trap, we just continue from beyond them. '1' must be align 4 like all */
        /* trap vectors and because the bottom two bits of mtvec are interrupt processing
        /* mode which must be direct (00) */
	la              t0, 1f
	csrw            mtvec, t0

        /* Disable all address translation and protection */
        /* Set our SATP (address translation) to Bare mode (on all harts) */
        csrw            satp, zero

.align 4
1:

	/* Move the trap vector to a new place */
	la              t0, early_trap_vector
	csrw            mtvec, t0

        /* Set the stack pointers (all harts) */
        csrr            a0, mhartid
        la              sp, _stacks_end
        la              t0, _stack_size
        mul             t0, t0, a0
        sub             sp, sp, t0

        /* If not mhartid 0, go idle */
        bnez            a0, idle

        /* Most OSes would clear the BSS. Rust doesn't presume and zeros memory. */
        /* We hope. So this is commented out. */
	/* la 		a0, _bss_start */
	/* la		a1, _bss_end */
	/* bgeu	        a0, a1, jump_into_rust */
/* bss_zeroing_loop: */
        /* sd              zero, (a0) */
        /* addi            a0, a0, 8 */
        /* bltu            a0, a1, bss_zeroing_loop */

jump_into_rust:
        /* Jump into rust: */

        /* NOTE: mstatus on reset has these values: MIE=0, MPRV=0, MBE=0 */

        /* We want to MRET, but stay in machine mode.  So we set MPP=M(11) */
        li              t0, 0x00000C00 /* set MPP <- 0b11 */
        csrrs           zero, mstatus, t0

        /* Set the machine exception PC to our rust program.
           When we return, this will be used to continue execution at that point. */
        la              t1, kernel_start
        csrw            mepc, t1

        /* Set the return address so that kernel_start can return control to us */
        la              ra, 3f

        /* Jump into rust */
        mret

3:
        /* Presently we just idle after rust. */

idle:
        wfi
        j idle


.global early_trap_vector
.align 2
early_trap_vector:
	/* Make these values register-available so we can see in a debugger why */
	/* it trapped */
        /*
.cfi_startproc
	csrr t0, mcause
	csrr t1, mepc
	csrr t2, mtval
	j early_trap_vector
.cfi_endproc
        */

        /* Nah, just return (ignoring any traps/interrupts) */
        mret
