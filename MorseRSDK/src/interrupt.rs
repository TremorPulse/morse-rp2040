//! # GPIO Interrupt Latency Benchmark Function for RP2040
//!
//! Contains the benchmark logic for measuring GPIO interrupt latency.
//! Designed to be called from a central benchmark runner.

#![no_std]

// --- RTT Import ---
use rtt_target::rprintln;

// Alias for our HAL crate
use rp2040_hal::{
    self as hal, fugit, gpio::{
        // Import Gpio pins from bank0
        bank0::{Gpio16, Gpio25}, FunctionSio, Interrupt as GpioInterrupt, Pin, PullDown, PullUp, SioInput, SioOutput
    }, pac::{self, interrupt, Interrupt as HalInterrupt}, timer::Timer, Clock // Import Clock trait for freq()
};

// For interrupt handling
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use cortex_m::delay::Delay;

use embedded_hal_0_2::digital::v2::OutputPin; // Use 0.2 trait for set_high/low

// For sharing peripherals with ISR
use core::cell::RefCell;
use critical_section::Mutex;

// Define GPIO pins used, matching the C code
const BUTTON_GPIO_PIN_NUM: u8 = 16; // GPIO16 for button

// --- Type Aliases for Shared Pins ---
// Define the specific types for the pins we need to share with the ISR
type ButtonPinType = Pin<Gpio16, FunctionSio<SioInput>, PullUp>;
// Although LED is flashed in the main loop, include if ISR needed it later

// --- Static Variables for ISR Communication ---
static IRQ_START_TIME: AtomicU32 = AtomicU32::new(0);
static IRQ_LATENCY: AtomicU32 = AtomicU32::new(0); // Stores raw latency ticks calculated by ISR
static TRIGGER_LED: AtomicBool = AtomicBool::new(false); // Flag set by ISR

// --- Mutex for Sharing Button Pin with ISR ---
// The ISR needs access to the button pin to clear the interrupt.
static SHARED_BUTTON_PIN: Mutex<RefCell<Option<ButtonPinType>>> = Mutex::new(RefCell::new(None));

// --- Interrupt Service Routine (ISR) ---
// Use the correct interrupt attribute
#[interrupt]
fn IO_IRQ_BANK0() {
    critical_section::with(|cs| {
        if let Some(button_pin) = SHARED_BUTTON_PIN.borrow(cs).borrow_mut().as_mut() {
            if button_pin.interrupt_status(GpioInterrupt::EdgeLow) {
                let timer = unsafe { &*pac::TIMER::ptr() };
                let end_time_raw = timer.timerawl().read().bits();
                let start_time_raw = IRQ_START_TIME.load(Ordering::Relaxed);
                let latency_raw = end_time_raw.wrapping_sub(start_time_raw);
                IRQ_LATENCY.store(latency_raw, Ordering::Relaxed);
                TRIGGER_LED.store(true, Ordering::Relaxed);
                button_pin.clear_interrupt(GpioInterrupt::EdgeLow);
            }
        }
    });
}

pub fn benchmark_interrupt(
    timer: &Timer,
    mut button_pin: ButtonPinType,
    mut led_pin: Pin<Gpio25, FunctionSio<SioOutput>, PullDown>,
    delay: &mut Delay,
    clock_freq: fugit::Rate<u32, 1, 1>,
) {
    // Initialize button pin with proper interrupt configuration
    rprintln!("Initializing interrupt benchmark...");
    delay.delay_ms(1000);
    
    // Make sure LED is off at start
    led_pin.set_low().ok();
    
    // Configure the button pin for interrupts
    button_pin.set_interrupt_enabled(GpioInterrupt::EdgeLow, false); // Disable first
    button_pin.clear_interrupt(GpioInterrupt::EdgeLow); // Clear any pending interrupts
    button_pin.set_interrupt_enabled(GpioInterrupt::EdgeLow, true); // Now enable
    
    // Share the button pin with the ISR
    critical_section::with(|cs| {
        SHARED_BUTTON_PIN.borrow(cs).replace(Some(button_pin));
    });

    // Enable the IO_IRQ_BANK0 interrupt in NVIC
    unsafe {
        rp2040_hal::pac::NVIC::unpend(HalInterrupt::IO_IRQ_BANK0);
        rp2040_hal::pac::NVIC::unmask(HalInterrupt::IO_IRQ_BANK0);
    }

    rprintln!("Benchmark: Interrupt Latency");
    rprintln!("task,method,latency_us");
    rprintln!("Press the button to trigger interrupt...");

    let num_triggers_to_run = 100; // Reduced for testing
    let mut triggers_counted = 0;

    // Store initial time
    IRQ_START_TIME.store(timer.get_counter_low(), Ordering::Relaxed);

    while triggers_counted < num_triggers_to_run {
        // Update start time periodically
        IRQ_START_TIME.store(timer.get_counter_low(), Ordering::Relaxed);

        // Check if interrupt triggered
        if TRIGGER_LED.load(Ordering::Relaxed) {
            TRIGGER_LED.store(false, Ordering::Relaxed);
            triggers_counted += 1;

            let latency_raw = IRQ_LATENCY.load(Ordering::Relaxed);
            let sys_freq_hz = clock_freq.to_Hz();
            let latency_us = if sys_freq_hz > 0 {
                (latency_raw as u64 * 1_000_000 / sys_freq_hz as u64) as u32
            } else {
                0
            };

            rprintln!("interrupt,triggered,{}", latency_us);

            led_pin.set_high().ok();
            delay.delay_ms(100);
            led_pin.set_low().ok();
            
            // Wait for button release to prevent multiple triggers
            delay.delay_ms(200);
        }
        
        // Short delay to avoid tight loop
        delay.delay_ms(5);
    }

    // Disable the interrupt when finished
    unsafe {
        rp2040_hal::pac::NVIC::mask(HalInterrupt::IO_IRQ_BANK0);
    }
    
    // Return ownership of the button pin
    let _button_pin_returned = critical_section::with(|cs| {
        SHARED_BUTTON_PIN.borrow(cs).replace(None)
    });

    rprintln!("Interrupt benchmark finished after {} triggers.", num_triggers_to_run);
}