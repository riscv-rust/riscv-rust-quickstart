#![no_std]
#![no_main]

/*
* Basic blinking LEDs example using mtime/mtimecmp registers
* for "sleep" in a loop. Blinks each led once and goes to the next one.
*/

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::delay::Sleep;
use hifive1::hal::DeviceResources;
use hifive1::{Led, pin, pins};
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
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, pin!(pins, uart0_tx), pin!(pins, uart0_rx), 115_200.bps(), clocks);

    // get all 3 led pins in a tuple (each pin is it's own type here)
    let rgb_pins = pins!(pins, (led_red, led_green, led_blue));
    let mut tleds = hifive1::rgb(rgb_pins.0, rgb_pins.1, rgb_pins.2);

    // get leds as the Led trait in an array so we can index them
    let ileds: [&mut Led; 3] = [&mut tleds.0, &mut tleds.1, &mut tleds.2];

    // get the local interrupts struct
    let clint = dr.core_peripherals.clint;

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
