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
use riscv::register::{mie, mip};

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
    let mut clint = p.CLINT.split();

    // enable timer
    unsafe {
        mie::set_mtimer();
    }

    let period = clocks.lfclk().0 as u64; // 1s
    loop {
        eled.toggle().unwrap();

        // set next wakeup time each iteration
        clint.mtimecmp.set_mtimecmp(clint.mtime.mtime() + period);

        unsafe {
            // Wait For Interrupt will put CPU to sleep until an interrupt hits
            // in our case when internal timer mtime value >= mtimecmp value
            // after which empty handler gets called and we go into the
            // next iteration of this loop
            loop {
                riscv::asm::wfi();

                // check if we got the right interrupt cause, otherwise just loop back to wfi
                if mip::read().mtimer() {
                    break;
                }
            }
        }
    }
}
