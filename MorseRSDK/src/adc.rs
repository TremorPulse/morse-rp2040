//! # ADC Read Time Benchmark Function for RP2040
//!
//! Contains the benchmark logic for measuring ADC read times.
//! Designed to be called from a central benchmark runner.

#![no_std]
// --- RTT Import ---
use rtt_target::rprintln;
// --- Other Necessary Imports ---
use rp2040_hal::{self as hal, timer::Timer};
use embedded_hal_0_2::adc::OneShot;
use hal::adc::Adc;
use cortex_m::delay::Delay;

/// Number of iterations for the benchmark outer loop
const NUM_ITERATIONS: usize = 100;
/// Number of ADC reads per iteration
const NUM_READS: usize = 1000;

// HAL version of the ADC benchmark
pub fn benchmark_adc<ADCPIN>(
    timer: &Timer,
    adc: &mut Adc,
    adc_pin: &mut ADCPIN, // Generic AdcPin type
    delay: &mut Delay,
) where
    ADCPIN: embedded_hal_0_2::adc::Channel<Adc, ID = u8>,
{
    rprintln!("task,method,iteration,reads,avg_time_us");

    for iteration in 0..NUM_ITERATIONS {
        let mut total_time: u64 = 0;

        for _ in 0..NUM_READS {
            let start = timer.get_counter();
            let _value: u16 = adc.read(adc_pin).unwrap();
            let end = timer.get_counter();

            let duration = end.checked_duration_since(start).map(|d| d.to_micros()).unwrap_or(0);
            total_time += duration;
        }

        let avg_time = if NUM_READS > 0 {
            total_time / (NUM_READS as u64)
        } else {
            0
        };

        rprintln!(
            "adc,hal_read,{},{},{}",
            iteration, NUM_READS, avg_time
        );

        delay.delay_ms(100);
    }
    rprintln!("ADC benchmark (HAL) finished.");
}

// Bare metal version of the ADC benchmark
pub fn benchmark_adc_raw(
    timer: &Timer,
    adc_input: u8, // ADC input number (0-3)
    delay: &mut Delay,
) {
    rprintln!("task,method,iteration,reads,avg_time_us");
    
    // Get direct pointer to ADC
    let adc = unsafe { &*hal::pac::ADC::ptr() };
    let resets = unsafe { &*hal::pac::RESETS::ptr() };
    
    // Reset ADC
    resets.reset().modify(|_, w| w.adc().set_bit());
    resets.reset().modify(|_, w| w.adc().clear_bit());
    while resets.reset_done().read().adc().bit_is_clear() {}
    
    // Configure ADC
    adc.cs().write(|w| {
        unsafe { w.en().set_bit()       // Enable ADC
         .ainsel().bits(adc_input) // Select input
         .start_once().clear_bit() } // Don't start yet
    });
    
    for iteration in 0..NUM_ITERATIONS {
        let mut total_time: u64 = 0;

        for _ in 0..NUM_READS {
            let start = timer.get_counter();
            
            // Start conversion
            adc.cs().modify(|_, w| w.start_once().set_bit());
            
            // Wait for conversion to complete
            while adc.cs().read().ready().bit_is_clear() {}
            
            // Read result
            let _value = adc.result().read().bits();
            
            let end = timer.get_counter();
            let duration = end.checked_duration_since(start).map(|d| d.to_micros()).unwrap_or(0);
            total_time += duration;
        }

        let avg_time = if NUM_READS > 0 {
            total_time / (NUM_READS as u64)
        } else {
            0
        };

        rprintln!(
            "adc,raw_read,{},{},{}",
            iteration, NUM_READS, avg_time
        );

        delay.delay_ms(100);
    }
    
    // Disable ADC
    adc.cs().modify(|_, w| w.en().clear_bit());
    
    rprintln!("ADC benchmark (Raw) finished.");
}