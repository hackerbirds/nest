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

// https://dannasman.github.io/aarch64-mmu
#[unsafe(naked)]
pub fn setup_mmu() {
    naked_asm!(
        // Set up MAIR_EL1
        "ldr     x0, ={mair_el1}",
        "msr     mair_el1, x0",
        // Memory barrier to ensure CPU processes instruction before continuing
        "isb",

        // Set up TTBR0
        "ldr     x0, ={ttbr0_el1}",
        "msr     ttbr0_el1, x0",
        "isb",
        // Set up TTBR1
        "ldr     x0, ={ttbr1_el1}",
        "msr     ttbr1_el1, x0",
        "isb",

        // Set up TCR_EL1
        // https://developer.arm.com/documentation/ddi0601/2025-06/AArch64-Registers/TCR-EL1--Translation-Control-Register--EL1-?lang=en
        "ldr     x0, ={tcr_el1}",
        "msr     tcr_el1, x0",
        "isb",

        // Set MMU bit to 1 in system control register
        "mov     x0, xzr",
        "mrs     x0, sctlr_el1",
        "orr     x0, x0, #1",
        "orr     x0, x0, #(1 << 12)",
        "msr     sctlr_el1, x0",
        // TODO: fails
        "isb",
        "ret",
        mair_el1 = const MAIR_EL1,
        ttbr0_el1 = sym crate::pagetable_level0,
        ttbr1_el1 = sym crate::pagetable_level1,
        tcr_el1 = const TCR_EL1
    );
}
