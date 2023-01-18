use crate::set16;
pub struct PWM;
//https://2143.me/f/FfId.png
/*
0x00 CTRLA 7:0 RUNSTDBY SYNCUPD CLKSEL[1:0] ENABLE
0x01 CTRLB 7:0 ASYNC CCMPINIT CCMPEN CNTMODE[2:0]
0x02 Reserved
0x03 Reserved
0x04 EVCTRL 7:0 FILTER EDGE CAPTEI
0x05 INTCTRL 7:0 CAPT
0x06 INTFLAGS 7:0 CAPT
0x07 STATUS 7:0 RUN
0x08 DBGCTRL 7:0 DBGRUN
0x09 TEMP 7:0 TEMP[7:0]
0x0A CNT 7:0 CNT[7:0]
15:8 CNT[15:8]
0x0C CCMP 7:0 CCMP[7:0]
15:8 CCMP[15:8]
*/

pub const PORTMUX: *mut u8 = 0x05E0 as *mut _;
pub const TCA0: *mut u8 = 0x0A00 as *mut _;
pub const TCB0: *mut u8 = 0x0A80 as *mut _;
pub const TCB1: *mut u8 = 0x0A90 as *mut _;
pub const TCB2: *mut u8 = 0x0AA0 as *mut _;
pub const TCB3: *mut u8 = 0x0AB0 as *mut _;

#[repr(u8)]
pub enum PWMPort {
    PORTA,
    PORTB,
    PORTC,
    PORTD,
    PORTE,
    PORTF,
}

#[repr(u8)]
pub enum WaveformGenerationMode {
    ///Normal PER TOP(1) TOP(1)
    NORMAL = 0x0,
    ///Frequency CMP0 TOP(1) TOP(1)
    FRQ = 0x1,
    ///Single-slope PWM PER BOTTOM BOTTOM
    SINGLESLOPE = 0x3,
    ///Dual-slope PWM PER BOTTOM TOP
    DSTOP = 0x5,
    ///Dual-slope PWM PER BOTTOM TOP and BOTTOM
    DSBOTH = 0x6,
    ///Dual-slope PWM PER BOTTOM BOTTOM
    DSBOTTOM = 0x7,
}

impl PWM {
    pub fn change_port_tca(p: PWMPort) {
        unsafe { PORTMUX.offset(0x04).write_volatile(p as u8) };
    }

    pub fn enable(w: WaveformGenerationMode) {
        let pins_enabled = 0b0111_0000;
        unsafe { TCA0.offset(0x01).write_volatile(pins_enabled | (w as u8)) };

        let clock_divisor = 0x0;
        let enable = 0x1;
        unsafe { TCA0.write_volatile(clock_divisor << 1 | enable) };
    }

    pub fn set_split_mode(split: bool) {
        unsafe { TCA0.offset(0x03).write_volatile(if split { 1 } else { 0 }) };
    }

    pub fn disable() {
        unsafe { TCA0.write_volatile(0x0) };
    }

    pub fn set_cnt(v: u16) {
        set16(unsafe { TCA0.offset(0x20) }, v);
    }

    pub fn set_per(v: u16) {
        set16(unsafe { TCA0.offset(0x26) }, v);
    }

    pub fn set_cmp0(v: u16) {
        set16(unsafe { TCA0.offset(0x28) }, v);
    }

    pub fn set_cmp1(v: u16) {
        set16(unsafe { TCA0.offset(0x2a) }, v);
    }

    pub fn set_cmp2(v: u16) {
        set16(unsafe { TCA0.offset(0x2c) }, v);
    }
}
