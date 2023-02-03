#![feature(asm_experimental_arch)]
#![allow(dead_code)]
#![no_std]
#![no_main]

use core::str::from_utf8_unchecked;

use atmega4809_hal::clock::{self, ClockPrescaler, ClockSelect};
use atmega4809_hal::gpio::{GPIO, ISC};
use atmega4809_hal::i2c::I2C;
use atmega4809_hal::usart::{BAUD9600, USART, USART1, USART3};
use atmega4809_hal::{Delay, DelayMs};
use ufmt::uwrite;

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
    uwrite!(STDOUT, "Idling...\r\n").unwrap();

    Ble::off();
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

    BLE_POWER.output_enable();
    BLE_POWER.pin_ctrl_pullup(false);
    BLE_POWER.pin_ctrl_isc(&ISC::IntDisable);
    BLE_POWER.output_low();

    ANALOG_DRDY.output_disable();
    ANALOG_DRDY.pin_ctrl_pullup(false);
    ANALOG_DRDY.pin_ctrl_isc(&ISC::IntDisable);

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
    //GPIO::PORTC(4).pin_ctrl_pullup(true);

    uwrite!(STDOUT, "Startup complete.\r\n").unwrap();
    let mut k = [0; 10];
    let res = Stdout::transact(b"Write: ", &mut k).unwrap();
    uwrite!(STDOUT, "nuum chars: {}\r\n", res.len()).unwrap();

    ble_begin();
    loop {
        send_ble_terminal_heartbeat();
    }
}

pub fn run_bt_command() {
    todo!()
}

fn ble_begin() {
    //Delay.delay_ms(200);
    //BLE_KEY.output_high();
    //BLE_POWER.output_high();

    //Delay.delay_ms(200);

    //BLE_POWER.output_low();
    BLE_KEY.output_low();
    BLE_POWER.output_high();
    Ble::change_baud(BAUD9600);
    Delay.delay_ms(200);

    // Assume \r\n:
    //
    // AT+ROLE=0 -> OK
    // AT+PSWD=1234 -> OK
    // AT+NAME=Rocketry Test Bench -> OK
    // AT+BIND=ACD6,18,E95B5F
}

fn send_ble_terminal_heartbeat() {
    //loop {
    //let mut k = [0; 10];
    //let res = Stdout::transact(b"Write: ", &mut k).unwrap();
    //if res.len() > 0 {
    //break;
    //}
    //}

    ONBOARD_LED.output_high();
    let mut k = [0u8; 10];
    match Ble::transact(b"ping\r\n", &mut k) {
        Ok(d) => {
            uwrite!(STDOUT, "-{}\r\n", d.len()).unwrap();
            let d = unsafe { from_utf8_unchecked(d) };
            uwrite!(STDOUT, "$$$${}\r\n", d).unwrap();
        }
        Err(_) => {
            uwrite!(STDOUT, "+{}\r\n", k[0]).unwrap();
        }
    }
    ONBOARD_LED.output_low();
    Delay.delay_ms(500);
}
