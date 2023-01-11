#![feature(asm_experimental_arch)]
#![allow(dead_code)]
#![no_std]
#![no_main]

mod gpio;

use panic_halt as _;

#[no_mangle]
pub extern "C" fn main() -> ! {
    direct_led()
}

pub fn duty_cycle(brightness: u8) {
    let led = gpio::GPIO::PORTE(2);
    unsafe {
        if brightness > 0 {
            led.output_high()
        }
        for _ in 0..(brightness) {
            core::arch::asm!("nop");
        }
        if brightness < 100 {
            led.output_low()
        }
        for _ in 0..(100 - brightness) {
            core::arch::asm!("nop");
        }
    }
}

pub fn direct_led() -> ! {
    //https://docs.arduino.cc/static/90c04d4cfb88446cafa299787bf06056/ABX00028-pinout.png

    let led = gpio::GPIO::PORTE(2);
    led.pin_ctrl_isc(&gpio::ISC::InputDisable);
    led.output_enable();

    loop {
        for x in 0..255 {
            for _ in 0..5 {
                duty_cycle((x / 3) as u8);
            }
        }

        for x in (0..255).rev() {
            for _ in 0..2 {
                duty_cycle((x / 3) as u8);
            }
        }
    }
}
