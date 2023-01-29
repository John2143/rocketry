#![feature(asm_experimental_arch)]
#![allow(dead_code)]
#![no_std]
#![no_main]

use atmega4809_hal::clock::{self, ClockPrescaler, ClockSelect};
use atmega4809_hal::gpio::{GPIO, ISC};
use atmega4809_hal::i2c::I2C;
use atmega4809_hal::usart::{USART, USART3};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        ONBOARD_LED.output_high();
        //poweroff
        clock::Sleep::Idle.set_sleep();
        ONBOARD_LED.output_low();
    }
}

type Stdout = USART<USART3, true>;
const STDOUT: Stdout = USART;

const ANALOG_DRDY: GPIO = GPIO::PORTD(0);
const BLE_STATE: GPIO = GPIO::PORTF(4);
const BLE_KEY: GPIO = GPIO::PORTD(1);
const ONBOARD_LED: GPIO = GPIO::PORTE(2);

fn setup_usart() {
    Stdout::setup(
        ((17 << 6) | 0b0001_1000) / 6, //9600 / 6 = 57200
        atmega4809_hal::usart::CommunicationMode::Asynchronous,
        atmega4809_hal::usart::ParityMode::Disabled,
        atmega4809_hal::usart::StopBitMode::One,
        atmega4809_hal::usart::CharacterSize::B8,
    );
}

#[no_mangle]
pub fn main() -> ! {
    real_main();
    // Wait for any peripherials to finish.
    for _ in 0..0x100 {
        unsafe { core::arch::asm!("nop") };
    }
    ufmt::uwrite!(STDOUT, "Idling...\r\n").unwrap();

    //Ble::off();
    Stdout::off();
    clock::Sleep::Idle.set_sleep();
    loop {}
}

pub fn real_main() {
    ClockSelect::OSC20M.set_clock();
    //ClockSelect::OSCULP32K.set_clock();
    ClockPrescaler::None.set_clock_prescaler();
    setup_usart();
    I2C::setup();

    ANALOG_DRDY.output_disable();
    ANALOG_DRDY.pin_ctrl_pullup(false);
    ANALOG_DRDY.pin_ctrl_isc(&ISC::IntDisable);

    BLE_STATE.output_disable();
    BLE_STATE.pin_ctrl_pullup(false);
    BLE_STATE.pin_ctrl_isc(&ISC::IntDisable);

    BLE_KEY.output_enable();
    BLE_KEY.pin_ctrl_pullup(false);
    BLE_KEY.pin_ctrl_isc(&ISC::IntDisable);
    BLE_KEY.output_high();

    ONBOARD_LED.output_enable();
    ONBOARD_LED.pin_ctrl_isc(&ISC::IntDisable);
    ONBOARD_LED.output_low();

    ufmt::uwrite!(STDOUT, "Startup complete.\r\n").unwrap();

    loop {}
}
