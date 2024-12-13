#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;

#[cfg(feature = "teensy")]
pub mod usart {
    const UART0_BASE: usize = 0x4006A000;
    const UART0_BDH: usize = 0x00; // Baud Rate Register High
    const UART0_BDL: usize = 0x01; // Baud Rate Register Low
    const UART0_C1: usize = 0x02;  // Control Register 1
    const UART0_C2: usize = 0x03;  // Control Register 2
    const UART0_S1: usize = 0x04;  // Status Register 1
    const UART0_D: usize = 0x07;   // Data Register

    // Initializes the USART
    pub fn init_usart(baud_rate: u32) {
        unsafe {
            // Set baud rate
            let baud = (48_000_000 / (16 * baud_rate) - 1) as u16;
            core::ptr::write_volatile((UART0_BASE + UART0_BDL) as *mut u8, baud as u8);
            core::ptr::write_volatile((UART0_BASE + UART0_BDH) as *mut u8, (baud >> 8) as u8);

            // Enable receiver and transmitter
            core::ptr::write_volatile((UART0_BASE + UART0_C2) as *mut u8, 0b00001100);

            // Set frame format: 8 data bits, 1 stop bit
            core::ptr::write_volatile((UART0_BASE + UART0_C1) as *mut u8, 0b00000000);
        }
    }

    // Transmits using USART.
    pub fn transmit(data: u8) {
        unsafe {
            // Wait for transmit buffer to be empty
            while (core::ptr::read_volatile((UART0_BASE + UART0_S1) as *mut u8) & 0b10000000) == 0 {}

            // Write the data to the USART
            core::ptr::write_volatile((UART0_BASE + UART0_D) as *mut u8, data);
        }
    }

    // Receives data using USART.
    pub fn receive() -> u8 {
        unsafe {
            // Wait for receive buffer to be full
            while (core::ptr::read_volatile((UART0_BASE + UART0_S1) as *mut u8) & 0b00100000) == 0 {}

            // Read and return the received byte from the USART
            core::ptr::read_volatile((UART0_BASE + UART0_D) as *mut u8)
        }
    }
}
