#![feature(lang_items)]
#![no_std]
#![no_main]

//use ruduino::cores::current::port;
//use ruduino::Pin;

#[no_mangle]
pub fn main() {
    //port::B5::set_output();

    //loop {
    //port::B5::set_high();

    //ruduino::delay::delay_ms(1000);

    //port::B5::set_low();

    //ruduino::delay::delay_ms(1000);
    //}
}

#[panic_handler]
pub fn pan(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
