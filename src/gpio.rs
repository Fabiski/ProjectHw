#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;

#[cfg(feature = "arduino")]
pub mod gpio {
    const PORTB: *mut u8 = 0x25 as *mut u8; // PORTB address
    const DDRB: *mut u8 = 0x24 as *mut u8;  // DDRB address
    const PORTD: *mut u8 = 0x2B as *mut u8; // PORTD address
    const DDRD: *mut u8 = 0x2A as *mut u8;  // DDRD address
    const PINB: *mut u8 = 0x23 as *mut u8;  //PINB address
    const PIND: *mut u8 = 0x29 as *mut u8;  //PIND address


    // Function to configure a pin as input or output
    pub fn configure_pin(pin: u8, mode: bool) {
        unsafe {

            // Determine the port, data direction register, and pin address based on the pin number
            let (port, ddr, pin_addr) = match pin {
                0..=7 => (PORTD, DDRD, PIND),
                8..=13 => (PORTB, DDRB, PINB),
                _ => panic!("Invalid pin number"),
            };

            // Calculate the source bit for the pin
            let source = 1 << (pin % 8);
            if mode {
                // Set the pin as output
                core::ptr::write_volatile(ddr, source);
            } else {
                // Set the pin as input
                core::ptr::write_volatile(ddr, 0);
            }
        }
    }

    // Function to read the state of a pin
    pub fn read_pin(pin: u8) -> bool {
        unsafe {

            // Determine the pin address based on the pin number
            let pin_addr = match pin {
                0..=7 => PIND,
                8..=13 => PINB,
                _ => panic!("Invalid pin number"),
            };

            // Calculate the source bit for the pin
            let source = 1 << (pin % 8);
            // Return true if the pin is high, false otherwise
            (*pin_addr & source) != 0
        }
    }

    // Function to write a value to a pin
    pub fn write_pin(pin: u8, value: bool) {
        unsafe {

            // Determine the port based on the pin number
            let port = match pin {
                0..=7 => PORTD,
                8..=13 => PORTB,
                _ => panic!("Invalid pin number"),
            };

            // Calculate the source bit for the pin
            let source = 1 << (pin % 8);
            if value {
                // Set the pin high
                core::ptr::write_volatile(port, source);
            } else {
                // Set the pin low
                core::ptr::write_volatile(port, 0);
            }
        }
    }
}
