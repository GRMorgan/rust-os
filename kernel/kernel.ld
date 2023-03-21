OUTPUT_FORMAT(elf64-x86-64)

ENTRY(_start)

SECTIONS
{
    . = 0xFFFFFFFF80000000;
    _KernelStart = .;
    .text :
    {
        *(.text .text.*)
    }

    .bss :
    {
        *(.bss .bss.*)
    }

    .rodata :
    {
        *(.rodata .rodata.*)
    }

    .data.rel.ro :
    {
        *(.data.rel.ro .data.rel.ro.*)
    }
    
    _KernelEnd = .;
}
