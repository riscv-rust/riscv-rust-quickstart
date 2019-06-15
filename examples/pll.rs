#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::serial::Serial;
use hifive1::hal::e310x::Peripherals;
use hifive1::hal::stdout::Stdout;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let gpio = p.GPIO0.split();
    let clint = p.CLINT.split();

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART
    let tx = gpio.pin17.into_iof0();
    let rx = gpio.pin16.into_iof0();
    let serial = Serial::new(p.UART0, (tx, rx), 115_200.bps(), clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);

    writeln!(stdout, "Measured clock frequency of {}MHz",
             clocks.measure_coreclk(&clint.mcycle).0 / 1_000_000).unwrap();
    writeln!(stdout, "Computed clock frequency of {}MHz",
             clocks.coreclk().0 / 1_000_000).unwrap();

    loop {}
}
