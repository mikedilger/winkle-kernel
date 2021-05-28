/* Winkle linker script for:   QEMU RISC-V 64-bit Virtual machine (virt)  */

OUTPUT_ARCH( "riscv" )
OUTPUT_FORMAT( "elf64-littleriscv" )

/* Set the entry point (this is where execution begins, see boot.S) */
ENTRY( _start )

MEMORY
{
        /* Any section which is not listed can be stored here, and that applies to
           read-only, read-write, executable and allocated sections (for some reason lld
           doesn't accept 'i' or 'l' for initialized sections) */

        /* sdram (rwxa) : ORIGIN = 0x80000000,  LENGTH = 0x400000000 */

        /* QEMU riscv64 virt has a few objects in memory such as:
           1000 - 1028  mrom.reset
           1028 - 1058  mrom.finfo
               [VIRT_DEBUG] =       {        0x0,         0x100 },
               [VIRT_MROM] =        {     0x1000,        0xf000 },
               [VIRT_TEST] =        {   0x100000,        0x1000 },
               [VIRT_RTC] =         {   0x101000,        0x1000 },
               [VIRT_CLINT] =       {  0x2000000,       0x10000 },
               [VIRT_PCIE_PIO] =    {  0x3000000,       0x10000 },
               [VIRT_PLIC] =        {  0xc000000, VIRT_PLIC_SIZE(VIRT_CPUS_MAX * 2) },
               [VIRT_UART0] =       { 0x10000000,         0x100 },
               [VIRT_VIRTIO] =      { 0x10001000,        0x1000 },
               [VIRT_FW_CFG] =      { 0x10100000,          0x18 },
               [VIRT_FLASH] =       { 0x20000000,     0x4000000 },
               [VIRT_PCIE_ECAM] =   { 0x30000000,    0x10000000 },
               [VIRT_PCIE_MMIO] =   { 0x40000000,    0x40000000 },
               [VIRT_DRAM] =        { 0x80000000,           0x0 },

           */

        lowram (rwxa) : ORIGIN = 0x80000000,  LENGTH = 0x3F000000
        hiram (rwxa) : ORIGIN = 0xBF000000, LENGTH = 0x41000000
}

PHDRS
{
        rom PT_LOAD; /* Read-only section. We can use the PMP and/or MMU to enforce this */
        ram PT_LOAD;
}

SECTIONS
{
	.text : {
	      PROVIDE(_text_start = .);
	      *(.text.init) *(.text .text.*)
              *(.gnu.linkonce.t.*)
              *(.eh_frame) *(.eh_frame.*)
	      PROVIDE(_text_end = .);
	} >lowram AT>lowram :rom

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
	} >lowram AT>lowram :rom

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
	} >lowram AT>lowram :ram

        /* Global unitialized variables (space for them only) */
	.bss (NOLOAD): {
              PROVIDE(_bss_start = .);
              *(.sbss .sbss.*)
              *(.gnu.linkonce.sb.*)
              *(.bss .bss.*)
              *(.gnu.linkonce.b.*)
              *(COMMON)
              PROVIDE(_bss_end = .);
	} >lowram AT>lowram :ram

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
        } >lowram AT>lowram :ram

        /* Heap layout */
	PROVIDE( _memory_start = ORIGIN(lowram) );
        PROVIDE( _memory_end = ORIGIN(lowram) + LENGTH(lowram));

        .heap (NOLOAD): ALIGN(8) {
              PROVIDE( _heap_start = .);
              . += _memory_end - _heap_start;
              PROVIDE( _heap_end = .);
        } >lowram AT>lowram : ram
        PROVIDE( _heap_size = _heap_end - _heap_start );

        /*  For release builds, we should discard these sections:
        /DISCARD/ : {
           *(.debug*)
           *(.comment*)
           *(.note*)
        }
        */
}
