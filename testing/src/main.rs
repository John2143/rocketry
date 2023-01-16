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
use atmega4809_hal::pwm::PWM;
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

pub fn test_nau() {
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
                Err(_) => sleep(10),
            };
        };

        I2C::write(0x04, &[(s >> 8) as u8]).unwrap();
    }
}

include!(concat!(env!("OUT_DIR"), "/hello.rs"));

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

    GPIO::PORTB(1).output_enable();
    GPIO::PORTB(1).pin_ctrl_isc(&ISC::IntDisable);
    PWM::change_port_tca(atmega4809_hal::pwm::PWMPort::PORTB);
    PWM::set_per(0x5000);
    PWM::enable(atmega4809_hal::pwm::WaveformGenerationMode::SINGLESLOPE);
    PWM::set_cmp1(0x1500);
    loop {
        PWM::set_cmp1(0x1000);
        sleep(0xFFFF);
        sleep(0xFFFF);
        sleep(0xFFFF);
        PWM::set_cmp1(0x1600);
        sleep(0xFFFF);
        sleep(0xFFFF);
        sleep(0xFFFF);
    }
}
