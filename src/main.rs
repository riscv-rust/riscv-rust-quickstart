//! RISCV Rust Quickstart Template

#![no_std]

extern crate hifive;

use core::fmt::Write;
use hifive::*;
use hifive::prelude::*;
use hifive::gpio::{Pin18, Pin19};

enum State {
    Running,
    Stopped,
}

static mut STATE: State = State::Stopped;

fn init() {
    let p = Peripherals::take().unwrap();

    // Initialize uart
    let serial = Serial(&p.UART0);
    serial.init(115_200.hz().invert(), &p.GPIO0);

    // Initialize leds
    led::init(&p.GPIO0);

    // Initialize stop btn
    Pin18::init(&p.GPIO0, PinConfig::InputPullup);
    Pin18::enable_interrupt(&p.GPIO0, PinInterrupt::Rise);

    // Initialize start btn
    Pin19::init(&p.GPIO0, PinConfig::InputPullup);
    Pin19::enable_interrupt(&p.GPIO0, PinInterrupt::Fall);

    // Initialize clint
    let timer = Clint(&p.CLINT);
    timer.set_timeout(1.s());

    // Initialize plic
    let plic = Plic(&p.PLIC);
    plic.enable(Interrupt::GPIO18);
    plic.enable(Interrupt::GPIO19);

    // Enable interrupts
    unsafe { interrupt::enable(); }
}

fn main() {
    init();
    loop {}
}

#[no_mangle]
pub fn mtimer_trap_handler(p: &Peripherals) {
    Clint(&p.CLINT).restart();
    Blue::toggle(&p.GPIO0);
}
#[no_mangle]
pub fn plic_trap_handler(p: &Peripherals, intr: &Interrupt) {
    let serial = Serial(&p.UART0);
    let mut stdout = Port(&serial);

    match *intr {
        Interrupt::GPIO18 => {
            unsafe { STATE = State::Stopped; }
            Red::on(&p.GPIO0);
            writeln!(stdout, "Stopped").unwrap();
        },
        Interrupt::GPIO19 => {
            unsafe { STATE = State::Running; }
            Red::off(&p.GPIO0);
            writeln!(stdout, "Started").unwrap();
        }
        _ => {},
    }
}
