use embedded_hal::digital::{blocking::OutputPin, ErrorType};

pub enum GPIO {
    PORTA(u8),
    PORTB(u8),
    PORTC(u8),
    PORTD(u8),
    PORTE(u8),
    PORTF(u8),
}

///Input/Sense Configuration
#[derive(Copy, Clone)]
pub enum ISC {
    ///Interrupt disabled but input buffer enabled
    IntDisable,
    ///Interrupt enabled with sense on both edges
    BothEdges,
    ///Interrupt enabled with sense on rising edge
    Rising,
    ///Interrupt enabled with sense on falling edge
    Falling,
    ///Interrupt and digital input buffer disabled
    InputDisable,
    ///Interrupt enabled with sense on low level
    Level,
}

impl ISC {
    fn val(self) -> u8 {
        match self {
            ISC::IntDisable => 0,
            ISC::BothEdges => 1,
            ISC::Rising => 2,
            ISC::Falling => 3,
            ISC::InputDisable => 4,
            ISC::Level => 5,
        }
    }
}
impl GPIO {
    fn base_ptr(&self) -> *mut u8 {
        (match self {
            GPIO::PORTA(_) => 0x0400,
            GPIO::PORTB(_) => 0x0420,
            GPIO::PORTC(_) => 0x0440,
            GPIO::PORTD(_) => 0x0460,
            GPIO::PORTE(_) => 0x0480,
            GPIO::PORTF(_) => 0x04A0,
        }) as *mut u8
    }

    pub fn pin(&self) -> u8 {
        *match self {
            GPIO::PORTA(a) => a,
            GPIO::PORTB(a) => a,
            GPIO::PORTC(a) => a,
            GPIO::PORTD(a) => a,
            GPIO::PORTE(a) => a,
            GPIO::PORTF(a) => a,
        }
    }

    pub fn output_enable(&self) {
        unsafe { self.base_ptr().offset(0x01).write_volatile(1 << self.pin()) }
    }

    pub fn output_disable(&self) {
        unsafe { self.base_ptr().offset(0x02).write_volatile(1 << self.pin()) }
    }

    pub fn output_high(&self) {
        unsafe { self.base_ptr().offset(0x05).write_volatile(1 << self.pin()) }
    }

    pub fn output_low(&self) {
        unsafe { self.base_ptr().offset(0x06).write_volatile(1 << self.pin()) }
    }

    pub fn output_toggle(&self) {
        unsafe { self.base_ptr().offset(0x07).write_volatile(1 << self.pin()) }
    }

    pub fn input_read(&self) -> bool {
        unsafe { self.base_ptr().offset(0x08).read_volatile() & (1 << self.pin()) > 0 }
    }

    pub fn int_flag_read(&self) -> bool {
        unsafe { self.base_ptr().offset(0x09).read_volatile() & (1 << self.pin()) > 0 }
    }

    pub fn int_flag_clear(&self) {
        unsafe { self.base_ptr().offset(0x09).write_volatile(1 << self.pin()) }
    }

    fn pin_ctrl(&self) -> *mut u8 {
        unsafe { self.base_ptr().offset(self.pin().into()).offset(0x10) }
    }

    pub fn pin_ctrl_invert(&self, b: bool) {
        unsafe {
            let v = if b { 1 } else { 0 };
            let old = self.pin_ctrl().read_volatile() & 0b0111_1111;
            self.pin_ctrl().write_volatile(old | (v << 7))
        }
    }

    pub fn pin_ctrl_pullup(&self, b: bool) {
        unsafe {
            let v = if b { 1 } else { 0 };
            let old = self.pin_ctrl().read_volatile() & 0b1111_0111;
            self.pin_ctrl().write_volatile(old | (v << 3))
        }
    }

    pub fn pin_ctrl_isc(&self, isc: &ISC) {
        unsafe {
            let old = self.pin_ctrl().read_volatile() & 0b1111_1000;
            self.pin_ctrl().write_volatile(old | isc.val())
        }
    }
}

impl OutputPin for GPIO {
    ///Make sure to enable the output mode for this pin
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.output_low();
        Ok(())
    }

    ///Make sure to enable the output mode for this pin
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.output_high();
        Ok(())
    }
}

impl ErrorType for GPIO {
    type Error = !;
}
