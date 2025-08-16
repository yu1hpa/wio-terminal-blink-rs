#![no_std]
#![no_main]

//! Wio TerminalのLチカサンプルプログラム

use panic_halt as _;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins, Sets};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let sets: Sets = Pins::new(peripherals.port).split();
    let mut user_led = sets.user_led.into_push_pull_output();

    loop {
        user_led.toggle().unwrap();
        delay.delay_ms(500u16);
    }
}
