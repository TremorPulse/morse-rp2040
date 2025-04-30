/**
 * @file uart.c
 * @brief UART Performance Benchmark for RP2040.
 *
 * This benchmark measures UART transmission time for various message sizes.
 * It uses the default UART0 peripheral (GPIO0/TX and GPIO1/RX) at 115200 baud.
 *
 * Hardware Setup:
 * - UART0 TX (GPIO0, pin 1) → Optional: Connect to a logic analyzer or loopback to RX
 * - UART0 RX (GPIO1, pin 2) → Optional: For loopback testing
 *
 * Output format:
 *   task,method,bytes,time_us,bytes_per_sec
 *
 * @author Samuel Ivuerah
 */
#include "pico/stdlib.h"
#include "hardware/uart.h"
#include <stdio.h>
#include <string.h>

#define UART_ID uart0
#define BAUD_RATE 115200
#define UART_TX_PIN 0
#define UART_RX_PIN 1
#define NUM_ITERATIONS 100  // Number of iterations for each benchmark

// Test message sizes
#define NUM_TEST_SIZES 4
const int test_sizes[NUM_TEST_SIZES] = {10, 50, 100, 250};


/**
 * @brief Run UART transmission benchmark.
 * 
 * Tests UART transmission speed at various message sizes.
 * Measures time to transmit each message size and calculates throughput.
 */
void benchmark_uart(void) {
    #define UART_ID uart0
    #define BAUD_RATE 115200
    #define UART_TX_PIN 0
    #define UART_RX_PIN 1
    
    // Test message sizes
    #define NUM_TEST_SIZES 4
    const int test_sizes[NUM_TEST_SIZES] = {10, 50, 100, 250};
    
    // Initialize UART
    uart_init(UART_ID, BAUD_RATE);
    gpio_set_function(UART_TX_PIN, GPIO_FUNC_UART);
    gpio_set_function(UART_RX_PIN, GPIO_FUNC_UART);
    
    printf("task,method,iteration,bytes,time_us,bytes_per_sec\n");
    
    // Create test message (filled with 'A')
    char test_message[256];
    memset(test_message, 'A', sizeof(test_message));
    
    // Run tests for different message sizes
    for (int size_idx = 0; size_idx < NUM_TEST_SIZES; size_idx++) {
        int msg_size = test_sizes[size_idx];
        test_message[msg_size - 1] = '\0';  // Null-terminate at the desired length
        
        for (int iteration = 0; iteration < NUM_ITERATIONS; iteration++) {
            // Time the transmission
            uint32_t start = time_us_32();
            uart_puts(UART_ID, test_message);
            // Wait for transmission to complete
            uart_tx_wait_blocking(UART_ID);
            uint32_t end = time_us_32();
            
            uint32_t duration = end - start;
            float bytes_per_sec = (float)(msg_size) * 1000000 / duration;
            
            // Output results
            printf("uart,tx,%d,%d,%lu,%.2f\n", iteration, msg_size, duration, bytes_per_sec);
            
            // Short delay between tests
            sleep_ms(50);
        }
    }
}