use core::arch::naked_asm;

pub static HEAP: [u8; 0x1000000] = [0; 0x1000000];

const MAIR_EL1: usize = {
    // Normal memory
    const ATTR_1: usize = 0b11111111;
    // nGnRE
    const ATTR_0: usize = 0b00000100;

    ATTR_1 << 8 | ATTR_0 << 0
};

const TCR_EL1: usize = {
    const T0SZ: usize = 0b10000;
    const RES0: usize = 0b0;
    const EPD0: usize = 0b0;
    const IRGN0: usize = 0b01;
    const ORGN0: usize = 0b01;
    const SH0: usize = 0b11;

    // 4kb page
    const TG0: usize = 0b00;

    const T1SZ: usize = 0b10000;
    const A1: usize = 0b0;
    const EPD1: usize = 0b0;
    const IRGN1: usize = 0b01;
    const ORGN1: usize = 0b01;
    const SH1: usize = 0b11;

    // 4kb page
    const TG1: usize = 0b10;

    // 48-bit table type
    const IPS: usize = 0b101;

    IPS << 32
        | TG1 << 30
        | SH1 << 28
        | ORGN1 << 26
        | IRGN1 << 24
        | EPD1 << 23
        | A1 << 22
        | T1SZ << 16
        | TG0 << 14
        | SH0 << 12
        | ORGN0 << 10
        | IRGN0 << 8
        | EPD0 << 7
        | RES0 << 6
        | T0SZ << 0
};

pub const USER_START_ADDRESS_RW: usize = 0x00000000;
pub const USER_END_ADDRESS_RW: usize = 0x3fffffff;
pub const USER_START_ADDRESS_RWE: usize = 0x40000000;
pub const USER_END_ADDRESS_RWE: usize = 0x7fffffff;

pub const KERNEL_ADDRESS_PREFIX: usize = 0xffff0000 << 32;
pub const KERNEL_START_ADDRESS_RW: usize = KERNEL_ADDRESS_PREFIX | USER_START_ADDRESS_RW;
pub const KERNEL_END_ADDRESS_RW: usize = KERNEL_ADDRESS_PREFIX | USER_END_ADDRESS_RW;
pub const KERNEL_START_ADDRESS_RWE: usize = KERNEL_ADDRESS_PREFIX | USER_START_ADDRESS_RWE;
pub const KERNEL_END_ADDRESS_RWE: usize = KERNEL_ADDRESS_PREFIX | USER_END_ADDRESS_RWE;

pub const TTRB1_BLOCK_ENTRY_0: usize = {
    const UXN: usize = 0b1;
    const PXN: usize = 0b1;
    const AF: usize = 0b1;
    const SH: usize = 0b10;
    const AP2_1: usize = 0b00;
    const NS: usize = 0b0;
    const ATTR_INDX_2_0: usize = 0b000;

    UXN << 54 | PXN << 53 | AF << 10 | SH << 8 | AP2_1 << 6 | NS << 5 | ATTR_INDX_2_0 << 2 | 0b01
};

// 0x40000000+ block: Executable
pub const TTRB1_BLOCK_ENTRY_1: usize = {
    const UXN: usize = 0b0;
    const PXN: usize = 0b0;
    const AF: usize = 0b1;
    const SH: usize = 0b11;
    const AP2_1: usize = 0b00;
    const NS: usize = 0b0;
    const ATTR_INDX_2_0: usize = 0b001;

    UXN << 54 | PXN << 53 | AF << 10 | SH << 8 | AP2_1 << 6 | NS << 5 | ATTR_INDX_2_0 << 2 | 0b01
};

#[unsafe(naked)]
pub fn setup_mmu() {
    naked_asm!(
        // Set up MAIR_EL1
        "ldr     x0, ={mair_el1}",
        "msr     mair_el1, x0",
        // Memory barrier to ensure CPU processes instruction before continuing
        "isb",

        // Set up TTBR0 and TTBR1
        "ldr     x0, =pagetable_level0",
        "msr     ttbr0_el1, x0",
        "isb",

        "ldr     x1, =pagetable_level1",
        // Table descriptor/entry
        "orr     x2, x1, 0b11",
        // Store level 1 table in entry of table 0
        "str     x2, [x0]",

        // (1gb) Block entry
        // zeroize 32 lsb
        "ldr     x4, =0x00000000",
        // shift right by 30 to get l1 table index in x5
        "lsr     x5, x4, 30",
        // set table index to 0b11111111
        "and     x5, x5, 0x1ff",
        // shift back to get loadable virtual address
        "lsl     x4, x5, 30",
        // add attributes
        "ldr     x6, ={ttrb1_0}",
        "orr     x4, x4, x6",
        // set value in x4 to x1 + (x5 << 3)
        "str     x4, [x1, x5, lsl 3]",

        "ldr     x4, =0x40000000",
        "lsr     x5, x4, #30",
        "and     x5, x5, 0b11111111",
        "lsl     x4, x5, #30",
        "ldr     x6, ={ttrb1_1}",
        "orr     x4, x4, x6",
        // set value of x4 to address of block entry 1
        "str     x4, [x1, x5, lsl 3]",

        // Set up TTBR1
        "msr     ttbr1_el1, x0",
        "isb",

        // Set up TCR_EL1
        // https://developer.arm.com/documentation/ddi0601/2025-06/AArch64-Registers/TCR-EL1--Translation-Control-Register--EL1-?lang=en
        "ldr     x0, ={tcr_el1}",
        "msr     tcr_el1, x0",
        "isb",

        // Set MMU bit to 1 in system control register
        "mrs     x0, sctlr_el1",
        "orr     x0, x0, #1",
        "orr     x0, x0, #(1 << 12)",
        "msr     sctlr_el1, x0",
        "isb",
        "ret",
        mair_el1 = const MAIR_EL1,
        tcr_el1 = const TCR_EL1,
        ttrb1_0 = const TTRB1_BLOCK_ENTRY_0,
        ttrb1_1 = const TTRB1_BLOCK_ENTRY_1,
    );
}
