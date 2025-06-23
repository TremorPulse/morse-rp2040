/*
 * Serial Communication Performance Test
 *
 * Evaluates UART transmission speeds with various message sizes
 * using the RP2040's UART peripheral.
 *
 * Connection details:
 *   - UART0: TX on GPIO0, RX on GPIO1 (standard UART pins)
 *   - 115200 baud rate
 */
#include "pico/stdlib.h"
#include "hardware/uart.h"
#include <stdio.h>
#include <string.h>

// UART configuration (keeping original pins)
#define UART_CHANNEL uart0
#define UART_SPEED 115200
#define UART_TX_PIN 0
#define UART_RX_PIN 1
#define TEST_ITERATIONS 100

// Test message lengths
#define MESSAGE_SIZE_COUNT 4
const int message_sizes[MESSAGE_SIZE_COUNT] = {10, 50, 100, 250};

/*
 * UART transmission speed evaluation function
 */
void measure_uart_throughput(void) {
    // Initialize UART hardware
    uart_init(UART_CHANNEL, UART_SPEED);
    gpio_set_function(UART_TX_PIN, GPIO_FUNC_UART);
    gpio_set_function(UART_RX_PIN, GPIO_FUNC_UART);
    
    printf("// SERIAL COMMUNICATION PERFORMANCE TEST //\n");
    printf("UART | Channel=%d | Baud=%d | TX=GPIO%d | RX=GPIO%d\n", 
           0, UART_SPEED, UART_TX_PIN, UART_RX_PIN);
    
    // Prepare test data (filled with 'X' character)
    char test_data[256];
    memset(test_data, 'X', sizeof(test_data));
    
    // Test each message size
    for (int size_idx = 0; size_idx < MESSAGE_SIZE_COUNT; size_idx++) {
        int length = message_sizes[size_idx];
        test_data[length - 1] = '\0';  // Null-terminate at desired length
        
        printf("SIZE_TEST | MessageLength=%d bytes\n", length);
        
        for (int test = 0; test < TEST_ITERATIONS; test++) {
            // Measure transmission time
            uint32_t start_time = time_us_32();
            uart_puts(UART_CHANNEL, test_data);
            uart_tx_wait_blocking(UART_CHANNEL);  // Wait for completion
            uint32_t end_time = time_us_32();
            
            uint32_t tx_time = end_time - start_time;
            float bytes_per_second = (float)(length) * 1000000 / tx_time;
            
            // Output results
            printf("TEST[%d] | Time=%lu µs | Throughput=%.2f B/s\n", 
                   test, tx_time, bytes_per_second);
            
            // Brief pause between tests
            sleep_ms(50);
        }
    }
}