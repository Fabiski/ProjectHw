#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;

// Import the GPIO, USART, and SPI modules
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

#[cfg(feature = "arduino")]
mod spi {
    include!("spi_atmega.rs");
}

#[cfg(feature = "teensy")]
mod spi {
    include!("spi_cortex.rs");
}

#[entry]
fn main() -> ! {
    // Initialize GPIO
    gpio::configure_pin(13, true); // Set pin 13 as output

    // Initialize USART
    usart::init_usart(9600);

    // Initialize SPI
    spi::init_spi();

    loop {
        // Read from USART
        let received_data = usart::receive();

        // Toggle LED based on received data
        if received_data == b'1' {
            gpio::write_pin(13, true);
        } else if received_data == b'0' {
            gpio::write_pin(13, false);
        }

        // Transmit data back over USART
        usart::transmit(received_data);

        // SPI Communication: Send a byte (e.g., 0x55) and receive the response
        let data_to_send = 0x55;
        spi::transmit(data_to_send);  // Send data via SPI
        let received_spi_data = spi::receive();  // Receive data via SPI

        // Do something with the received SPI data
        if received_spi_data == 0x55 {
            gpio::write_pin(13, true);  // Turn on LED if SPI data matches
        }
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
