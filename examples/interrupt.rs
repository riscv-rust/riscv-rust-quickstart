#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::hal::core::CorePeripherals;
use hifive1::hal::core::plic::Priority;
use hifive1::hal::e310x::Interrupt;
use hifive1::{sprint, sprintln, pin};
use core::sync::atomic::{AtomicUsize, Ordering};
use riscv::register::{mstatus, mie};
use bare_metal::Nr;

static COUNTER: AtomicUsize = AtomicUsize::new(0);
static COUNTER2: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub unsafe extern "C" fn MachineTimer() {
    COUNTER.fetch_add(1, Ordering::SeqCst);

    let mut clint = CorePeripherals::steal().clint;
    clint.mtimecmp.set_mtimecmp(clint.mtime.mtime() + 65536 / 2);
}

#[no_mangle]
pub unsafe extern "C" fn MachineExternal() {
    let mut plic = CorePeripherals::steal().plic;
    let intr = plic.claim.claim().unwrap();
    match intr {
        Interrupt::RTC => {
            COUNTER2.fetch_add(1, Ordering::SeqCst);

            let rtc = &*hifive1::hal::e310x::RTC::ptr();
            rtc.rtccmp.modify(|r, w| w.bits(r.bits() + 65536));
            core::sync::atomic::compiler_fence(Ordering::SeqCst);
        }
        _ => {
            sprintln!("Unknown interrupt #{}!", intr.nr());
            panic!("Unknown interrupt");
        }
    }
    plic.claim.complete(intr);
}

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 64.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, pin!(pins, uart0_tx), pin!(pins, uart0_rx), 115_200.bps(), clocks);

    sprintln!("\nhello world!");

    // Disable watchdog
    let wdg = p.WDOG;
    wdg.wdogcfg.modify(|_, w| w.enalways().clear_bit());

    let mut rtc = p.RTC.constrain();
    rtc.enable();
    rtc.set_scale(0);
    rtc.set_rtc(0);
    rtc.set_rtccmp(10000);
    rtc.enable();

    let mut clint = dr.core_peripherals.clint;
    clint.mtimecmp.set_mtimecmp(clint.mtime.mtime() + 10000);

    unsafe {
        mie::set_mtimer();
        mstatus::set_mie();

        let rplic = &*hifive1::hal::e310x::PLIC::ptr();
        for p in rplic.priority.iter() {
            p.write(|w| w.bits(0));
        }

        let mut plic = CorePeripherals::steal().plic;
        plic.rtc.set_priority(Priority::P7);
        plic.rtc.enable();
        plic.threshold.set(Priority::P0);
        plic.mext.enable();
    }

    loop {
        let cnt = COUNTER.load(Ordering::SeqCst);
        let cnt2 = COUNTER2.load(Ordering::SeqCst);
        sprint!("\rCounter: {}, {}           ", cnt, cnt2);
    }
}
