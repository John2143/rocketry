#![feature(asm_experimental_arch)]
#![feature(default_alloc_error_handler)]
#![allow(dead_code)]
#![no_std]
#![no_main]

extern crate alloc;

pub mod avr_alloc;
pub mod ints;
pub mod testing;

use core::convert::TryInto;

use atmega4809_hal::clock::{ClockPrescaler, ClockSelect};
use atmega4809_hal::gpio::{GPIO, ISC};
use atmega4809_hal::i2c::I2C;
use atmega4809_hal::pwm::PWM;
use atmega4809_hal::usart::USART;
use atmega4809_hal::Delay;
use avr_alloc::AVRAlloc;

use icm20948::ICMI2C;
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
        nau7802::Gain::G128,
        nau7802::SamplesPerSecond::SPS80,
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

        ufmt::uwrite!(USART, "{}\r\n", s).unwrap();
        // 0 - 200_000 -> 0x1000 - 0x1800
        let sanatized = match s + 100_000 {
            i32::MIN..=0 => 0,
            v @ 1..=200_000 => v,
            200_001..=i32::MAX => 200_000,
        };

        PWM::set_cmp1(((sanatized * 0x800) / 200_000 + 0x1000).try_into().unwrap());
        //sleep(0xff00);
    }
}

fn setup_pwm() {
    GPIO::PORTB(1).output_enable();
    GPIO::PORTB(1).pin_ctrl_isc(&ISC::IntDisable);
    PWM::change_port_tca(atmega4809_hal::pwm::PWMPort::PORTB);
    PWM::set_per(0xAF00); //60hz
    PWM::enable(atmega4809_hal::pwm::WaveformGenerationMode::SINGLESLOPE);
    PWM::set_cmp1(32);
}

fn setup_usart() {
    USART::setup(
        ((17 << 6) | 0b0001_1000) / 6, //9600
        atmega4809_hal::usart::CommunicationMode::Asynchronous,
        atmega4809_hal::usart::ParityMode::Disabled,
        atmega4809_hal::usart::StopBitMode::One,
        atmega4809_hal::usart::CharacterSize::B8,
    );
}

fn test_pwm() {
    setup_pwm();
    loop {
        for x in (0x1000..0x1400).step_by(0x10) {
            PWM::set_cmp1(x);
            sleep(0xFFFF);
        }
    }
}

fn test_icm() {
    let icm: ICMI2C<I2C, atmega4809_hal::i2c::I2CError, 0x77> = ICMI2C::new(&mut I2C).unwrap();
    loop {
        let v = icm.get_values_accel_gyro(&mut I2C).unwrap();
        PWM::set_cmp1((v.0 + v.1 + v.2 + v.3 + v.4 + v.5) as u16);
    }
}

pub fn t() {
    GPIO::PORTA(1).output_enable();
    GPIO::PORTA(1).output_high();
}

include!(concat!(env!("OUT_DIR"), "/hello.rs"));

#[no_mangle]
pub fn main() -> ! {
    real_main();
    loop {}
}

pub fn real_main() {
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
    setup_pwm();
    setup_usart();

    test_nau()
}
