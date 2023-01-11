#![feature(lang_items)]
#![no_std]
#![no_main]

use avr_device::atmega4809;

#[no_mangle]
fn main() -> ! {
    let p = atmega4809::Peripherals::take().unwrap();

    p.PORTE.outset.write(|w| w.pe2().set_bit());

    loop {
        p.PORTE.outtgl.write(|w| w.pe2().set_bit());
        for _ in 0..20 {
            avr_device::asm::nop();
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
            //toggle state
            porte.offset(0x07).write_volatile(1 << 2);
        }
    }
}

#[panic_handler]
pub fn pan(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
