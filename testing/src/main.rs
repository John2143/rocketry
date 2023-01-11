#![feature(asm_experimental_arch)]
#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    //let mut led = pins.d13.into_output();

    //loop {
        //led.toggle();
        //arduino_hal::delay_ms(1000);
    //}
    //
    direct_led()
}

pub fn duty_cycle(brightness: u8) {
    let porte = 0x0480 as *mut u8;
    unsafe {
        if brightness > 0 {
            porte.offset(0x05).write_volatile(1 << 2);
        }
        for _ in 0..(brightness) {
            core::arch::asm!("nop");
        }
        if brightness < 100 {
            porte.offset(0x06).write_volatile(1 << 2);
        }
        for _ in 0..(100 - brightness) {
            core::arch::asm!("nop");
        }
    }
}

pub fn direct_led() -> ! {
    unsafe {
        //https://docs.arduino.cc/static/90c04d4cfb88446cafa299787bf06056/ABX00028-pinout.png
        // Build in LED = D13 = PORTE 2
        let porte = 0x0480 as *mut u8;

        //disable input
        porte.offset(0x12).write_volatile(0x04);

        //set to output
        porte.offset(0x01).write_volatile(1 << 2);

        //write 1
        //core::ptr::write_volatile(porte.offset(0x05), 1 << 2);
        //write 0
        //core::ptr::write_volatile(porte.offset(0x06), 1 << 2);
        loop {

            for x in 0..1000 {
                duty_cycle((x / 10) as u8);
            }

            for x in (0..1000).rev() {
                duty_cycle((x / 10) as u8);
            }
        }
    }
}
