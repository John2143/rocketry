#![feature(asm_experimental_arch)]
#![feature(default_alloc_error_handler)]
#![allow(dead_code)]
#![no_std]
#![no_main]

extern crate alloc;

pub mod avr_alloc;
pub mod ints;
pub mod testing;

use atmega4809_hal::clock::{ClockPrescaler, ClockSelect};
use atmega4809_hal::gpio::{GPIO, ISC};
use atmega4809_hal::i2c::I2C;
use atmega4809_hal::Delay;
use avr_alloc::AVRAlloc;

use testing::sleep;

#[global_allocator]
static ALLOCATOR: AVRAlloc = AVRAlloc::new();

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        GPIO::PORTE(2).output_high();
    }
}

#[no_mangle]
pub fn main() -> ! {
    ClockSelect::OSC20M.set_clock();
    //ClockSelect::OSCULP32K.set_clock();
    ClockPrescaler::None.set_clock_prescaler();

    let led = GPIO::PORTE(2);
    let led2 = GPIO::PORTD(3);
    led.output_enable();
    led.pin_ctrl_isc(&ISC::IntDisable);
    led2.output_enable();
    led2.pin_ctrl_isc(&ISC::IntDisable);

    //testing::blink_led();

    led.output_low();
    led2.output_low();
    I2C::setup();

    let mut v = nau7802::Nau7802::new_with_settings(
        I2C,
        nau7802::Ldo::L3v3,
        nau7802::Gain::G64,
        nau7802::SamplesPerSecond::SPS20,
        &mut Delay,
    )
    .unwrap();

    loop {
        let s = loop {
            match v.read() {
                Ok(v) => break v,
                Err(_) => {}
            };
        };

        I2C::write(0x04, &[(s >> 8) as u8]).unwrap();
        //let s = (s).abs_diff(first);
        let s = (s / 1000) % 100;
        for _ in 0..30 {
            testing::duty_cycle(s as u8);
        }
    }

    //testing::blink_led();
}
