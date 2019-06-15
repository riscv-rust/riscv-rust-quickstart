#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::e310x::Peripherals;
use hifive1::hal::spi::{Spi, MODE_0};

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let gpio = p.GPIO0.split();

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure SPI pins
    let mosi = gpio.pin3.into_iof0();
    let miso = gpio.pin4.into_iof0();
    let sck = gpio.pin5.into_iof0();
    let cs = gpio.pin2.into_iof0();

    // Configure SPI
    let pins = (mosi, miso, sck, cs);
    let mut spi = Spi::new(p.QSPI1, pins, MODE_0, 1_000_000.hz(), clocks);

    let mut buf = [0x41, 0x42, 0xab, 0xcd];
    let _ = spi.transfer(&mut buf);

    loop {}
}
