#![feature(lang_items)]
#![no_std]
#![no_main]

//use ruduino::cores::atmega328p as avr_core;
//use ruduino::Register;

//use avr_core::{DDRB, PORTB};

#[no_mangle]
pub extern "C" fn main() {
    // Set all PORTB pins up as outputs
    //DDRB::set_mask_raw(0xFFu8);

    //loop {
    //// Set all pins on PORTB to high.
    //PORTB::set_mask_raw(0xFF);

    //small_delay();

    //// Set all pins on PORTB to low.
    //PORTB::unset_mask_raw(0xFF);

    //small_delay();
    //}
}

#[panic_handler]
pub fn pan(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
