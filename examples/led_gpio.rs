#![no_std]
#![no_main]

/*
* Basic blinking external LED using GPIO pin example.
* WARNING: requires a LED to be wired to physical PIN9 with at least
* a 320 Ohm resistor in series similar to
* https://create.arduino.cc/projecthub/rowan07/make-a-simple-led-circuit-ce8308
*/

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::e310x::Peripherals;
use hifive1::hal::delay::Sleep;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Get GPIO
    let gpio = p.GPIO0.split();

    // GPIO PIN1 -> PIN9 physical on board (both hifive1 and hifive1-revB)
    let mut eled = gpio.pin1.into_output();

    // get the local interrupts struct
    let clint = p.CLINT.split();

    // get the sleep struct
    let mut sleep = Sleep::new(clint.mtimecmp, clocks);

    const PERIOD: u32 = 1000; // 1s
    loop {
        eled.toggle().unwrap();
        
        // sleep for 1s
        sleep.delay_ms(PERIOD);
    }
}
