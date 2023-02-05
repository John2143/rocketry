#![feature(asm_experimental_arch)]
#![allow(dead_code)]
#![no_std]
#![no_main]

mod process;

use atmega4809_hal::clock::{self, ClockPrescaler, ClockSelect};
use atmega4809_hal::gpio::{GPIO, ISC};
use atmega4809_hal::i2c::I2C;
use atmega4809_hal::usart::{BAUD9600, USART, USART1, USART3};
use ufmt::uwrite;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    //let _ = uwrite!(STDOUT, "\r\nPanic!\r\n");
    //if let Some(loc) = _info.location() {
    //let _ = uwrite!(STDOUT, "File: {}\r\nLine: {}\r\n", loc.file(), loc.line());
    //}
    loop {
        ONBOARD_LED.output_high();
        //poweroff
        clock::Sleep::Idle.set_sleep();
    }
}

type Stdout = USART<USART3, true>;
const STDOUT: Stdout = USART;

type Ble = USART<USART1, true>;
const BLE: Ble = USART;

const ANALOG_DRDY: GPIO = GPIO::PORTD(0);
const BLE_STATE: GPIO = GPIO::PORTF(4);
const BLE_KEY: GPIO = GPIO::PORTD(1);
const ONBOARD_LED: GPIO = GPIO::PORTE(2);
const BLE_POWER: GPIO = GPIO::PORTD(2);

const BLE_RX: GPIO = GPIO::PORTC(5);
const BLE_TX: GPIO = GPIO::PORTC(4);

fn setup_usart() {
    Stdout::setup(
        BAUD9600 / 6, //9600 / 6 = 57200
        atmega4809_hal::usart::CommunicationMode::Asynchronous,
        atmega4809_hal::usart::ParityMode::Disabled,
        atmega4809_hal::usart::StopBitMode::One,
        atmega4809_hal::usart::CharacterSize::B8,
    );

    Ble::setup(
        BAUD9600 / 4, //9600 / 4 = 37600
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
    let _ = uwrite!(STDOUT, "Idling...\r\n");

    Ble::off();
    Stdout::off();
    clock::Sleep::Idle.set_sleep();
    loop {}
}

pub fn real_main() {
    //default settings
    ClockSelect::OSC20M.set_clock();
    ClockPrescaler::D6.set_clock_prescaler();

    setup_usart();
    I2C::setup();

    BLE_POWER.output_enable();
    BLE_POWER.pin_ctrl_pullup(false);
    BLE_POWER.pin_ctrl_isc(&ISC::IntDisable);
    BLE_POWER.output_low();

    ANALOG_DRDY.output_disable();
    ANALOG_DRDY.pin_ctrl_pullup(false);
    ANALOG_DRDY.pin_ctrl_isc(&ISC::Rising);

    BLE_STATE.output_disable();
    BLE_STATE.pin_ctrl_pullup(false);
    BLE_STATE.pin_ctrl_isc(&ISC::IntDisable);

    BLE_KEY.output_enable();
    BLE_KEY.pin_ctrl_pullup(false);
    BLE_KEY.pin_ctrl_isc(&ISC::IntDisable);

    ONBOARD_LED.output_enable();
    ONBOARD_LED.pin_ctrl_isc(&ISC::IntDisable);
    ONBOARD_LED.output_low();

    BLE_RX.output_disable();
    BLE_RX.pin_ctrl_pullup(false);
    BLE_RX.pin_ctrl_isc(&ISC::IntDisable);

    // maybe need pullup when using serial?
    //GPIO::PORTC(4).output_enable();
    GPIO::PORTC(5).pin_ctrl_pullup(false);

    let _ = uwrite!(STDOUT, "Startup complete.\r\n");

    process::ble_begin();
    let mut nau = process::nau_setup().unwrap_or_else(|_| panic!("Nau setup failed"));
    process::nau_run(&mut nau);
}

#[no_mangle]
pub fn __vector_20() {
    //ONBOARD_LED.output_high();
}

#[no_mangle]
pub fn __vector_34() {
    ONBOARD_LED.output_high();
}

pub fn run_bt_command() {
    todo!()
}
