#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;

// Import the GPIO and USART modules
mod gpio;
mod usart;

#[cfg(feature = "arduino")]
mod gpio {
    include!("gpio.rs");
}

#[cfg(feature = "teensy")]
mod gpio {
    include!("gpio_cortex.rs");
}

#[cfg(feature = "arduino")]
mod usart {
    include!("usart.rs");
}

#[cfg(feature = "teensy")]
mod usart {
    include!("usart_cortex.rs");
}

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
