OUTPUT_ARCH(riscv)


ENTRY(_start)

MEMORY
{
	ROM (rwx) : ORIGIN = 0x00000000, LENGTH = 200K 
}

SECTIONS
{
  .init : { *(.init); } > ROM
  .rodata : { *(.rodata); } > ROM
  .text : { *(.text.*); } > ROM
  .trap : ALIGN(4) { *(.trap); } > ROM


  .data (NOLOAD): { *(.data.*); } > ROM
  .sdata (NOLOAD) : { *(.sdata); } > ROM
  .bss (NOLOAD) : { *(.bss); } > ROM

}

PROVIDE(_hart_stack_size = 2K);
PROVIDE(_stack_start = ORIGIN(ROM) + LENGTH(ROM));
