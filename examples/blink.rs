#![no_main]
#![no_std]

extern crate cortex_m;
extern crate panic_halt;

use xiao_m0::clock::GenericClockController;
use xiao_m0::delay::Delay;
use xiao_m0::entry;
use xiao_m0::pac::{CorePeripherals, Peripherals};
use xiao_m0::prelude::*;

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
    let mut pins = xiao_m0::Pins::new(peripherals.PORT);
    // let mut led0 = pins.led0.into_open_drain_output(&mut pins.port);
    let mut led1 = pins.led1.into_open_drain_output(&mut pins.port);
    let mut led2 = pins.led2.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    led1.toggle();
    loop {
        delay.delay_ms(200u8);
        led1.toggle();
        led2.toggle();
    }
}
