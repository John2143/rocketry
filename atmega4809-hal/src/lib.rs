#![feature(asm_experimental_arch)]
#![feature(never_type)]
#![no_std]
/*
0x0000 VPORTA Virtual Port A X X X X
0x0004 VPORTB Virtual Port B X
0x0008 VPORTC Virtual Port C X X X X
0x000C VPORTD Virtual Port D X X X X
0x0010 VPORTE Virtual Port E X X
0x0014 VPORTF Virtual Port F X X X X
0x001C GPIO General Purpose I/O registers X X X X
0x0030 CPU CPU X X X X
0x0040 RSTCTRL Reset Controller X X X X
0x0050 SLPCTRL Sleep Controller X X X X
0x0060 CLKCTRL Clock Controller X X X X
0x0080 BOD Brown-out Detector X X X X
0x00A0 VREF Voltage Reference X X X X
0x0100 WDT Watchdog Timer X X X X
0x0110 CPUINT Interrupt Controller X X X X
0x0120 CRCSCAN Cyclic Redundancy Check Memory Scan X X X X
0x0140 RTC Real-Time Counter X X X X
0x0180 EVSYS Event System X X X X
0x01C0 CCL Configurable Custom Logic X X X X
0x0400 PORTA Port A Configuration X X X X
0x0420 PORTB Port B Configuration X
0x0440 PORTC Port C Configuration X X X X
0x0460 PORTD Port D Configuration X X X X
0x0480 PORTE Port E Configuration X X
0x04A0 PORTF Port F Configuration X X X X
0x05E0 PORTMUX Port Multiplexer X X X X
0x0600 ADC0 Analog-to-Digital Converter X X X X
0x0680 AC0 Analog Comparator 0 X X X X
0x0800 USART0 Universal Synchronous Asynchronous Receiver Transmitter 0 X X X X
0x0820 USART1 Universal Synchronous Asynchronous Receiver Transmitter 1 X X X X
0x0840 USART2 Universal Synchronous Asynchronous Receiver Transmitter 2 X X X X
0x0860 USART3 Universal Synchronous Asynchronous Receiver Transmitter 3 X X
0x08A0 TWI0 Two-Wire Interface X X X X
0x08C0 SPI0 Serial Peripheral Interface X X X X
0x0A00 TCA0 Timer/Counter Type A instance 0 X X X X
0x0A80 TCB0 Timer/Counter Type B instance 0 X X X X
0x0A90 TCB1 Timer/Counter Type B instance 1 X X X X
0x0AA0 TCB2 Timer/Counter Type B instance 2 X X X X
0x0AB0 TCB3 Timer/Counter Type B instance 3 X X
0x0F00 SYSCFG System Configuration X X X X
0x1000 NVMCTRL Nonvolatile Memory Controller X X X X
0x1100 SIGROW Signature Row X X X X
0x1280 FUSE Device-specific fuses X X X X
0x1300 USERROW User Row
*/

pub mod clock;
pub mod gpio;
pub mod i2c;
pub mod pwm;
pub mod spi;
pub mod usart;

pub struct Delay;

pub use embedded_hal::blocking::delay::DelayMs;

pub fn set16(p: *mut u8, v: u16) {
    // little endian?
    unsafe {
        p.offset(0).write_volatile((v & 0xFF) as u8);
        p.offset(1).write_volatile((v >> 8) as u8);
    }
}

impl embedded_hal::blocking::delay::DelayMs<u16> for Delay {
    ///TODO: make this accurate
    fn delay_ms(&mut self, ms: u16) {
        let c = clock::ClockSelect::get_clock();
        let loop_max = match c {
            clock::ClockSelect::OSC20M => 202,
            clock::ClockSelect::OSCULP32K => 32,
            clock::ClockSelect::XOSC32K => 32,
            clock::ClockSelect::EXTCLK => todo!(),
        };

        for _ in 0..ms {
            for _ in 0..loop_max {
                unsafe { core::arch::asm!("nop") };
            }
        }
    }
}
