#![no_std]
#![no_main]

use rp2040_hal::{
    clocks::init_clocks_and_plls,
    gpio::FunctionPwm,
    pac::{self, CorePeripherals, Peripherals},
    Sio, Watchdog, 
    pwm::Pwm
};
use rp_pico::{Pins, XOSC_CRYSTAL_FREQ};
use morse_rsdk::benchmarks::benchmark_pwm;

fn test_pwm_setup() {
    let mut pac = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
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
    let pwm_pin = pins.gpio15.into_function::<FunctionPwm>();
    let pwm = Pwm::new(pac.PWM, pwm_pin.channel(), &mut pac.RESETS);
    benchmark_pwm(pwm, &mut delay);
}