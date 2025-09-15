ENTRY(boot)
SECTIONS {
    . = 0x40100000;

    .text :{
        KEEP(*(.text.boot));
        *(.text .text.*);
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(.bss COMMON)
    }

    . = ALIGN(8);
    . += 0x1000;
    stack_top = .;
}