#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::{sprintln, BoardResources};

#[entry]
fn main() -> ! {
    let board = BoardResources::take().unwrap();
    let p = board.peripherals;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, board.pins.dig1, board.pins.dig0, 115_200.bps(), clocks);

    sprintln!("hello world!");

    loop {}
}
