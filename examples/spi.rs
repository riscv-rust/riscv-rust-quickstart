#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::spi::{Spi, MODE_0};
use hifive1::hal::DeviceResources;
use hifive1::pin;

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure SPI pins
    let mosi = pin!(pins, spi0_mosi).into_iof0();
    let miso = pin!(pins, spi0_miso).into_iof0();
    let sck = pin!(pins, spi0_sck).into_iof0();
    let cs = pin!(pins, spi0_ss0).into_iof0();

    // Configure SPI
    let pins = (mosi, miso, sck, cs);
    let mut spi = Spi::new(p.QSPI1, pins, MODE_0, 1_000_000.hz(), clocks);

    let mut buf = [0x41, 0x42, 0xab, 0xcd];
    let _ = spi.transfer(&mut buf);

    loop {}
}
