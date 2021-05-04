/* Winkle linker script for:   SiFive HiFive Unmatched */
/* This is a conglomeration of stuff taken from many sources including:
     freedom-e-sdk/bsp/qemu-sifive-u54mc/metal.default.lds
 */

OUTPUT_ARCH( "riscv" )
OUTPUT_FORMAT( "elf64-littleriscv" )

/* Set the entry point (this is where execution begins, see boot.S) */
ENTRY( _start )

MEMORY
{
        /* Any section which is not listed can be stored here, and that applies to
           read-only, read-write, executable and allocated sections (for some reason lld
           doesn't accept 'i' or 'l' for initialized sections) */

        /* 2GB start + 16 GB of RAM */
        /* sdram (rwxa) : ORIGIN = 0x80000000,  LENGTH = 0x400000000 */

        /* We use less ram (just shy of 2 GB) while testing */
        /* Something is occupying memory for QEMU at ffe0_0000...?
           which affects QEMU rom check and register reset if we try to use
           that area. So we will end at ffe0_0000 for now during development. */
        sdram (rwxa) : ORIGIN = 0x80000000,  LENGTH = 0x7FE00000
}

PHDRS
{
        rom PT_LOAD; /* Read-only section. We can use the PMP and/or MMU to enforce this */
        ram PT_LOAD;
}

SECTIONS
{
        /* Executable code (that is not jacked up in the ITIM or LIM, something we may
           consider later on) */
	.text : {
	      PROVIDE(_text_start = .);
	      *(.text.init) *(.text .text.*)
              *(.gnu.linkonce.t.*)
              *(.eh_frame) *(.eh_frame.*)
	      PROVIDE(_text_end = .);
	} >sdram AT>sdram :rom

        /* Read only constant data */
	.rodata : {
	        PROVIDE(_rodata_start = .);
                *(.rdata)
	        *(.rodata .rodata.*)
                *(.gnu.linkonce.r.*)
                . = ALIGN(8);
                *(.srodata.cst16)
                *(.srodata.cst8)
                *(.srodata.cst4)
                *(.srodata.cst2)
                *(.srodata .srodata.*)
	        PROVIDE(_rodata_end = .);
	} >sdram AT>sdram :rom

        /* Global variables initialized at compile time */
        /* Pages are 4k; We get ourselves out of the ROM pages area */
	.data : ALIGN(4096) {
	      PROVIDE(_data_start = .);

              *(.data .data.*)
              *(.gnu.linkonce.d.*)

              /* We want to get as many global variables to be within a 12-bit
                 signed offset of _global_pointer as possible for performance */
              /* I don't know why .data (above) doesn't count, but I'm mimicking
                 what the sifive linker scripts do. */
              /* See https://www.sifive.com/blog/all-aboard-part-3-linker-relaxation-in-riscv-toolchain */
              /* See meta.default.lds in freedom-e-sdk */
              PROVIDE(_global_pointer = . + 0x800);

	      *(.sdata .sdata.* .sdata2.*)
              *(.gnu.linkonce.s.*)
	      PROVIDE(_data_end = .);
	} >sdram AT>sdram :ram

        /* Global unitialized variables (space for them only) */
	.bss (NOLOAD): {
              PROVIDE(_bss_start = .);
              *(.sbss .sbss.*)
              *(.gnu.linkonce.sb.*)
              *(.bss .bss.*)
              *(.gnu.linkonce.b.*)
              *(COMMON)
              PROVIDE(_bss_end = .);
	} >sdram AT>sdram :ram

        /* Stack layout */
        /* Each hart gets its own stack of __stack_size. */
        /* Default is 80000 = 512K per hart */
        PROVIDE(_stack_size = 0x80000);

        .stack (NOLOAD): ALIGN(16) {
               PROVIDE(_stacks_start = .);
               . += _stack_size; /* Hart 4 */
               . += _stack_size; /* Hart 3 */
               . += _stack_size; /* Hart 2 */
               . += _stack_size; /* Hart 1 */
               . += _stack_size; /* Hart 0 */
               PROVIDE(_stacks_end = .);
        } >sdram AT>sdram :ram

        /* Heap layout */
	PROVIDE( _memory_start = ORIGIN(sdram) );
        PROVIDE( _memory_end = ORIGIN(sdram) + LENGTH(sdram));

        .heap (NOLOAD): ALIGN(8) {
              PROVIDE( _heap_start = .);
              . += _memory_end - _heap_start;
              PROVIDE( _heap_end = .);
        } >sdram AT>sdram : ram
        PROVIDE( _heap_size = _heap_end - _heap_start );

        /*  For release builds, we should discard these sections:
        /DISCARD/ : {
           *(.debug*)
           *(.comment*)
           *(.note*)
        }
        */
}
