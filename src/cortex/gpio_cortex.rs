#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;

const GPIOA_BASE: usize = 0x400FF000; // Base address for GPIO Port A (example)
const GPIOB_BASE: usize = 0x400FF040; // Base address for GPIO Port B (example)

const GPIO_PDDR_OFFSET: usize = 0x14; // Offset for Data Direction Register
const GPIO_PDOR_OFFSET: usize = 0x0;  // Offset for Data Output Register
const GPIO_PDIR_OFFSET: usize = 0x10; // Offset for Data Input Register

// Function to get the base address for the GPIO port and offset pin
fn get_gpio(pin: u8) -> (*mut u32, u8) {
    match pin {
        0..=31 => (GPIOA_BASE as *mut u32, pin), // Pins 0-31 on Port A
        32..=63 => (GPIOB_BASE as *mut u32, pin - 32), // Pins 32-63 on Port B
        _ => panic!("Invalid pin number"),
    }
}

// Function to configure a pin as input or output
pub fn configure_pin(pin: u8, mode: bool) {
    unsafe {
        let (gpio_base, pin_offset) = get_gpio(pin);
        let pddr = gpio_base.add(GPIO_PDDR_OFFSET / 4); // Pointer to Data Direction Register
        let mask = 1 << pin_offset;

        if mode {
            // Set pin as output
            *pddr |= mask;
        } else {
            // Set pin as input
            *pddr &= !mask;
        }
    }
}

// Function to read the state of a pin
pub fn read_pin(pin: u8) -> bool {
    unsafe {
        let (gpio_base, pin_offset) = get_gpio(pin);
        let pdir = gpio_base.add(GPIO_PDIR_OFFSET / 4); // Pointer to Data Input Register
        let mask = 1 << pin_offset;

        (*pdir & mask) != 0
    }
}

// Function to write a value to a pin
pub fn write_pin(pin: u8, value: bool) {
    unsafe {
        let (gpio_base, pin_offset) = get_gpio(pin);
        let pdor = gpio_base.add(GPIO_PDOR_OFFSET / 4); // Pointer to Data Output Register
        let mask = 1 << pin_offset;

        if value {
            // Set pin high
            *pdor |= mask;
        } else {
            // Set pin low
            *pdor &= !mask;
        }
    }
}

#[entry]
fn main() -> ! {
    // Example: Configure pin 13 as output and set it high
    configure_pin(13, true); // Configure pin 13 as output
    write_pin(13, true);     // Set pin 13 high

    loop {
        // Toggle pin 13
        let state = read_pin(13);
        write_pin(13, !state);
        cortex_m::asm::delay(1_000_000); // Simple delay
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
