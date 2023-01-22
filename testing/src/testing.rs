use atmega4809_hal::gpio::{GPIO, ISC};

pub fn sleep(cycles: u16) {
    for _ in 0..cycles {
        unsafe { core::arch::asm!("nop") };
    }
}
