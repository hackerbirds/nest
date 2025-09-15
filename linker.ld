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

    . = ALIGN(4096); /* align page */
    heap_start = .;
    . += 1024 * 1024; /* 1mb heap */
    heap_end = .;

}