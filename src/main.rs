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

    #[cfg(feature = "arduino")]
    gpio::configure_pin(13, true); // Set pin 13 as output

    #[cfg(feature = "teensy")]
    gpio::configure_pin(13, true); // Set pin 13 as output

    // Platform-specific USART initialization
    #[cfg(feature = "arduino")]
    usart::init_usart(9600);

    #[cfg(feature = "teensy")]
    usart::init_usart(9600);

    // Platform-specific SPI initialization
    #[cfg(feature = "arduino")]
    spi::init_spi();

    #[cfg(feature = "teensy")]
    spi::init_spi();

    loop {
        // Read from USART
        #[cfg(feature = "arduino")]
        let received_data = usart::receive();

        #[cfg(feature = "teensy")]
        let received_data = usart::receive();

        // Toggle LED based on received data
        #[cfg(feature = "arduino")]
        if received_data == b'1' {
            gpio::write_pin(13, true);
        } else if received_data == b'0' {
            gpio::write_pin(13, false);
        }

        #[cfg(feature = "teensy")]
        if received_data == b'1' {
            gpio::write_pin(13, true);
        } else if received_data == b'0' {
            gpio::write_pin(13, false);
        }
        // Transmit data back over USART
        #[cfg(feature = "arduino")]
        usart::transmit(received_data);

        #[cfg(feature = "teensy")]
        usart::transmit(received_data);

        // SPI Communication: Send a byte (e.g., 0x55) and receive the response
        let data_to_send = 0x55;

        // Platform-specific USART transmit
        #[cfg(feature = "arduino")]
        usart::transmit(received_data);

        #[cfg(feature = "teensy")]
        usart::transmit(received_data);

        #[cfg(feature = "arduino")]
        {
            let received_spi_data = spi::receive();  // Receive data via SPI

            // Do something with the received SPI data
            if received_spi_data == 0x55 {
                gpio::write_pin(13, true);  // Turn on LED if SPI data matches
            }
        }

        #[cfg(feature = "teensy")]
        {
            let received_spi_data = spi::receive();  // Receive data via SPI

            // Do something with the received SPI data
            if received_spi_data == 0x55 {
                gpio::write_pin(13, true);  // Turn on LED if SPI data matches
            }
        }
        
        /*
        [CORRECTION SPI] (don't hesitate to remove this part)
        You have this type of compilation's error :

            error[E0425]: cannot find function `write_pin` in module `gpio`
              --> src/main.rs:73:19
               |
            73 |             gpio::write_pin(13, true);  // Turn on LED if SPI data matches
               |                   ^^^^^^^^^ not found in `gpio`
               |
            help: consider importing this function
               |
            4  + use crate::gpio::gpio::write_pin;

        Even though everything should be fine (you put everything in public for example) : 
        This is because your function's declaration are guarded by features ("atmega" and "teensy"), but their call are not.
        To solve this you could : 
            - guard the call by features as well
            - make more generic function's declaration, and specify your target more precise function under it, with features
        In your case, the first option seems like the best one 
        */
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
