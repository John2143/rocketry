use crate::gpio::{GPIO, ISC};

pub struct I2CBus {
    scl: GPIO,
    sda: GPIO,
}

fn sleep_cycle(amt: usize) {
    for _ in 0..amt {
        unsafe { core::arch::asm!("nop") };
    }
}

impl I2CBus {
    pub fn new(scl: GPIO, sda: GPIO) -> Self {
        Self { scl, sda }
    }

    pub fn enable(&self) {
        GPIO::PORTA(3).pin_ctrl_pullup(true);
        GPIO::PORTA(3).pin_ctrl_isc(&ISC::IntDisable);
        GPIO::PORTA(3).output_high();
        GPIO::PORTA(3).output_enable();

        GPIO::PORTA(2).pin_ctrl_pullup(true);
        GPIO::PORTA(2).pin_ctrl_isc(&ISC::IntDisable);
        GPIO::PORTA(2).output_high();
        GPIO::PORTA(2).output_enable();
    }

    pub fn test(&self) {
        sleep_cycle(100);

        GPIO::PORTA(2).output_low();
        GPIO::PORTA(3).output_low();

        let mut byte = 0b0101_0101u8;

        for _ in 0..6 {
            let bit = byte & 1;
            byte >>= 1;
            if bit == 1 {
                GPIO::PORTA(2).output_high();
            } else {
                GPIO::PORTA(2).output_low();
            }
            GPIO::PORTA(3).output_high();
            GPIO::PORTA(3).output_low();
        }
        GPIO::PORTA(2).output_low();
        GPIO::PORTA(3).output_high();
        GPIO::PORTA(2).output_high();
    }
}
