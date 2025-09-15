#![no_std]
#![no_main]

pub mod memory;
pub mod uart;
pub mod util;

extern crate alloc;

use buddy_system_allocator::LockedHeap;
use core::arch::asm;
use core::panic::PanicInfo;
use smccc::{Hvc, psci::system_off};

use crate::{
    uart::Pl011Uart,
    util::{ElLevel, el_level},
};

unsafe extern "C" {
    static mut heap_start: u8;
    static mut heap_end: u8;
}

#[global_allocator]
pub static HEAP_ALLOCATOR: LockedHeap<33> = LockedHeap::empty();

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
    Pl011Uart::print(b"the first twig has been placed.\n");

    // Initiate allocator
    // SAFETY: Heap pointers are defined by linker
    unsafe {
        const HEAP_SIZE: usize = 1024 * 1024;

        let heap_start_addr = (&raw mut heap_start).addr();
        let heap_end_addr = (&raw mut heap_end).addr();

        debug_assert_eq!(heap_end_addr - heap_start_addr, HEAP_SIZE);

        HEAP_ALLOCATOR.lock().init(heap_start_addr, HEAP_SIZE);
    }

    Pl011Uart::print(b"the nest is coming together.\n");

    match el_level() {
        ElLevel::EL0 => Pl011Uart::print(b"EL0!\n"),
        ElLevel::EL1 => Pl011Uart::print(b"EL1!\n"),
        ElLevel::EL2 => Pl011Uart::print(b"EL2!\n"),
        ElLevel::EL3 => Pl011Uart::print(b"EL3!\n"),
    }

    system_off::<Hvc>().unwrap();

    panic!("system_off execution completed");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Pl011Uart::print(b"PANIC!\n");
    Pl011Uart::print(info.message().as_str().unwrap_or_default().as_bytes());

    // For now, turn off system after panic to prevent terminal printing infinitely
    system_off::<Hvc>().unwrap();
    // system_reset::<Hvc>().unwrap();

    loop {}
}
