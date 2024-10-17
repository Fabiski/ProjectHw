#![no_std]
#![no_main]

use cortex_m_rt::entry;


const PORTB: *mut u8 = 0x25 as *mut u8; // PORTB address
const DDRB: *mut u8 = 0x24 as *mut u8;  // DDRB address

#[entry]
fn main() -> ! {
    // Infinite loop
    loop {
        // Turn on the LED (set bit 5 in PORTB)
        unsafe {
            core::ptr::write_volatile(DDRB,0b00100000);
            loop {
                for _ in 0..1_000_000 
                {
                    core::ptr::write_volatile(PORTB, 0b00100000);
                }
                for _ in 0..100_000
                {
                    core::ptr::write_volatile(PORTB, 0b00000000);
                }
            }
        }
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}