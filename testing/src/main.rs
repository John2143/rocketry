#![feature(asm_experimental_arch)]
#![feature(default_alloc_error_handler)]
#![allow(dead_code)]
#![no_std]
#![no_main]

extern crate alloc;

pub mod avr_alloc;
//pub mod bme280;
pub mod ints;
pub mod testing;

use atmega4809_hal::clock::{self, ClockPrescaler, ClockSelect};
use atmega4809_hal::gpio::{GPIO, ISC};
use atmega4809_hal::i2c::I2C;
use atmega4809_hal::pwm::PWM;
use atmega4809_hal::usart::{USART, USART1, USART3, BAUD9600};
use atmega4809_hal::Delay;
use avr_alloc::AVRAlloc;
use embedded_hal::delay::blocking::DelayUs;

//use icm20948::ICMI2C;
use testing::sleep;

#[global_allocator]
static ALLOCATOR: AVRAlloc = AVRAlloc::new();

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let _ = ufmt::uwrite!(STDOUT, "PANIC!\r\n");
    let l = _info.location().unwrap().line();
    let f = _info.location().unwrap().file();
    let _ = ufmt::uwrite!(STDOUT, "{} {}\r\n", l, f);
    loop {
        ONBOARD_LED.output_high();
        //poweroff
        BRIGHT_LED.output_high();
        clock::Sleep::Idle.set_sleep();
        ONBOARD_LED.output_low();
    }
}

type Stdout = USART<USART3, true>;
const STDOUT: Stdout = USART;

type Ble = USART<USART1, true>;
const BLE: Ble = USART;

const ONBOARD_LED: GPIO = GPIO::PORTE(2);
const BRIGHT_LED: GPIO = GPIO::PORTA(0);
const PWM_PIN: GPIO = GPIO::PORTB(1);

fn test_nau() {
    //let mut v = nau7802::Nau7802::new_with_settings(
        //I2C,
        //nau7802::Ldo::L3v3,
        //nau7802::Gain::G128,
        //nau7802::SamplesPerSecond::SPS80,
        //&mut Delay,
    //)
    //.unwrap();

    //loop {
        //let s = loop {
            //match v.read() {
                //Ok(v) => break v,
                //Err(_) => sleep(10),
            //};
        //};

        //ufmt::uwrite!(STDOUT, "{}\r\n", s).unwrap();
        //// 0 - 200_000 -> 0x1000 - 0x1800
        //let sanatized = match s + 100_000 {
            //i32::MIN..=0 => 0,
            //v @ 1..=200_000 => v,
            //200_001..=i32::MAX => 200_000,
        //};

        //PWM::set_cmp1(((sanatized * 0x800) / 200_000 + 0x1000).try_into().unwrap());
        //sleep(0xff00);
    //}
}

fn test_bme() {
    //let b = bme280::i2c::BME280::new(I2C, 0x77);
    let mut bme = bme280::i2c::BME280::new_secondary(I2C);

    // or, initialize the BME280 using the secondary I2C address 0x77
    // let mut bme280 = BME280::new_secondary(i2c_bus, Delay);

    // or, initialize the BME280 using a custom I2C address
    // let bme280_i2c_addr = 0x88;
    // let mut bme280 = BME280::new(i2c_bus, bme280_i2c_addr, Delay);

    ufmt::uwrite!(STDOUT, "BME Init Start\r\n").unwrap();
    // initialize the sensor
    bme.init(&mut Delay).unwrap();
    ufmt::uwrite!(STDOUT, "BME Init Complete, starting base measure\r\n").unwrap();

    Delay.delay_ms(1).unwrap();

    // measure temperature, pressure, and humidity
    let measurements = match bme.measure(&mut Delay) {
        Ok(m) => m,
        Err(bme280::Error::CompensationFailed) => {
            ufmt::uwrite!(STDOUT, "BME Compensation Failed\r\n").unwrap();
            return;
        },
        Err(bme280::Error::Bus(_)) => {
            ufmt::uwrite!(STDOUT, "BME BUS\r\n").unwrap();
            return;
        },
        Err(bme280::Error::InvalidData) => {
            ufmt::uwrite!(STDOUT, "BME Invalid Data\r\n").unwrap();
            return;
        },
        Err(bme280::Error::NoCalibrationData) => {
            ufmt::uwrite!(STDOUT, "BME No Calibration Data\r\n").unwrap();
            return;
        },
        Err(bme280::Error::UnsupportedChip) => {
            ufmt::uwrite!(STDOUT, "BME Unsupported Chip\r\n").unwrap();
            return;
        },
        Err(bme280::Error::Delay) => {
            ufmt::uwrite!(STDOUT, "BME Delay\r\n").unwrap();
            return;
        },
    };

    ufmt::uwrite!(STDOUT, "Relative Humidity = {}%", measurements.humidity as u32).unwrap();
    ufmt::uwrite!(STDOUT, "Temperature = {} deg C", measurements.temperature as u32).unwrap();
    ufmt::uwrite!(STDOUT, "Pressure = {} pascals", measurements.pressure as u32).unwrap();
}

