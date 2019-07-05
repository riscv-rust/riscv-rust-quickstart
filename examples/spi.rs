#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::spi::{Spi, MODE_0};
use hifive1::BoardResources;

#[entry]
fn main() -> ! {
    let board = BoardResources::take().unwrap();
    let p = board.peripherals;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure SPI pins
    let mosi = board.pins.dig11.into_iof0();
    let miso = board.pins.dig12.into_iof0();
    let sck = board.pins.dig13.into_iof0();
    let cs = board.pins.dig15.into_iof0();

    // Configure SPI
    let pins = (mosi, miso, sck, cs);
    let mut spi = Spi::new(p.QSPI1, pins, MODE_0, 1_000_000.hz(), clocks);

    let mut buf = [0x41, 0x42, 0xab, 0xcd];
    let _ = spi.transfer(&mut buf);

    loop {}
}
