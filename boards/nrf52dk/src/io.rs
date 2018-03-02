use core::fmt::{Arguments, Write};
use kernel::debug;
use kernel::hil::led;
use kernel::hil::uart::{self, UART};
use nrf52;
use nrf5x;

pub struct Writer {
    initialized: bool,
}

pub static mut WRITER: Writer = Writer { initialized: false };

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        // The UART is not enabled yet
        let uart = unsafe { &mut nrf52::uart::UART0 };
        if !self.initialized {
            self.initialized = true;
            uart.init(uart::UARTParams {
                baud_rate: 115200,
                stop_bits: uart::StopBits::One,
                parity: uart::Parity::None,
                hw_flow_control: false,
            });
        }
        for c in s.bytes() {
            unsafe {
                uart.send_byte(c);
            }
            while !uart.tx_ready() {}
        }
        Ok(())
    }
}

#[cfg(not(test))]
#[no_mangle]
#[lang = "panic_fmt"]
pub unsafe extern "C" fn panic_fmt(args: Arguments, file: &'static str, line: u32) -> ! {
    // The nRF52 DK LEDs (see back of board)
    const LED1_PIN: usize = 17;
    let led = &mut led::LedLow::new(&mut nrf5x::gpio::PORT[LED1_PIN]);
    let writer = &mut WRITER;
    debug::panic(led, writer, args, file, line)
}
