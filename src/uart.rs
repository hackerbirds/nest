//! Basic UART print

const PL011_ADDRESS: *mut u8 = 0x0900_0000 as _;

pub struct Pl011Uart;

impl Pl011Uart {
    pub fn print(s: &[u8]) {
        for &c in s {
            // SAFETY: Is guaranteed by CPU implementation of UART
            unsafe {
                *PL011_ADDRESS = c;
            }
        }
    }
}
