#![no_std]
#![no_main]

mod lib;

extern crate cortex_m;

use panic_halt as _;

use lib as hal;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.a1.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        delay.delay_ms(30u32);
        red_led.set_high().unwrap();
        delay.delay_ms(30u32);
        red_led.set_low().unwrap();
    }
}
