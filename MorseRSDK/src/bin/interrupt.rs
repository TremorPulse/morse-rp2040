#![no_std]
#![no_main]

use rp2040_hal::{
    pac,
    clocks::{Clock, init_clocks_and_plls},
    watchdog::Watchdog,
    Sio,
    gpio::Pins,
};
use rp_pico::XOSC_CRYSTAL_FREQ;

fn test_interrupt_latency() {
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

    let button_pin = pins.gpio16.into_pull_up_input();
    let led_pin = pins.gpio25.into_push_pull_output();
    benchmark_interrupt(button_pin, led_pin, &mut delay);
}