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
    . += 0x1000; /* 4kB stack */
    stack_top = .;

    . = ALIGN(0x1000);
    pagetable_level0 = .;
    . += 0x1000;
    . = ALIGN(0x1000);
    pagetable_level1 = .;
    . += 0x1000;
}