#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[cfg(feature = "teensy")]
pub mod spi {
    const SPI1_CR1: *mut u32 = 0x4002_3000 as *mut u32;  // SPI1 Control Register 1
    const SPI1_DR: *mut u32 = 0x4002_300C as *mut u32;   // SPI1 Data Register

        /*
        [CORRECTION SPI] (don't hesitate to remove this part)
        Some steps are missing in order for your SPI to work
        For example, since you are working with a cortex-m3 apparently, see p691 and p725 of https://www.st.com/resource/en/reference_manual/cd00225773-stm32f205xx-stm32f207xx-stm32f215xx-and-stm32f217xx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf
        */
    pub fn init_spi() {
        unsafe {
            // Set up SPI: Enable SPI, configure as master, set baud rate
            core::ptr::write_volatile(SPI1_CR1, 0x34);  // Example: SPI mode setup
        }
    }

    pub fn transmit(data: u8) {
        unsafe {
            // Wait until the transmit buffer is empty
            while core::ptr::read_volatile(SPI1_CR1) & 0x80 == 0 {}
            // Send data
            core::ptr::write_volatile(SPI1_DR, data as u32);
        }
    }

    pub fn receive() -> u8 {
        unsafe {
            // Wait until data is received
            while core::ptr::read_volatile(SPI1_CR1) & 0x40 == 0 {}
            // Read received data
            core::ptr::read_volatile(SPI1_DR) as u8
        }
    }
}
