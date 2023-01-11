#![feature(asm_experimental_arch)]
#![allow(dead_code)]
#![no_std]
#![no_main]

pub mod gpio;
pub mod i2c;
pub mod testing;

use gpio::GPIO;
use panic_halt as _;

#[no_mangle]
pub fn main() -> ! {
    let led = GPIO::PORTE(2);
    led.output_enable();
    led.pin_ctrl_isc(&gpio::ISC::IntDisable);
    led.output_high();

    let bus = i2c::I2CBus::new(GPIO::PORTA(3), GPIO::PORTA(2));
    bus.enable();
    bus.test();

    testing::blink_led();
}
