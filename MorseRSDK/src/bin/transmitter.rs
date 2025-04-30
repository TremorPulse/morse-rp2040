#![no_std]
#![no_main]
#![feature(used_with_arg)]

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

use panic_halt as _;
use cortex_m_semihosting::hprintln;

use rp2040_hal::{
    gpio::{bank0::{Gpio16, Gpio21, Gpio25}, Pin, FunctionSio, SioOutput, SioInput, PullUp, PullDown},
    pac,
    timer::Timer,
    clocks::{Clock, init_clocks_and_plls},
    watchdog::Watchdog,
    Sio,
    gpio::Pins,
};
use rp2040_hal::entry;
use rp_pico::XOSC_CRYSTAL_FREQ;
use cortex_m::delay::Delay;
use embedded_hal::digital::{InputPin, OutputPin};
use morse_rsdk::{DEBOUNCE_TIME_MS, DOT_FREQ, DASH_FREQ, SYNC_PATTERN, DOT_THRESHOLD_MS, DASH_THRESHOLD_MS};

pub struct Transmitter {
    button_pin: Pin<Gpio16, FunctionSio<SioInput>, PullUp>,
    speaker_pin: Pin<Gpio21, FunctionSio<SioOutput>, PullDown>,
    led_pin: Pin<Gpio25, FunctionSio<SioOutput>, PullDown>,
    timer: Timer,
    delay: Delay,
}

impl Transmitter {
    pub fn new(
        button_pin: Pin<Gpio16, FunctionSio<SioInput>, PullUp>,
        speaker_pin: Pin<Gpio21, FunctionSio<SioOutput>, PullDown>,
        led_pin: Pin<Gpio25, FunctionSio<SioOutput>, PullDown>,
        timer: Timer,
        delay: Delay,
    ) -> Self {
        Self {
            button_pin,
            speaker_pin,
            led_pin,
            timer,
            delay,
        }
    }

    pub fn init(&mut self) {
        hprintln!("Transmitter initialized");
    }

    pub fn generate_tone(&mut self, freq_hz: u32, duration_ms: u32) {
        let period_us = 1_000_000 / freq_hz;
        let cycles = (duration_ms * 1000) / period_us;

        for _ in 0..cycles {
            self.speaker_pin.set_high().unwrap();
            self.delay.delay_us(period_us / 2);
            self.speaker_pin.set_low().unwrap();
            self.delay.delay_us(period_us / 2);
        }
    }

    pub fn transmit_dot(&mut self) {
        self.led_pin.set_high().unwrap();
        self.generate_tone(DOT_FREQ, 250);
        self.led_pin.set_low().unwrap();
        hprintln!(".");
        self.transmit_gap(250);
    }

    pub fn transmit_dash(&mut self) {
        self.led_pin.set_high().unwrap();
        self.generate_tone(DASH_FREQ, 750);
        self.led_pin.set_low().unwrap();
        hprintln!("-");
        self.transmit_gap(250);
    }

    pub fn transmit_gap(&mut self, duration_ms: u32) {
        self.delay.delay_ms(duration_ms);
    }

    pub fn transmit_sync(&mut self) {
        hprintln!("Transmitting sync pattern...");
        for c in SYNC_PATTERN.chars() {
            match c {
                '.' => self.transmit_dot(),
                '-' => self.transmit_dash(),
                _ => continue,
            }
        }
        hprintln!("Sync pattern transmitted");
    }

    pub fn transmit_morse_input(&mut self) {
        let mut press_start = 0u64;
        let mut last_release_time = 0u64;
        let mut last_log_time = 0u64;
        let mut button_was_pressed = false;
        let mut in_word = false;

        hprintln!("Starting Morse transmission...");
        hprintln!("Ready for input");

        self.transmit_sync();

        loop {
            let current_ticks = self.timer.get_counter().ticks();
            let button_state = self.button_pin.is_low().unwrap();

            if button_state && !button_was_pressed {
                let current_ms = current_ticks / 1000;
                let last_release_ms = last_release_time / 1000;
                
                if current_ms - last_release_ms > DEBOUNCE_TIME_MS.into() {
                    press_start = current_ticks;
                    button_was_pressed = true;
                }
            }
            else if !button_state && button_was_pressed {
                let current_ms = current_ticks / 1000;
                let press_start_ms = press_start / 1000;
                
                if current_ms - press_start_ms > DEBOUNCE_TIME_MS.into() {
                    let press_duration_ms = current_ms - press_start_ms;

                    if press_duration_ms <= DOT_THRESHOLD_MS.into() {
                        self.transmit_dot();
                    } else if press_duration_ms <= DASH_THRESHOLD_MS.into() {
                        self.transmit_dash();
                    }
                    last_release_time = current_ticks;
                    in_word = true;
                }
                button_was_pressed = false;
            }
            else if !button_state && in_word {
                let current_ms = current_ticks / 1000;
                let last_release_ms = last_release_time / 1000;
                let gap_duration_ms = current_ms - last_release_ms;

                if gap_duration_ms > 1750 {
                    hprintln!("WORD GAP");
                    in_word = false;
                } else if gap_duration_ms > 750 && current_ms - (last_log_time / 1000) > 750 {
                    hprintln!("CHAR GAP");
                    last_log_time = current_ticks;
                }
            }

            self.delay.delay_ms(10);
        }
    }
}

#[entry]
fn main() -> ! {
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
    ).ok().unwrap();
    
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    
    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    
    let button_pin = pins.gpio16.into_pull_up_input();
    let speaker_pin = pins.gpio21.into_push_pull_output();
    let led_pin = pins.gpio25.into_push_pull_output();
    
    let mut transmitter = Transmitter::new(
        button_pin,
        speaker_pin,
        led_pin,
        timer,
        delay,
    );
    
    transmitter.init();
    transmitter.transmit_morse_input();
    
    loop {}
}