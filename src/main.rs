#![no_std]
#![no_main]

pub mod memory;
pub mod uart;
pub mod util;

use core::arch::asm;
use core::panic::PanicInfo;
use smccc::{
    Hvc,
    psci::{system_off, system_reset},
};

use crate::uart::Pl011Uart;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
pub extern "C" fn boot() -> ! {
    unsafe {
        asm!(
            "ldr x30, =stack_top",
            "mov sp, x30",
            "bl {main}",
            main = sym main,
            options(noreturn)
        );
    }
}

fn main() -> ! {
    Pl011Uart::print(b"the first twig has been placed. the nest is coming together.\n");

    system_off::<Hvc>().unwrap();

    panic!("system_off execution completed");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    system_reset::<Hvc>().unwrap();

    loop {}
}
