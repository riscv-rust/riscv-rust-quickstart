#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::e310x::Peripherals;
use hifive1::hal::i2c::{Speed, I2c};
use hifive1::hal::serial::Serial;
use hifive1::hal::stdout::Stdout;


#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut gpio = p.GPIO0.split();

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 100.mhz().into());

    // Configure UART
    let (tx, rx) = hifive1::tx_rx(
        gpio.pin17,
        gpio.pin16,
        &mut gpio.out_xor,
        &mut gpio.iof_sel,
        &mut gpio.iof_en
    );
    let serial = Serial::uart0(p.UART0, (tx, rx), 115_200.bps(), clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);

    // Configure I2C
    let sda = gpio.pin12.into_iof0(&mut gpio.out_xor, &mut gpio.iof_sel, &mut gpio.iof_en);
    let scl = gpio.pin13.into_iof0(&mut gpio.out_xor, &mut gpio.iof_sel, &mut gpio.iof_en);
    let mut i2c = I2c::new(p.I2C0, sda, scl, Speed::Normal, clocks);

    // Read calibration data from BME280 sensor (registers 0xE1..0xF0)
    let send_buffer = [0xe1];
    let mut recv_buffer = [0u8; 0x10];
    match i2c.write_read(0x76, &send_buffer, &mut recv_buffer) {
        Ok(_) => writeln!(stdout, "Data received = {:?}", recv_buffer).unwrap(),
        Err(e) => writeln!(stdout, "Error: {:?}", e).unwrap(),
    }

    loop { }
}
