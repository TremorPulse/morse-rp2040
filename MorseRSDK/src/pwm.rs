//! # PWM Setup Time Benchmark Function for RP2040
//!
//! Contains the benchmark logic for measuring PWM setup time.
//! Designed to be called from a central benchmark runner.

#![no_std]

// --- RTT Import ---
use rtt_target::rprintln;
// --- Other Necessary Imports ---
use embedded_hal::pwm::SetDutyCycle;
use rp2040_hal::{
    self as hal, fugit, gpio::AnyPin, pac::{self, RESETS}, pwm::{self, Pwm4, B, ValidPwmOutputPin}, timer::Timer
};
use embedded_hal_0_2::blocking::delay::DelayMs;

/// Number of iterations for the benchmark
const NUM_ITERATIONS: usize = 100;

// HAL version of the PWM benchmark
pub fn benchmark_pwm<P, T>(
    timer: &Timer,
    pin: P, // The pin passed in
    pwm_peripheral: pac::PWM,
    resets: &mut RESETS,
    delay: &mut T,
    _clock_freq: fugit::Rate<u32, 1, 1>, // Prefix with _ as it's not directly used here
) where
    P: AnyPin + 'static, // Keep AnyPin if needed for output_to, OutputPin not needed here
    T: DelayMs<u32>,
    <P as AnyPin>::Id: ValidPwmOutputPin<Pwm4, B>, // Constraint for output_to
{
    rprintln!("task,method,iteration,setup_time_us");

    let mut pwm_slices = hal::pwm::Slices::new(pwm_peripheral, resets);
    let pwm = &mut pwm_slices.pwm4;
    pwm.enable();

    // Configure channel B for the pin
    let _led_pin_pwm = pwm.channel_b.output_to(pin); // pin is consumed here

    for iteration in 0..NUM_ITERATIONS {
        pwm.disable();
        pwm.channel_b.set_duty_cycle(0).unwrap();
        let start = timer.get_counter().ticks();

        pwm.set_div_int(100);
        pwm.set_div_frac(0);
        let _ = pwm.channel_b.set_duty_cycle(32768);
        pwm.enable();

        let end = timer.get_counter().ticks();
        let duration = end.wrapping_sub(start);

        rprintln!("pwm,setup_hal,{},{}", iteration, duration);

        delay.delay_ms(100);
        pwm.disable();
        delay.delay_ms(100);
    }
    rprintln!("PWM benchmark (HAL) finished.");
}

// Bare metal version of the PWM benchmark
pub fn benchmark_pwm_raw<T>(
    timer: &Timer,
    pin_number: u8,  // GPIO pin number
    delay: &mut T,
    _clock_freq: fugit::Rate<u32, 1, 1>,
) where
    T: DelayMs<u32>,
{
    rprintln!("task,method,iteration,setup_time_us");

    // Directly access the PWM hardware
    let pwm = unsafe { &*pac::PWM::ptr() };
    let resets = unsafe { &*pac::RESETS::ptr() };
    
    // Reset PWM
    resets.reset().modify(|_, w| w.pwm().set_bit());
    resets.reset().modify(|_, w| w.pwm().clear_bit());
    while resets.reset_done().read().pwm().bit_is_clear() {}
    
    // Configure GPIO for PWM function
    let io_bank0 = unsafe { &*pac::IO_BANK0::ptr() };
    let pads_bank0 = unsafe { &*pac::PADS_BANK0::ptr() };
    
    // Set pin to PWM function (function 4)
    if pin_number <= 29 {
        // Convert u8 to usize for gpio() method
        let pin_idx = pin_number as usize;
        
        // Configure pin for PWM function
        io_bank0.gpio(pin_idx).gpio_ctrl().write(|w| unsafe { w.funcsel().bits(4) });
        pads_bank0.gpio(pin_idx).write(|w| w.od().clear_bit().ie().set_bit());
    } else {
        return; // Invalid pin
    }
    
    // PWM4 = slice 4, Channel B
    let slice_num = 4;
    let chan_b = true;
    
    for iteration in 0..NUM_ITERATIONS {
        // Disable PWM
        pwm.ch(slice_num).csr().modify(|_, w| w.en().clear_bit());
        
        // Set channel B duty cycle to 0
        pwm.ch(slice_num).cc().modify(|_, w| unsafe { w.b().bits(0) });
        
        let start = timer.get_counter().ticks();
        
        // Set divider
        pwm.ch(slice_num).div().write(|w| unsafe { 
            w.int().bits(100).frac().bits(0)
        });
        
        // Set top value (period)
        pwm.ch(slice_num).top().write(|w| unsafe { w.bits(65535) });
        
        // Set duty cycle
        if chan_b {
            pwm.ch(slice_num).cc().modify(|_, w| unsafe { w.b().bits(32768) });
        }
        
        // Enable PWM
        pwm.ch(slice_num).csr().modify(|_, w| w.en().set_bit());
        
        let end = timer.get_counter().ticks();
        let duration = end.wrapping_sub(start);
        
        rprintln!("pwm,setup_raw,{},{}", iteration, duration);
        
        delay.delay_ms(100);
        
        // Disable PWM
        pwm.ch(slice_num).csr().modify(|_, w| w.en().clear_bit());
        
        delay.delay_ms(100);
    }
    
    rprintln!("PWM benchmark (Raw) finished.");
}