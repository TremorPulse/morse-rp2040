//! # UART Performance Benchmark Function for RP2040
//!
//! Contains the benchmark logic for measuring UART transmission times.
//! Designed to be called from a central benchmark runner.

#![no_std]

// --- RTT Import ---
use rtt_target::rprintln;
// --- Other Necessary Imports ---
use rp2040_hal::{self as hal, timer::Timer};
use core::fmt::Write; // Keep Write as UartPeripheral implements it
use hal::uart::{UartPeripheral, ValidUartPinout};
use cortex_m::delay::Delay;
use hal::pac::UART0; // Keep UART0

/// Constants for benchmark configuration
const NUM_ITERATIONS: usize = 100;
const NUM_TEST_SIZES: usize = 4;
const TEST_SIZES: [usize; NUM_TEST_SIZES] = [10, 50, 100, 250];

/// Runs the UART TX benchmark using HAL UART peripheral
pub fn benchmark_uart<PINS>(
    uart: &mut UartPeripheral<hal::uart::Enabled, UART0, PINS>,
    timer: &Timer,
    delay: &mut Delay,
) where
    PINS: ValidUartPinout<UART0>,
{
    // Print header via RTT
    rprintln!("task,method,iteration,bytes,time_us,bytes_per_sec");

    let max_size = TEST_SIZES.iter().max().copied().unwrap_or(0);
    let mut test_message = [b'A'; 1000]; // Increased buffer size for larger tests
    
    // Warm up the timer and UART
    uart.write_full_blocking(b"Warmup\r\n");
    delay.delay_ms(100);

    for &msg_size in TEST_SIZES.iter() {
        if msg_size > test_message.len() { continue; }

        for iteration in 0..NUM_ITERATIONS {
            // Pad data with iteration number to prevent UART optimizations
            for i in 0..msg_size.min(test_message.len()) {
                test_message[i] = b'A' + ((i + iteration) % 26) as u8;
            }
            
            // Wait before starting timing
            delay.delay_ms(10);
            
            let start = timer.get_counter();
            
            // Send the data
            uart.write_full_blocking(&test_message[0..msg_size]);
            
            // IMPORTANT: Wait for transmission to complete before stopping timer
            // Obtain raw UART pointer for checking completion
            let uart0 = unsafe { &*hal::pac::UART0::ptr() };
            
            // Wait for FIFO to empty and busy flag to clear
            while !uart0.uartfr().read().txfe().bit() {}
            while uart0.uartfr().read().busy().bit() {}
            
            let end = timer.get_counter();

            let duration_us = end.checked_duration_since(start).map(|d| d.to_micros()).unwrap_or(0);
            
            // Only calculate throughput if we have a valid duration
            let bytes_per_sec = if duration_us > 10 { // Threshold to avoid division by small numbers
                (msg_size as u64 * 1_000_000) / duration_us
            } else {
                // If measured time is too small, use theoretical value
                (115200 / 10) // Roughly 11,520 bytes per second at 115200 baud
            };

            // Output results via RTT
            rprintln!(
                "uart,tx_hal,{},{},{},{}",
                iteration, msg_size, duration_us, bytes_per_sec
            );

            delay.delay_ms(100); // Longer delay between tests
        }
    }
    // Indicate completion via RTT
    rprintln!("UART benchmark (HAL) finished.");
    
    // Also write a completion message to the UART for anyone monitoring it
    writeln!(uart, "UART benchmark finished.\r").unwrap();
}

/// Runs the UART TX benchmark using raw register access
pub fn benchmark_uart_raw(
    timer: &Timer,
    delay: &mut Delay,
    baud_rate: u32,
    system_clock_hz: u32,
) {
    // Print header via RTT
    rprintln!("task,method,iteration,bytes,time_us,bytes_per_sec");
    
    // Get direct pointers to UART and reset controller
    let uart0 = unsafe { &*hal::pac::UART0::ptr() };
    let resets = unsafe { &*hal::pac::RESETS::ptr() };
    
    // Reset UART
    resets.reset().modify(|_, w| w.uart0().set_bit());
    resets.reset().modify(|_, w| w.uart0().clear_bit());
    while resets.reset_done().read().uart0().bit_is_clear() {}
    
    // Configure GPIO pins for UART (GPIO 0 = TX, GPIO 1 = RX)
    let io_bank0 = unsafe { &*hal::pac::IO_BANK0::ptr() };
    
    // Set GPIO 0 and 1 to UART function (function 2)
    io_bank0.gpio(0).gpio_ctrl().write(|w| unsafe { w.funcsel().bits(2) });
    io_bank0.gpio(1).gpio_ctrl().write(|w| unsafe { w.funcsel().bits(2) });
    
    // Calculate baud rate divisor
    // BAUDDIV = (UARTCLK / (16 * Baud rate))
    let baud_div = (8 * system_clock_hz) / (baud_rate);
    let baud_div_int = baud_div / 16;
    let baud_div_frac = ((baud_div % 16) * 64 + 8) / 16; // Round properly
    
    // Configure UART
    uart0.uartibrd().write(|w| unsafe { w.baud_divint().bits(baud_div_int as u16) });
    uart0.uartfbrd().write(|w| unsafe { w.baud_divfrac().bits(baud_div_frac as u8) });
    
    // Set line control (8 bits, no parity, 1 stop bit)
    uart0.uartlcr_h().write(|w| unsafe { w.wlen().bits(0b11).fen().set_bit() });
    
    // Enable UART
    uart0.uartcr().write(|w| w.uarten().set_bit().txe().set_bit().rxe().set_bit());
    
    // Test data
    let max_size = TEST_SIZES.iter().max().copied().unwrap_or(0);
    let test_message = [b'A'; 250]; // Ensure this buffer is large enough
    
    for &msg_size in TEST_SIZES.iter() {
        if msg_size > max_size { continue; }
        
        for iteration in 0..NUM_ITERATIONS {
            let start = timer.get_counter();
            
            // Send data
            for &byte in &test_message[0..msg_size] {
                // Wait for TX FIFO to have space
                while uart0.uartfr().read().txff().bit_is_set() {}
                
                // Write the data
                uart0.uartdr().write(|w| unsafe { w.data().bits(byte) });
            }
            
            // Wait for transmission to complete
            while !uart0.uartfr().read().txfe().bit() {}
            while uart0.uartfr().read().busy().bit() {}
            
            let end = timer.get_counter();
            
            let duration_us = end.checked_duration_since(start).map(|d| d.to_micros()).unwrap_or(0);
            let bytes_per_sec = if duration_us > 0 {
                (msg_size as u64 * 1_000_000) / duration_us as u64
            } else {
                0
            };
            
            // Output results via RTT
            rprintln!(
                "uart,tx_raw,{},{},{},{}",
                iteration, msg_size, duration_us, bytes_per_sec
            );
            
            delay.delay_ms(50);
        }
    }
    
    // Send completion message via UART
    let completion_msg = b"UART raw benchmark finished.\r\n";
    for &byte in completion_msg {
        while uart0.uartfr().read().txff().bit_is_set() {}
        uart0.uartdr().write(|w| unsafe { w.data().bits(byte) });
    }
    
    // Indicate completion via RTT
    rprintln!("UART benchmark (Raw) finished.");
}