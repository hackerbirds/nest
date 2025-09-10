#![no_std]
#![no_main]

use aarch64_rt::entry;
use arm_pl011_uart::{PL011Registers, Uart, UniqueMmioPointer};
use core::{fmt::Write, panic::PanicInfo, ptr::NonNull};
use smccc::{
    Hvc,
    psci::{system_off, system_reset},
};

const PL011_ADDRESS: *mut PL011Registers = 0x0900_0000 as _;

entry!(main);
fn main(_arg0: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> ! {
    let uart_ptr = unsafe { UniqueMmioPointer::new(NonNull::new(PL011_ADDRESS).unwrap()) };
    let mut uart = Uart::new(uart_ptr);

    writeln!(
        uart,
        "the first twig has been placed. the nest is coming together."
    )
    .unwrap();

    system_off::<Hvc>().unwrap();

    panic!("system_off execution completed");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    system_reset::<Hvc>().unwrap();

    loop {}
}
