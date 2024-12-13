#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[cfg(feature = "arduino")]
pub mod spi {
    const SPCR: *mut u8 = 0x2C as *mut u8;  // SPI Control Register
    const SPSR: *mut u8 = 0x2E as *mut u8;  // SPI Status Register
    const SPDR: *mut u8 = 0x2F as *mut u8;  // SPI Data Register
    const PINB: *mut u8 = 0x23 as *mut u8;  // Port B input register

    pub fn init_spi() {
        unsafe {
            // Set SPI to Master mode, enable SPI, set clock polarity and phase, and baud rate
            core::ptr::write_volatile(SPCR, 0x53);  // Example: SPI mode setup
        }
    }

    pub fn transmit(data: u8) {
        unsafe {
            // Wait for the transmit buffer to be empty
            while core::ptr::read_volatile(SPSR) & 0x01 == 0 {}
            // Send data
            core::ptr::write_volatile(SPDR, data);
        }
    }

    pub fn receive() -> u8 {
        unsafe {
            // Wait for data to be received
            while core::ptr::read_volatile(SPSR) & 0x80 == 0 {}
            // Read received data
            core::ptr::read_volatile(SPDR)
        }
    }
}
