#![no_std]
#![no_main]

use rp2040_hal::{clocks::init_clocks_and_plls, Clock, Sio, Watchdog};
use rp_pico::{pac, Pins, XOSC_CRYSTAL_FREQ};

fn test_gpio_toggle() {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let toggle_pin = pins.gpio2.into_push_pull_output();
    benchmark_gpio_toggle(toggle_pin, &mut delay);
}