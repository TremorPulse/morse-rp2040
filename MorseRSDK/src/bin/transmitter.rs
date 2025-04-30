#![no_std]
#![no_main]

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_RAM_MEMCPY;

use core::convert::Into;
use rp2040_hal::{
    gpio::{bank0::{Gpio16, Gpio21, Gpio25}, Pin, FunctionSio, SioOutput, SioInput, PullUp, PullDown},
    pac,
    timer::Timer,
    clocks::{Clock, init_clocks_and_plls},
    watchdog::Watchdog,
    Sio,
    gpio::Pins,
    uart::{UartPeripheral, UartConfig, DataBits, StopBits, Parity},
};
use rp2040_hal::entry;
use rp_pico::XOSC_CRYSTAL_FREQ;
use cortex_m::delay::Delay;
use nb::block;
use embedded_hal::digital::v2::{OutputPin, InputPin};
use rp2040_hal::fugit::RateExtU32;
use morse_rsdk::DEBOUNCE_TIME_MS;
use morse_rsdk::DOT_FREQ;
use morse_rsdk::DASH_FREQ;
use morse_rsdk::SYNC_PATTERN;
use morse_rsdk::DOT_THRESHOLD_MS;
use morse_rsdk::DASH_THRESHOLD_MS;
use morse_rsdk::BAUD_RATE;

pub struct Transmitter<UART> 
where
    UART: embedded_hal::serial::Write<u8>,
{
    uart: UART,
    button_pin: Pin<Gpio16, FunctionSio<SioInput>, PullUp>,
    speaker_pin: Pin<Gpio21, FunctionSio<SioOutput>, PullDown>,
    led_pin: Pin<Gpio25, FunctionSio<SioOutput>, PullDown>,
    timer: Timer,
    delay: Delay,
}

impl<UART> Transmitter<UART>
where
    UART: embedded_hal::serial::Write<u8>,
{
    pub fn new(
        uart: UART,
        button_pin: Pin<Gpio16, FunctionSio<SioInput>, PullUp>,
        speaker_pin: Pin<Gpio21, FunctionSio<SioOutput>, PullDown>,
        led_pin: Pin<Gpio25, FunctionSio<SioOutput>, PullDown>,
        timer: Timer,
        delay: Delay,
    ) -> Self {
        Self {
            uart,
            button_pin,
            speaker_pin,
            led_pin,
            timer,
            delay,
        }
    }

    pub fn init(&mut self) {
        self.uart_log("Transmitter initialized");
    }

    pub fn uart_log(&mut self, message: &str) {
        for byte in message.as_bytes() {
            let _ = block!(self.uart.write(*byte));
        }
        let _ = block!(self.uart.write(b'\r'));
        let _ = block!(self.uart.write(b'\n'));
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
        self.uart_log(".");
        self.transmit_gap(250);
    }

    pub fn transmit_dash(&mut self) {
        self.led_pin.set_high().unwrap();
        self.generate_tone(DASH_FREQ, 750);
        self.led_pin.set_low().unwrap();
        self.uart_log("-");
        self.transmit_gap(250);
    }

    pub fn transmit_gap(&mut self, duration_ms: u32) {
        self.delay.delay_ms(duration_ms);
    }

    pub fn transmit_sync(&mut self) {
        self.uart_log("Transmitting sync pattern...");
        for c in SYNC_PATTERN.chars() {
            match c {
                '.' => self.transmit_dot(),
                '-' => self.transmit_dash(),
                _ => continue,
            }
        }
        self.uart_log("Sync pattern transmitted");
    }

    pub fn transmit_morse_input(&mut self) {
        let mut press_start = 0u64;
        let mut last_release_time = 0u64;
        let mut last_log_time = 0u64;
        let mut button_was_pressed = false;
        let mut in_word = false;

        self.uart_log("Starting Morse transmission...");
        self.uart_log("Ready for input");

        self.transmit_sync();

        loop {
            let current_time = self.timer.get_counter().ticks();
            let button_state = self.button_pin.is_low().unwrap();

            if button_state && !button_was_pressed {
                if current_time - last_release_time > DEBOUNCE_TIME_MS {
                    press_start = current_time;
                    button_was_pressed = true;
                }
            }
            else if !button_state && button_was_pressed {
                if current_time - press_start > DEBOUNCE_TIME_MS.into() {
                    let press_duration = current_time - press_start;

                    if press_duration <= DOT_THRESHOLD_MS.into() {
                        self.transmit_dot();
                    } else if press_duration <= DASH_THRESHOLD_MS.into() {
                        self.transmit_dash();
                    }
                    last_release_time = current_time;
                    in_word = true;
                }
                button_was_pressed = false;
            }
            else if !button_state && in_word {
                let gap_duration = current_time - last_release_time;

                if gap_duration > 1750u64 {
                    self.uart_log("WORD GAP");
                    in_word = false;
                } else if gap_duration > 750u64 && gap_duration <= 1750u64 {
                    if current_time - last_log_time > 750u64 {
                        self.uart_log("CHAR GAP");
                        last_log_time = current_time;
                    }
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
    
    let uart = UartPeripheral::new(
        pac.UART0,
        (pins.gpio0.into_function(), pins.gpio1.into_function()),
        &mut pac.RESETS,
    )
    .enable(
        UartConfig::new(BAUD_RATE.Hz(), DataBits::Eight, core::prelude::v1::Some(Parity::Even), StopBits::One),
        clocks.peripheral_clock.freq(),
    )
    .unwrap();
    
    let button_pin = pins.gpio16.into_pull_up_input();
    let speaker_pin = pins.gpio21.into_push_pull_output();
    let led_pin = pins.gpio25.into_push_pull_output();
    
    let mut transmitter = Transmitter::new(
        uart,
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