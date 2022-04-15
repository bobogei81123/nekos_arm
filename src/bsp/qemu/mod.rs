use core::ptr;

use embedded_hal::serial;

pub const BOARD_NAME: &'static str = "QEMU";

pub struct Serial;

const UART0: *mut u8 = 0x0900_0000 as *mut u8;

impl Serial {
    pub fn new() -> Self {
        Self
    }
}

impl serial::Write<u8> for Serial {
    type Error = !;

    fn write(&mut self, word: u8) -> nb::Result<(), !> {
        unsafe { ptr::write_volatile(UART0, word); }
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), !> {
        Ok(())
    }
}
