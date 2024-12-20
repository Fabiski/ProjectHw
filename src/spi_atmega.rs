#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[cfg(feature = "arduino")]
pub mod spi {
    const SPCR: *mut u8 = 0x4C as *mut u8;  // SPI Control Register
    const SPSR: *mut u8 = 0x4E as *mut u8;  // SPI Status Register
    const SPDR: *mut u8 = 0x4F as *mut u8;  // SPI Data Register


    pub fn init_master() {
        unsafe {
            core::ptr::write_volatile(
                SPCR,
                (1 << 6)  // SPE: SPI Enable (active)
                | (1 << 4)  // MSTR: Master (mode maître)
                | (0 << 3)  // CPOL: Clock Polarity low (0)
                | (0 << 2)  // CPHA: Clock Phase 1st edge (0)
                | (0 << 1)  // SPR1: Clock Rate Select bit 1 (f_CPU / 4)
                | (1 << 0)  // SPR0: Clock Rate Select bit 0 (f_CPU / 4)
            );
        }
    }
    
    pub fn init_slave() {
        unsafe {
            core::ptr::write_volatile(
                SPCR,
                (1 << 6)  // SPE: SPI Enable (active)
                | (0 << 4)  // MSTR: Master (mode esclave)
                | (0 << 3)  // CPOL: Clock Polarity low (0)
                | (0 << 2)  // CPHA: Clock Phase 1st edge (0)
                | (0 << 1)  // SPR1: Clock Rate Select bit 1 (f_CPU / 4)
                | (1 << 0)  // SPR0: Clock Rate Select bit 0 (f_CPU / 4)
            );
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
