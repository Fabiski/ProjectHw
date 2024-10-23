#![no_std]
#![no_main]

use cortex_m_rt::entry;


const PORTB: *mut u8 = 0x25 as *mut u8; // PORTB address
const DDRB: *mut u8 = 0x24 as *mut u8;  // DDRB address
const PORTD: *mut u8 = 0x2B as *mut u8; // PORTD address
const DDRD: *mut u8 = 0x2A as *mut u8;  // DDRD address
const PINB: *mut u8 = 0x23 as *mut u8;  //PINB address
const PIND: *mut u8 = 0x29 as *mut u8;  //PIND address
const SOURCE: *mut u8 = 0b00000001 as *mut u8;  // SOURCE (chosen port, will change in the switch) 
const CHOICED: *mut u8;
const CHOICEP: *mut u8;
const CHOICEPIN: *mut u8;

//https://tenor.com/view/rust-femboy-rust-femboy-programming-rust-programming-gif-27321790

#[entry]
fn main() -> ! {

    let num=3;
    match num{
     0=>{SOURCE = 0b00000001;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},   
     1=>{SOURCE = 0b00000010;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     2=>{SOURCE = 0b00000100;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     3=>{SOURCE = 0b00001000;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     4=>{SOURCE = 0b00010000;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     5=>{SOURCE = 0b00100000;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     6=>{SOURCE = 0b01000000;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     7=>{SOURCE = 0b10000000;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     8=>{SOURCE = 0b00000001;CHOICED = DDRB; CHOICEP = PORTB; CHOICEPIN = PINB;},
     9=>{SOURCE = 0b00000010;CHOICED = DDRB; CHOICEP = PORTB; CHOICEPIN = PINB;},
     10=>{SOURCE = 0b00000100;CHOICED = DDRB; CHOICEP = PORTB; CHOICEPIN = PINB;},
     11=>{SOURCE = 0b00001000;CHOICED = DDRB; CHOICEP = PORTB; CHOICEPIN = PINB;},
     12=>{SOURCE = 0b00010000;CHOICED = DDRB; CHOICEP = PORTB; CHOICEPIN = PINB;},
     13=>{SOURCE = 0b00100000;CHOICED = DDRB; CHOICEP = PORTB; CHOICEPIN = PINB;},
     _=>println!("Raté"),
    }
    loop {
        unsafe {
        // Infinite loop
            loop {
            // Turn on the LED (set bit 5 in PORTB)
                core::ptr::write_volatile(CHOICED,SOURCE);
            /*loop {
                for _ in 0..1_000_000 
                {
                    core::ptr::write_volatile(PORTB, 0b00100000);
                }
                for _ in 0..100_000
                {
                    core::ptr::write_volatile(PORTB, 0b00000000);
                }
            }*/
                let pinb = core::ptr::read_volatile(PINB);
                let port13_high = pinb & 0b00100000; // Vérifier si PB5 (bit 5) est à 1

                if port13_high != 0 {
                    // Si port 13 est HIGH, mettre port 12 (PB4) à HIGH
                    core::ptr::write_volatile(CHOICEP, SOURCE); // PB4 = 1
                } else {
                    // Sinon, mettre port 12 (PB4) à LOW
                    core::ptr::write_volatile(CHOICEP, 0b00000000); // PB4 = 0
                }
            }
        }
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}