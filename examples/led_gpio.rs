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
    const PERIOD: u64 = 32000; // ~1s
    let p = Peripherals::take().unwrap();
    // Configure clocks
    hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Get GPIO
    let mut gpio = p.GPIO0.split();

    // GPIO PIN1 -> PIN9 physical on board (both hifive1 and hifive1-revB)
    let mut eled = gpio.pin1.into_output(&mut gpio.output_en, &mut gpio.drive,
                                         &mut gpio.out_xor, &mut gpio.iof_en);

    // get the local interrupts struct
    let mut clint = p.CLINT.split();

    // enable timer
    unsafe {
        mie::set_mtimer();
    }

    loop {
        eled.toggle().unwrap();

        // set next wakeup time each iteration
        clint.mtimecmp.set_mtimecmp(clint.mtime.mtime() + PERIOD);

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