fn setup_pwm() {
    PWM_PIN.output_enable();
    PWM_PIN.pin_ctrl_isc(&ISC::IntDisable);
    PWM::change_port_tca(atmega4809_hal::pwm::PWMPort::PORTB); // pin 28
    //PWM::set_per(69); //38.13khz (IR transmission = 38khz)
    PWM::set_per(0xAF00); //60hz
    PWM::enable(atmega4809_hal::pwm::WaveformGenerationMode::SINGLESLOPE);
    PWM::set_cmp1(0xAF00 / 2); //60hz
}

fn setup_usart() {
    Stdout::setup(
        BAUD9600 / 12, //9600 / 6 = 57200
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

//fn test_icm() {
    //ufmt::uwrite!(STDOUT, "Starting to measure {:x}\r\n", 0x69).unwrap();
    //let icm: ICMI2C<I2C, atmega4809_hal::i2c::I2CError, 0x69> = ICMI2C::new(&mut I2C).unwrap();
    //loop {
        //let v = icm.get_values_accel_gyro(&mut I2C).unwrap();

        //ufmt::uwrite!(STDOUT, "{:?}\r\n", v).unwrap();
    //}
//}

fn test_ble() {
    Stdout::transact(b"Starting BLE Tets\r\n", &mut []);

    //let d = data_back.into_iter().take_while(|c| c != b'\n').collect();
    let mut data_back = [0; 10];
    Ble::transact(b"AT\r\n", &mut data_back);

    ufmt::uwrite!(STDOUT, "Ok\r\n").unwrap();
    //ufmt::uwrite!(STDOUT, "Got back some data: '{}'\r\n", d).unwrap();
}

fn test_spi() {
    use atmega4809_hal::spi;
    use spi::SPI;
    ufmt::uwrite!(STDOUT, "Starting SPI...\r\n").unwrap();
    SPI::setup(false, false, spi::Polarity::P0);
    ufmt::uwrite!(STDOUT, "SPI Setup Complete\r\n").unwrap();
    let mut test = [1, 2, 3, 4u8];
    //match SPI.transfer(&mut test) {
        //Ok(v) => {
            //ufmt::uwrite!(STDOUT, "OK\r\n").unwrap();
        //}
        //Err(e) => {
            //ufmt::uwrite!(STDOUT, "Err {:?}\r\n", e).unwrap();
        //}
    //}
    ufmt::uwrite!(STDOUT, "Transfer Complete\r\n").unwrap();
}

fn test_imu() {
    ufmt::uwrite!(STDOUT, "Yo...\r\n").unwrap();
}

#[no_mangle]
pub fn main() -> ! {
    sleep(0xff);
    real_main();
    // Wait for any peripherials to finish.
    for _ in 0..10 {
        sleep(0xffff);
    }

    let _ = ufmt::uwrite!(STDOUT, "Idling...\r\n");

    //Ble::off();
    Stdout::off();
    clock::Sleep::Idle.set_sleep();
    loop {}
}

pub fn real_main() {
    ClockSelect::OSC20M.set_clock();
    //ClockSelect::OSCULP32K.set_clock();
    ClockPrescaler::None.set_clock_prescaler();

    ONBOARD_LED.output_enable();
    ONBOARD_LED.pin_ctrl_isc(&ISC::IntDisable);
    BRIGHT_LED.output_enable();
    BRIGHT_LED.pin_ctrl_isc(&ISC::IntDisable);
    ONBOARD_LED.output_low();
    BRIGHT_LED.output_low();

    I2C::setup();
    //setup_pwm();
    //test_pwm();
    setup_usart();

    //let mut x = alloc::vec::Vec::new();
    //x.push(2u8);
    //let ptr = x.as_ptr() as u16;
    //let mut x2 = alloc::vec![1u8;128];
    //let ptr2 = x2.as_ptr() as u16;

    // Wait for all systems/clocks to update just in case

    // Test Floats
    let s = 3.0;
    for i in 0..10 {
        ufmt::uwrite!(STDOUT, "Counting to 10: {}\r\n", i).unwrap();
        Delay.delay_ms(1000).unwrap();
    }

    //ufmt::uwrite!(STDOUT, "Nice\r\n").unwrap();
    //ufmt::uwrite!(STDOUT, "Test Heaps: 1=0x{:x} 2=0x{:x}\r\n", ptr, ptr2).unwrap();
    //ufmt::uwrite!(STDOUT, "Test Heaps: 1=0x{:x} 2=0x{:x}\r\n", 1, 2).unwrap();

    ufmt::uwrite!(STDOUT, "Startup complete...\r\n").unwrap();

    //test_bme();
    //test_ble();
    //test_nau();
    //test_icm();
    //BRIGHT_LED.output_high();
    //test_spi();
    //test_imu();
}
