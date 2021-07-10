#![no_std]
#![no_main]

extern crate panic_halt;

use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;

    // Configure clocks
    let _clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    loop {}
}
