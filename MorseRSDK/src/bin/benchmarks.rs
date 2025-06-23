#![no_std]
#![no_main]

// External crate imports
use cortex_m::delay::Delay;
use cortex_m::peripheral::Peripherals as CorePeripherals;
use panic_halt as _;
use rp2040_hal::{
    clocks::{init_clocks_and_plls, Clock}, fugit::RateExtU32, gpio::FunctionUart, pac::{self, Peripherals}, sio, timer::Timer, uart::{DataBits, StopBits, UartConfig}, Sio, Watchdog
};
use rp_pico::{Pins, XOSC_CRYSTAL_FREQ};

// --- RTT Import ---
use rtt_target::{rtt_init_print, rprintln};

// Import modules from the parent crate
extern crate morse_rsdk;
use morse_rsdk::adc;
use morse_rsdk::gpio;
use morse_rsdk::interrupt;
use morse_rsdk::pwm;
use morse_rsdk::uart;

// Define the benchmark mode as a constant
// 1 = GPIO (HAL)
// 2 = GPIO (Raw)
// 3 = PWM (HAL)
// 4 = PWM (Raw)
// 5 = ADC (HAL)
// 6 = ADC (Raw)
// 7 = Interrupt (HAL)
// 8 = UART (HAL)
// 9 = UART (Raw)
const BENCHMARK_MODE: u8 = 8;

#[rp2040_hal::entry]
fn main() -> ! {
    // --- Initialize RTT ---
    rtt_init_print!();
    
    // --- Initial Setup ---
    let mut pac = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let mut resets = pac.RESETS;

    // --- 1. Initialize Clocks ---
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut resets,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // --- 2. Initialize Timer ---
    let timer = Timer::new(pac.TIMER, &mut resets, &clocks);

    // --- 3. Extract Clock Info ---
    let system_clock_freq = clocks.system_clock.freq();
    let peripheral_clock_freq = clocks.peripheral_clock.freq();

    // --- 5. Initialize Delay ---
    let mut delay = Delay::new(core.SYST, system_clock_freq.to_Hz());

    // --- Initialize SIO and Pins ---
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut resets,
    );

    // --- Print Header using RTT ---
    rprintln!("Raspberry Pi Pico Benchmark Suite");
    rprintln!("Benchmark Mode: {}", BENCHMARK_MODE);
    rprintln!("---------------------------------");

    // --- Run Benchmarks ---
    match BENCHMARK_MODE {
        1 => { // GPIO using HAL abstractions
            rprintln!("Running GPIO Toggle Benchmark (HAL)...");
            let mut toggle_pin = pins.gpio2.into_push_pull_output();
            gpio::benchmark_gpio_toggle(&timer, &mut toggle_pin, &mut delay);
        }
        2 => { // GPIO using raw register access
            rprintln!("Running GPIO Toggle Benchmark (Raw)...");
            gpio::benchmark_gpio_toggle_raw(&timer, 2, &mut delay); // Using GPIO2
        }
        3 => { // PWM using HAL
            rprintln!("Running PWM Benchmark (HAL)...");
            let led_pin = pins.led.into_push_pull_output();
            pwm::benchmark_pwm(
                &timer,
                led_pin,
                pac.PWM,
                &mut resets,
                &mut delay,
                system_clock_freq,
            );
        }
        4 => { // PWM using raw register access
            rprintln!("Running PWM Benchmark (Raw)...");
            pwm::benchmark_pwm_raw(
                &timer,
                25, // LED pin (GPIO25)
                &mut delay,
                system_clock_freq,
            );
        }
        5 => { // ADC using HAL
            rprintln!("Running ADC Benchmark (HAL)...");
            let mut adc_hal = rp2040_hal::adc::Adc::new(pac.ADC, &mut resets);
            let adc_pin_gpio = pins.gpio27.into_floating_input();
            let mut adc_pin = rp2040_hal::adc::AdcPin::new(adc_pin_gpio).unwrap();
            adc::benchmark_adc(
                &timer,
                &mut adc_hal,
                &mut adc_pin,
                &mut delay,
            );
        }
        6 => { // ADC using raw register access
            rprintln!("Running ADC Benchmark (Raw)...");
            adc::benchmark_adc_raw(
                &timer,
                0, // ADC input 0 (GPIO27 = ADC1)
                &mut delay,
            );
        }
        7 => { // Interrupt using HAL
            rprintln!("Running Interrupt Benchmark (HAL)...");
            let button_pin = pins.gpio16.into_pull_up_input();
            let led_pin = pins.led.into_push_pull_output();
            interrupt::benchmark_interrupt(
                &timer,
                button_pin,
                led_pin,
                &mut delay,
                system_clock_freq,
            );
        }
        8 => { // UART using HAL
            rprintln!("Running UART Benchmark (HAL)...");
            let uart_pins = (
                pins.gpio0.into_function::<FunctionUart>(), // TX
                pins.gpio1.into_function::<FunctionUart>(), // RX
            );
            let mut uart = rp2040_hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut resets)
                .enable(
                    UartConfig::new(115_200u32.Hz(), DataBits::Eight, None, StopBits::One),
                    peripheral_clock_freq,
                )
                .unwrap();
            uart::benchmark_uart(
                &mut uart, 
                &timer,
                &mut delay,
            );
        }
        9 => { // UART using raw register access
            rprintln!("Running UART Benchmark (Raw)...");
            uart::benchmark_uart_raw(
                &timer,
                &mut delay,
                115_200, // Baud rate
                system_clock_freq.to_Hz(),
            );
        }
        _ => {
            rprintln!("Invalid benchmark mode selected");
        }
    }

    rprintln!("Selected benchmark finished.");

    // Loop forever instead of exiting
    loop {
        cortex_m::asm::wfi(); // Wait for interrupt
    }
}