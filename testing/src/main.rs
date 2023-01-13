#![feature(asm_experimental_arch)]
#![feature(default_alloc_error_handler)]
#![allow(dead_code)]
#![no_std]
#![no_main]

extern crate alloc;

pub mod avr_alloc;
pub mod ints;
pub mod nau7802;
pub mod testing;

use alloc::boxed::Box;
use atmega4809_hal::clock::{ClockPrescaler, ClockSelect};
use atmega4809_hal::gpio::{GPIO, ISC};
use atmega4809_hal::i2c::I2C;
use atmega4809_hal::i2c::RW::DirWrite;
use avr_alloc::AVRAlloc;

use nau7802::Nau7802;
use panic_halt as _;
use testing::sleep;

#[global_allocator]
static ALLOCATOR: AVRAlloc = AVRAlloc::new();

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

    Nau7802::setup();

    let mut first: Option<u32> = None;
    loop {
        led.output_high();
        loop {
            match Nau7802.data_available() {
                Ok(true) => break,
                Ok(false) => sleep(20),
                Err(_) => {}
            }
        }
        led.output_low();

        match Nau7802.read_unchecked_m() {
            Ok(v) => {
                let s: u32 = (v[0] as u32) << 16 | (v[1] as u32) << 8 | v[2] as u32;
                let first = match first {
                    Some(f) => f,
                    None => {
                        first = Some(s);
                        s
                    }
                };

                //let s = (s).abs_diff(first);
                let s = (s / 1000) % 100;
                for _ in 0..1000 {
                    testing::duty_cycle(s as u8);
                }
            }
            Err(_) => {}
        };
        sleep(400);
    }

    //testing::blink_led();
}
