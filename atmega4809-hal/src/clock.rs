pub enum ClockSelect {
    ///20MHz internal
    OSC20M,
    ///32.7kHz low power
    OSCULP32K,
    ///32.7kHz external
    XOSC32K,
    ///External
    EXTCLK,
}

impl ClockSelect {
    pub fn set_clock(&self) {
        let clk_ctrl = 0x0060 as *mut u8;
        let val = match self {
            ClockSelect::OSC20M => 0x0,
            ClockSelect::OSCULP32K => 0x1,
            ClockSelect::XOSC32K => 0x2,
            ClockSelect::EXTCLK => 0x3,
        };
        unsafe { clk_ctrl.write_volatile(val) };
    }
}

pub enum ClockPrescaler {
    None,
    D2,
    D4,
    D8,
    D16,
    D32,
    D64,
    D6,
    D10,
    D12,
    D24,
    D48,
}
impl ClockPrescaler {
    pub fn set_clock_prescaler(&self) {
        let pdiv = match self {
            ClockPrescaler::None => 0,
            ClockPrescaler::D2 => 0,
            ClockPrescaler::D4 => 1,
            ClockPrescaler::D8 => 2,
            ClockPrescaler::D16 => 3,
            ClockPrescaler::D32 => 4,
            ClockPrescaler::D64 => 5,
            ClockPrescaler::D6 => 8,
            ClockPrescaler::D10 => 9,
            ClockPrescaler::D12 => 0xa,
            ClockPrescaler::D24 => 0xb,
            ClockPrescaler::D48 => 0xc,
        };
        let pen = match self {
            ClockPrescaler::None => 0,
            _ => 1,
        };

        unsafe { (0x0061 as *mut u8).write_volatile(pdiv << 1 | pen) };
    }
}
