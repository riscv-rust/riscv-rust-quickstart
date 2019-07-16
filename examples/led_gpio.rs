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
use hifive1::hal::delay::Sleep;
use hifive1::hal::DeviceResources;
use hifive1::pin;

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // GPIO PIN1 -> DIG9 physical on board (both hifive1 and hifive1-revB)
    let mut eled = pin!(pins, dig9).into_output();

    // get the local interrupts struct
    let clint = dr.core_peripherals.clint;

    // get the sleep struct
    let mut sleep = Sleep::new(clint.mtimecmp, clocks);

    const PERIOD: u32 = 1000; // 1s
    loop {
        eled.toggle().unwrap();
        
        // sleep for 1s
        sleep.delay_ms(PERIOD);
    }
}
