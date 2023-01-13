use core::iter;

use atmega4809_hal::{
    gpio::GPIO,
    i2c::{I2CError, I2C},
};

use crate::testing::sleep;

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Register {
    PuCtrl = 0x00,
    Ctrl1,
    Ctrl2,
    Ocal1B2,
    Ocal1B1,
    Ocal1B0,
    Gcal1B3,
    Gcal1B2,
    Gcal1B1,
    Gcal1B0,
    Ocal2B2,
    Ocal2B1,
    Ocal2B0,
    Gcal2B3,
    Gcal2B2,
    Gcal2B1,
    Gcal2B0,
    I2CControl,
    AdcoB2,
    AdcoB1,
    AdcoB0,
    Adc = 0x15, // Shared ADC and OTP 32:24
    OtpB1,      // OTP 23:16 or 7:0?
    OtpB0,      // OTP 15:8
    Pga = 0x1B,
    PgaPwr = 0x1C,
    DeviceRev = 0x1F,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum PuCtrlBits {
    RR = 0,
    PUD,
    PUA,
    PUR,
    CS,
    CR,
    OSCS,
    AVDDS,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum PgaRegisterBits {
    ChpDis = 0,
    Inv = 3,
    BypassEn,
    OutEn,
    LdoMode,
    RdOtpSel,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum PgaPwrRegisterBits {
    Curr = 0,
    AdcCurr = 2,
    MstrBiasCurr = 4,
    CapEn = 7,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Ctrl2RegisterBits {
    CalMod = 0,
    Cals = 2,
    CalError = 3,
    Crs = 4,
    Chs = 7,
}

pub trait RegisterBits {
    fn get(&self) -> u8;
}

macro_rules! impl_register_bits {
    ($($type:ident),*) => {
        $(
            impl RegisterBits for $type {
                fn get(&self) -> u8 {
                    *self as _
                }
            }
        )*
    }
}

impl_register_bits!(
    PuCtrlBits,
    PgaRegisterBits,
    PgaPwrRegisterBits,
    Ctrl2RegisterBits
);

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Ldo {
    L2v4 = 0b111,
    L2v7 = 0b110,
    L3v0 = 0b101,
    L3v3 = 0b100,
    L3v6 = 0b011,
    L3v9 = 0b010,
    L3v2 = 0b001,
    L4v5 = 0b000,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Gain {
    G128 = 0b111,
    G64 = 0b110,
    G32 = 0b101,
    G16 = 0b100,
    G8 = 0b011,
    G4 = 0b010,
    G2 = 0b001,
    G1 = 0b000,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum SamplesPerSecond {
    SPS320 = 0b111,
    SPS80 = 0b011,
    SPS40 = 0b010,
    SPS20 = 0b001,
    SPS10 = 0b000,
}

#[derive(PartialEq)]
pub enum AfeCalibrationStatus {
    InProgress,
    Failure,
    Success,
}

pub struct Nau7802;

impl Nau7802 {
    fn read<const N: usize>(&mut self, address: u8) -> Result<[u8; N], I2CError> {
        I2C::write(0x2a, &[address])?;
        I2C::read::<N>(0x2a)
    }
    fn read1(&mut self, address: u8) -> Result<u8, I2CError> {
        I2C::write(0x2a, &[address])?;
        Ok(I2C::read::<1>(0x2a)?[0])
    }

    fn write(&mut self, address: u8, data: u8) -> Result<(), I2CError> {
        I2C::write(0x2a, &[address, data])?;
        Ok(())
    }

    fn set_bit<B: RegisterBits>(&mut self, addr: Register, bit_idx: B) -> Result<(), I2CError> {
        let mut val = self.read1(addr as u8)?;
        val |= 1 << bit_idx.get();
        self.write(addr as u8, val)
    }

    fn clear_bit<B: RegisterBits>(&mut self, addr: Register, bit_idx: B) -> Result<(), I2CError> {
        let mut val = self.read1(addr as u8)?;
        val &= !(1 << bit_idx.get());
        self.write(addr as u8, val)
    }

    fn get_bit<B: RegisterBits>(&mut self, addr: Register, bit_idx: B) -> Result<bool, I2CError> {
        let mut val = self.read1(addr as u8)?;
        val &= 1 << bit_idx.get();
        Ok(val != 0)
    }

    pub fn power_up(&mut self) -> Result<(), I2CError> {
        const NUM_ATTEMPTS: usize = 100;

        self.set_bit(Register::PuCtrl, PuCtrlBits::PUD)?;
        self.set_bit(Register::PuCtrl, PuCtrlBits::PUA)?;

        let check_powered_up = || self.get_bit(Register::PuCtrl, PuCtrlBits::PUR);

        let powered_up = iter::repeat_with(check_powered_up)
            .take(NUM_ATTEMPTS)
            .filter_map(Result::ok)
            .any(|rdy| rdy == true);

        if powered_up {
            Ok(())
        } else {
            Err(I2CError::NACK)
        }
    }

    pub fn set_ldo(&mut self, ldo: Ldo) -> Result<(), I2CError> {
        const LDO_MASK: u8 = 0b11000111;
        const LDO_START_BIT: u8 = 3;

        self.set_function_helper(Register::Ctrl1, LDO_MASK, LDO_START_BIT, ldo as _)?;

        self.set_bit(Register::PuCtrl, PuCtrlBits::AVDDS)
    }

    fn set_function_helper(
        &mut self,
        reg: Register,
        mask: u8,
        start_idx: u8,
        new_val: u8,
    ) -> Result<(), I2CError> {
        let mut val = self.read1(reg as u8)?;
        val &= mask;
        val |= new_val << start_idx;

        self.write(reg as u8, val)
    }

    pub fn setup() {
        let led = GPIO::PORTE(2);
        let led2 = GPIO::PORTD(3);

        match Self.set_bit(Register::PuCtrl, PuCtrlBits::RR) {
            Ok(_) => {}
            Err(_) => led2.output_high(),
        }

        sleep(100);

        match Self.clear_bit(Register::PuCtrl, PuCtrlBits::RR) {
            Ok(_) => {}
            Err(_) => led2.output_high(),
        }

        match Self.power_up() {
            Ok(_) => {}
            Err(_) => led2.output_high(),
        }

        match Self.set_sample_rate(SamplesPerSecond::SPS80) {
            Ok(_) => {}
            Err(_) => led2.output_high(),
        }
        match Self.set_gain(Gain::G128) {
            Ok(_) => {}
            Err(_) => led2.output_high(),
        }
        match Self.set_ldo(Ldo::L3v6) {
            Ok(_) => {}
            Err(_) => led2.output_high(),
        }

        const TURN_OFF_CLK_CHPL: u8 = 0x30;

        // Turn off CLK_CHP. From 9.1 power on sequencing
        Self.write(Register::Adc as u8, TURN_OFF_CLK_CHPL);

        // Enable 330pF decoupling cap on chan 2. From 9.14 application circuit note
        Self.set_bit(Register::PgaPwr, PgaPwrRegisterBits::CapEn);

        Self.begin_afe_calibration();

        while let Ok(AfeCalibrationStatus::InProgress) = Self.poll_afe_calibration_status() {}
    }

    pub fn begin_afe_calibration(&mut self) -> Result<(), I2CError> {
        self.set_bit(Register::Ctrl2, Ctrl2RegisterBits::Cals)
    }

    pub fn poll_afe_calibration_status(&mut self) -> Result<AfeCalibrationStatus, I2CError> {
        if self.get_bit(Register::Ctrl2, Ctrl2RegisterBits::Cals)? {
            return Ok(AfeCalibrationStatus::InProgress);
        }

        if self.get_bit(Register::Ctrl2, Ctrl2RegisterBits::CalError)? {
            return Ok(AfeCalibrationStatus::Failure);
        }

        Ok(AfeCalibrationStatus::Success)
    }

    pub fn set_gain(&mut self, gain: Gain) -> Result<(), I2CError> {
        const GAIN_MASK: u8 = 0b11111000;
        const GAIN_START_BIT: u8 = 0;

        self.set_function_helper(Register::Ctrl1, GAIN_MASK, GAIN_START_BIT, gain as _)
    }

    pub fn set_sample_rate(&mut self, sps: SamplesPerSecond) -> Result<(), I2CError> {
        const SPS_MASK: u8 = 0b10001111;
        const SPS_START_BIT_IDX: u8 = 4;

        self.set_function_helper(Register::Ctrl2, SPS_MASK, SPS_START_BIT_IDX, sps as _)
    }

    pub fn read_unchecked_s(&mut self) -> Result<[u8; 3], I2CError> {
        let buf = [
            self.read::<1>(Register::AdcoB2 as u8)?[0],
            self.read::<1>(Register::AdcoB2 as u8 + 1)?[0],
            self.read::<1>(Register::AdcoB2 as u8 + 2)?[0],
        ];

        Ok(buf)
    }

    pub fn read_unchecked_m(&mut self) -> Result<[u8; 3], I2CError> {
        self.read::<3>(Register::AdcoB2 as u8)
    }

    pub fn data_available(&mut self) -> Result<bool, I2CError> {
        self.get_bit(Register::PuCtrl, PuCtrlBits::CR)
    }
}
