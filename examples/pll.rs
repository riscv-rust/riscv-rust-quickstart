#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::e310x::Peripherals;
use hifive1::sprintln;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let gpio = p.GPIO0.split();
    let clint = p.CLINT.split();

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, gpio.pin17, gpio.pin16, 115_200.bps(), clocks);

    sprintln!("Measured clock frequency of {}MHz",
             clocks.measure_coreclk(&clint.mcycle).0 / 1_000_000);
    sprintln!("Computed clock frequency of {}MHz",
             clocks.coreclk().0 / 1_000_000);

    loop {}
}
