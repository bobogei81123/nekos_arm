struct SerialWriter;

use core::fmt::{self, Write};
use core::ptr;
use crate::crazy::CrazyCell;

const UART0: *mut u8 = 0x0900_0000 as *mut u8;

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // TODO: handle utf-8
        for byte in s.bytes() {
            unsafe {
                ptr::write_volatile(UART0, byte);
            }
        }
        Ok(())
    }
}

static SERIAL: CrazyCell<SerialWriter> = CrazyCell::new(SerialWriter {});

#[doc(hidden)]
unsafe fn mutable_serial() -> &'static mut SerialWriter {
    &mut *SERIAL.get()
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    unsafe { mutable_serial() }.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
