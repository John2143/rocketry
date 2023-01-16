use atmega4809_hal::gpio::{GPIO, ISC};

pub fn blink_led() -> ! {
    //https://docs.arduino.cc/static/90c04d4cfb88446cafa299787bf06056/ABX00028-pinout.png

    let led = GPIO::PORTE(2);
    led.pin_ctrl_isc(&ISC::InputDisable);
    led.output_enable();

    loop {
        for x in 0..255 {
            for _ in 0..5 {
                duty_cycle((x / 3) as u8);
            }
        }

        for x in (0..255).rev() {
            for _ in 0..5 {
                duty_cycle((x / 3) as u8);
            }
        }
    }
}

pub fn sleep(cycles: u16) {
    for _ in 0..cycles {
        unsafe { core::arch::asm!("nop") };
    }
}

pub fn duty_cycle(brightness: u8) {
    let brightness = brightness / 2;
    let led = GPIO::PORTD(3);
    if brightness > 0 {
        led.output_high();
    }
    sleep(brightness.into());
    if brightness < 49 {
        led.output_low();
    }
    sleep((50 - brightness).into());
}
