ENTRY(_start)
SECTIONS
{
    . = 0x40080000;
    .text.boot : { *(.text.boot) }
    .text : { *(.text*) }
    .exception_vectors : { *(.exception_vectors) }
    .data : { *(.data*) }
    .rodata : { *(.rodata*) }
    .bss : { 
        __EXT_BSS_START = .;
        *(.bss*);
        . = ALIGN(8);
        
        . += 8;
        __EXT_BSS_END_INCLUSIVE = . - 8;
    }

    . = ALIGN(8);
    . = . + 0x4000;
    __EXT_STACK_END = .;
}
