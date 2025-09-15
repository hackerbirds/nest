use core::arch::asm;

pub fn el_level() -> ElLevel {
    let mut el: u64;

    unsafe {
        asm!("mrs {}, CurrentEL", out(reg) el);
    }

    el.into()
}

#[derive(Debug)]
pub enum ElLevel {
    EL0,
    EL1,
    EL2,
    EL3,
}

impl From<u64> for ElLevel {
    fn from(value: u64) -> Self {
        match value {
            0b0000 => ElLevel::EL0,
            0b0100 => ElLevel::EL1,
            0b1000 => ElLevel::EL2,
            0b1100 => ElLevel::EL3,
            _ => unreachable!("incorrect el value"),
        }
    }
}
