use crate::bsp;
use crate::{singleton::Singleton, sync::SpinMutex};
use core::fmt::{self, Write};
use core::ptr;

static_assertions::assert_impl_all!(bsp::Serial: embedded_hal::serial::Write<u8>);

struct Serial {
    bsp: bsp::Serial,
}

impl Serial {
    fn new() -> Self {
        Self { bsp: bsp::Serial::new() }
    }
}

static SERIAL: Singleton<SpinMutex<Serial>> = Singleton::new();

pub fn serial_init() {
    unsafe { SERIAL.init(SpinMutex::new(Serial::new())); }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut serial = SERIAL.get().lock();
    (&mut serial.bsp as &mut dyn embedded_hal::serial::Write<u8, Error = !>)
        .write_fmt(args)
        .unwrap();
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
