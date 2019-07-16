#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::{sprintln, pin};

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, pin!(pins, uart0_tx), pin!(pins, uart0_rx), 115_200.bps(), clocks);

    sprintln!("Measured clock frequency of {}MHz",
             clocks.measure_coreclk().0 / 1_000_000);
    sprintln!("Computed clock frequency of {}MHz",
             clocks.coreclk().0 / 1_000_000);

    loop {}
}
