#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;

mod gpio;
mod usart;

#[entry]
fn main() -> ! {
    // Initialize GPIO
    gpio::configure_pin(13, true); // Set pin 13 as output

    // Initialize USART
    usart::init_usart(9600);

    loop {
        // Read from USART
        let received_data = usart::receive();

        // Toggle LED based on received data
        if received_data == b'1' {
            gpio::write_pin(13, true);
        } else if received_data == b'0' {
            gpio::write_pin(13, false);
        }

        // Transmit data back
        usart::transmit(received_data);
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
