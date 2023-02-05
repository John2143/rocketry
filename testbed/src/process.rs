use crate::{Ble, ANALOG_DRDY, BLE, BLE_KEY, BLE_POWER, STDOUT};
use atmega4809_hal::{i2c::I2C, usart::BAUD9600, Delay, DelayMs};
use core::str::from_utf8_unchecked;
use nau7802::Nau7802;
use ufmt::uwrite;

pub fn read_nau(n: &mut Nau7802<I2C>) -> Result<i32, ()> {
    for _ in 0..10000 {
        if !ANALOG_DRDY.input_read() {
            continue;
        }
        match n.read() {
            Ok(v) => return Ok(v),
            Err(_) => {} //sleep(10),
        };
    }
    Err(())
}

#[derive(Debug)]
pub enum FatalStartupError {
    NoSensor,
    CalibrationFailure(&'static str),
}

pub type Nau = Nau7802<I2C>;

pub fn nau_setup() -> Result<Nau, FatalStartupError> {
    let mut v = Nau7802::new_with_settings(
        I2C,
        nau7802::Ldo::L3v9,
        nau7802::Gain::G128,
        nau7802::SamplesPerSecond::SPS80,
        &mut Delay,
    )
    .map_err(|_| FatalStartupError::NoSensor)?;

    let _ = uwrite!(STDOUT, "Calibrating.");
    v.begin_afe_calibration()
        .map_err(|_| FatalStartupError::CalibrationFailure("nau start"))?;
    loop {
        match v.poll_afe_calibration_status().unwrap() {
            nau7802::AfeCalibrationStatus::InProgress => {
                let _ = uwrite!(STDOUT, ".");
                Delay.delay_ms(1);
            }
            nau7802::AfeCalibrationStatus::Failure => {
                let _ = uwrite!(STDOUT, "rip\r\n");
                return Err(FatalStartupError::CalibrationFailure("nau"));
            }
            nau7802::AfeCalibrationStatus::Success => {
                let _ = uwrite!(STDOUT, "success\r\n");
                break;
            }
        };
    }

    for _ in 0..10 {
        read_nau(&mut v).unwrap();
    }

    Ok(v)
}

pub fn nau_run(n: &mut Nau7802<I2C>) {
    let first = read_nau(n).unwrap();

    loop {
        let s = read_nau(n).unwrap();
        if s.abs_diff(first) > 1000 {
            break;
        }
    }

    ufmt::uwriteln!(STDOUT, "Triggered.\r\n").unwrap();

    for _ in 0..1000 {
        let s = read_nau(n).unwrap();
        let s = s - first;

        //ufmt::uwrite!(STDOUT, "{}\r\n", s).unwrap();
        ufmt::uwrite!(BLE, "{}\r\n", s).unwrap();
        //let mut k = [0u8; 10];
        //let k = Ble::transact(b"", &mut k).unwrap();
    }
}

pub fn ble_begin() {
    //Delay.delay_ms(200);
    //BLE_KEY.output_high();
    //BLE_POWER.output_high();

    //Delay.delay_ms(200);

    //BLE_POWER.output_low();
    BLE_KEY.output_low();
    BLE_POWER.output_high();
    Ble::change_baud(BAUD9600 / 12);
    //Delay.delay_ms(200);

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
    Delay.delay_ms(500);
}
