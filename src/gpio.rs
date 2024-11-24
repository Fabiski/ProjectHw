#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;

const PORTB: *mut u8 = 0x25 as *mut u8; // PORTB address
const DDRB: *mut u8 = 0x24 as *mut u8;  // DDRB address
const PORTD: *mut u8 = 0x2B as *mut u8; // PORTD address
const DDRD: *mut u8 = 0x2A as *mut u8;  // DDRD address
const PINB: *mut u8 = 0x23 as *mut u8;  //PINB address
const PIND: *mut u8 = 0x29 as *mut u8;  //PIND address

pub fn configure_pin(pin: u8, mode: bool) {
    unsafe {
        let (port, ddr, pin_addr) = match pin {
            0..=7 => (PORTD, DDRD, PIND),
            8..=13 => (PORTB, DDRB, PINB),
            _ => panic!("Invalid pin number"),
        };

        let source = 1 << (pin % 8);
        if mode {
            core::ptr::write_volatile(ddr, source);
        } else {
            core::ptr::write_volatile(ddr, 0);
        }
    }
}

pub fn read_pin(pin: u8) -> bool {
    unsafe {
        let pin_addr = match pin {
            0..=7 => PIND,
            8..=13 => PINB,
            _ => panic!("Invalid pin number"),
        };

        let source = 1 << (pin % 8);
        (*pin_addr & source) != 0
    }
}

pub fn write_pin(pin: u8, value: bool) {
    unsafe {
        let port = match pin {
            0..=7 => PORTD,
            8..=13 => PORTB,
            _ => panic!("Invalid pin number"),
        };

        let source = 1 << (pin % 8);
        if value {
            core::ptr::write_volatile(port, source);
        } else {
            core::ptr::write_volatile(port, 0);
        }
    }
}
