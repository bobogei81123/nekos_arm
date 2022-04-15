use core::ptr;
use embedded_hal::serial;

pub const BOARD_NAME: &'static str = "Pinephone";

pub struct Serial;

const CCU_BUS_CLK_GATING_REG3: *mut u32 = 0x01C2006C as *mut _;
const BUS_CLK_GATING_REG3_UART0_GATING: u32 = 1u32 << 16;

const UART0_USR: *mut u32 = 0x01C2807C as *mut _;
const UART_USR_BUSY: u32 = 1 << 0;
const UART0_LCR: *mut u32 = 0x01C2800C as *mut _;
const UART_LCR_DLAB: u32 = 1 << 7;
const UART0_DLL: *mut u32 = 0x01C28000 as *mut _;
const UART0_LSR: *mut u32 = 0x01C28014 as *mut _;
const UART_LSR_THRE: u32 = 1 << 5;
const UART0_THR: *mut u32 = 0x01C28000 as *mut _;
const UART0_FCR: *mut u32 = 0x01C28008 as *mut _;
const UART_FCR_FIFOE: u32 = 1 << 0;

fn sleep() {
    for _ in 0..100_000_000 {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

impl Serial {
    pub fn new() -> Self {
        let mut s = Self;
        s.setup_serial();
        s
    }

    fn wait_serial_ready(&mut self) {
        while unsafe { ptr::read_volatile(UART0_USR) & UART_USR_BUSY != 0 } {}
    }

    fn setup_serial(&mut self) {
        unsafe {
            ptr::write_volatile(CCU_BUS_CLK_GATING_REG3, BUS_CLK_GATING_REG3_UART0_GATING);
            sleep();

            self.wait_serial_ready();
            ptr::write_volatile(UART0_LCR, UART_LCR_DLAB);
            self.wait_serial_ready();
            ptr::write_volatile(UART0_DLL, 13);
            self.wait_serial_ready();
            ptr::write_volatile(UART0_LCR, 0);
            self.wait_serial_ready();
            ptr::write_volatile(UART0_LCR, 0x03);
            self.wait_serial_ready();
        }
    }

    fn send_serial(&mut self, chr: u8) {
        self.wait_serial_ready();
        unsafe {
            while (ptr::read_volatile(UART0_LSR) & UART_LSR_THRE) == 0 {}
            ptr::write_volatile(UART0_THR, chr as u32);
        }
        self.wait_serial_ready();
    }
}

impl serial::Write<u8> for Serial {
    type Error = !;

    fn write(&mut self, word: u8) -> nb::Result<(), !> {
        unsafe {
            self.send_serial(word);
        }
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), !> {
        Ok(())
    }
}
