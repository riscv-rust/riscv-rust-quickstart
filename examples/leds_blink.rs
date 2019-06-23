#![no_std]
#![no_main]

/*
* Basic blinking LEDs example using mtime/mtimecmp registers
* for "sleep" in a loop. Blinks each led once and goes to the next one.
*/

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::e310x::Peripherals;
use hifive1::hal::delay::Sleep;
use hifive1::Led;
use hifive1::sprintln;

// switches led according to supplied status returning the new state back
fn toggle_led(led: &mut Led, status: bool) -> bool {
    match status {
        true => led.on(),
        false => led.off(),
    }

    !status
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let gpio = p.GPIO0.split();

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, gpio.pin17, gpio.pin16, 115_200.bps(), clocks);

    // get all 3 led pins in a tuple (each pin is it's own type here)
    let mut tleds = hifive1::rgb(gpio.pin22, gpio.pin19, gpio.pin21);

    // get leds as the Led trait in an array so we can index them
    let ileds: [&mut Led; 3] = [&mut tleds.0, &mut tleds.1, &mut tleds.2];

    // get the local interrupts struct
    let clint = p.CLINT.split();

    let mut led_status = [true, true, true]; // start on red
    let mut current_led = 0; // start on red

    // get the sleep struct
    let mut sleep = Sleep::new(clint.mtimecmp, clocks);

    sprintln!("Starting blink loop");

    const PERIOD: u32 = 1000; // 1s
    loop {
        // toggle led
        led_status[current_led] = toggle_led(ileds[current_led], led_status[current_led]);

        // increment index if we blinked back to blank
        if led_status[current_led] {
            current_led = (current_led + 1) % 3
        }

        // sleep for 1
        sleep.delay_ms(PERIOD);
    }
}
