#![no_std]
#![no_main]

//extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::e310x::Peripherals;
use hifive1::hal::spi::{Spi, MODE_0, SpiX};
use hifive1::hal::gpio::{gpio0::{Pin9, Pin10}, Output, Regular, Invert, Input, Floating};
use hifive1::hal::delay::Delay;
use hifive1::hal::clock::Clocks;
use hifive1::sprintln;
use core::panic::PanicInfo;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::blocking::spi::WriteIter;

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sprintln!("panic: {}", info);
    loop {
        use core::sync::atomic;
        use core::sync::atomic::Ordering;
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[derive(Debug)]
enum EspError {
    ProtocolError,
    BufferOverflow,
    WouldBlock
}

struct EspWiFi<SPI, PINS> {
    spi: Spi<SPI, PINS>,
    cs: Pin9<Output<Regular<Invert>>>,
    handshake: Pin10<Input<Floating>>,
    delay: FastDelay,
}

impl<SPI: SpiX, PINS> EspWiFi<SPI, PINS> {
    fn set_cs(&mut self, active: bool) {
        self.delay.delay_us(18u32);
        match active {
            true => self.cs.set_high().unwrap(),
            false => self.cs.set_low().unwrap(),
        }
        self.delay.delay_us(18u32);
    }

    fn send_bytes(&mut self, bytes: &[u8]) {
        self.set_cs(true);
        self.spi.write(bytes).unwrap();
        self.set_cs(false);
    }

    fn transfer(&mut self, buffer: &mut [u8]) {
        self.set_cs(true);
        self.spi.transfer(buffer).unwrap();
        self.set_cs(false);
    }

    fn discard(&mut self, size: usize) {
        self.set_cs(true);
        self.spi.write_iter((0..size).map(|_| 0x00)).unwrap();
        self.set_cs(false);
    }

    pub fn send(&mut self, s: &str) {
        let bytes = s.as_bytes();
        assert!(bytes.len() <= 127);

        self.send_bytes(&[0x02, 0x00, 0x00, 0x00]);
        self.send_bytes(&[bytes.len() as u8, 0x00, 0x00, 0x41]);
        self.send_bytes(bytes);
    }

    pub fn recv<'a>(&mut self, buffer: &'a mut [u8]) -> Result<&'a str, EspError> {
        if self.handshake.is_low().unwrap() {
            return Err(EspError::WouldBlock);
        }

        self.send_bytes(&[0x01, 0x00, 0x00, 0x00]);

        let mut request = [0u8; 4];
        self.transfer(&mut request);
        if request[3] != 0x42 {
            return Err(EspError::ProtocolError);
        }

        let n = (request[0] & 0x7F) as usize + ((request[1] as usize) << 7);
        if n > buffer.len() {
            self.discard(n);
            return Err(EspError::BufferOverflow);
        }

        self.transfer(&mut buffer[..n]);
        Ok(core::str::from_utf8(&buffer[..n]).unwrap())
    }
}

struct FastDelay {
    us_cycles: u64,
}

impl FastDelay {
    pub fn new(clocks: Clocks) -> Self {
        Self {
            us_cycles: clocks.coreclk().0 as u64 * 3 / 2_000_000,
        }
    }
}

impl DelayUs<u32> for FastDelay {
    fn delay_us(&mut self, us: u32) {
        use riscv::register::mcycle;

        let t = mcycle::read64() + self.us_cycles * (us as u64);
        while mcycle::read64() < t {}
    }
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let gpio = p.GPIO0.split();

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 50.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, gpio.pin17, gpio.pin16, 115_200.bps(), clocks);

    // Configure SPI pins
    let mosi = gpio.pin3.into_iof0();
    let miso = gpio.pin4.into_iof0();
    let sck = gpio.pin5.into_iof0();
    //let cs = gpio.pin9.into_iof0();
    let mut cs = gpio.pin9.into_inverted_output();
    cs.set_low().unwrap();

    // Configure SPI
    let pins = (mosi, miso, sck);
    let spi = Spi::new(p.QSPI1, pins, MODE_0, 100_000.hz(), clocks);

    let handshake = gpio.pin10.into_floating_input();
    let mut wifi = EspWiFi {
        spi,
        cs,
        handshake,
        delay: FastDelay::new(clocks),
    };

    sprintln!("WiFi Test");

    Delay.delay_ms(10u32);

    let mut buffer = [0u8; 256];

    wifi.send("AT+CWMODE=0\r\n");
    sprintln!("resp: {:?}", wifi.recv(&mut buffer));

    loop {}
}
