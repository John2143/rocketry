#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ClockSelect {
    ///20MHz internal
    OSC20M = 0x0,
    ///32.7kHz low power
    OSCULP32K = 0x1,
    ///32.7kHz external
    XOSC32K = 0x2,
    ///External
    EXTCLK = 0x3,
}

const CLK_CTRL: *mut u8 = 0x0060 as *mut u8;

impl ClockSelect {
    pub fn set_clock(&self) {
        let val = *self as u8;
        //TODO: force all timers to be awake, so we can always switch
        unsafe { CLK_CTRL.offset(0x18).write_volatile(0x2) };
        unsafe { CLK_CTRL.write_volatile(val) };
    }
    pub fn get_clock() -> Self {
        let clock = unsafe { CLK_CTRL.read_volatile() };
        assert!(clock <= 0x03);
        unsafe { core::mem::transmute(clock) }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ClockPrescaler {
    None = 0xff,
    D2 = 0,
    D4 = 1,
    D8 = 2,
    D16 = 3,
    D32 = 4,
    D64 = 5,
    D6 = 8,
    D10 = 9,
    D12 = 10,
    D24 = 11,
    D48 = 12,
}

impl ClockPrescaler {
    pub fn set_clock_prescaler(&self) {
        let pdiv = *self as u8;

        let pen = match self {
            ClockPrescaler::None => 0,
            _ => 1,
        };

        unsafe { CLK_CTRL.offset(1).write_volatile(pdiv << 1 | pen) };
    }
    pub fn get_clock_prescaler() -> Self {
        let v = unsafe { CLK_CTRL.offset(1).read_volatile() };
        if v & 1 == 0 {
            return ClockPrescaler::None;
        }

        unsafe { core::mem::transmute(v >> 1) }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Sleep {
    Idle = 0x0,
    Standby = 0x1,
    PowerOff = 0x2,
}

impl Sleep {
    pub fn set_sleep(self) {
        unsafe { (0x0050 as *mut u8).write_volatile(1 | ((self as u8) << 1)) };
    }
}
