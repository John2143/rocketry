#![feature(asm_experimental_arch)]
#![allow(dead_code)]
#![no_std]
#![no_main]

pub mod gpio;
pub mod testing;

use core::{arch::asm, borrow::Borrow};

use avr_oxide::{
    boards::board, devices::OxideSerialPort, hal::generic::serial::SerialPortMode, io::Write,
};
use gpio::GPIO;

#[no_mangle]
static __OXIDE_MAIN_THREAD_STACK_SIZE: usize = 1024usize;

#[link(name = "oxide-boot-atmega4809", kind = "static")]
extern "C" {}

#[no_mangle]
pub fn __oxide_main() -> ! {
    let led = GPIO::PORTE(2);
    led.output_enable();
    led.pin_ctrl_isc(&gpio::ISC::IntDisable);
    led.output_high();

    //let supervisor = oxide::instance();

    //let i2c = i2
    let mut serial = OxideSerialPort::using_port_and_pins(
        board::usb_serial(),
        board::usb_serial_pins().0,
        board::usb_serial_pins().1,
    )
    .mode(SerialPortMode::Synch(
        avr_oxide::hal::generic::serial::SynchronousMode::Master(
            avr_oxide::hal::generic::serial::BaudRate::Baud9600,
        ),
    ));
    let _ = serial.write(b"Welcome to AVRoxide\n");

    loop {
        let buf = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let _ = serial.write(&buf);
        let s = avr_oxide::sleepctrl!();
        for _ in 0..100_000 {
            unsafe { asm!("nop") };
        }
    }

    testing::blink_led();
}
