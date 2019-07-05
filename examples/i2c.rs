#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::i2c::{Speed, I2c};
use hifive1::{sprintln, BoardResources};


#[entry]
fn main() -> ! {
    let board = BoardResources::take().unwrap();
    let p = board.peripherals;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 100.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, board.pins.dig1, board.pins.dig0, 115_200.bps(), clocks);

    // Configure I2C
    let sda = board.pins.dig18.into_iof0();
    let scl = board.pins.dig19.into_iof0();
    let mut i2c = I2c::new(p.I2C0, sda, scl, Speed::Normal, clocks);

    // Read calibration data from BME280 sensor (registers 0xE1..0xF0)
    let send_buffer = [0xe1];
    let mut recv_buffer = [0u8; 0x10];
    match i2c.write_read(0x76, &send_buffer, &mut recv_buffer) {
        Ok(_) => sprintln!("Data received = {:?}", recv_buffer),
        Err(e) => sprintln!("Error: {:?}", e),
    }

    loop { }
}
