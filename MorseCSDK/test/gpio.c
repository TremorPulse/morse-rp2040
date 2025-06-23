/*
 * GPIO Switching Speed Evaluation
 * 
 * Tests the RP2040's digital output performance by
 * measuring the time required for repeated pin toggles.
 */
#include "pico/stdlib.h"
#include "hardware/gpio.h"
#include <stdio.h>

// Test parameters
#define TEST_PIN 5          // Using GPIO5 (changed from GPIO2)
#define COUNT 1000   // Number of on/off cycles per test
#define TEST_RUNS 100       // Number of test repetitions

void measure_gpio_performance(void) {
    // Configure GPIO for output
    gpio_init(TEST_PIN);
    gpio_set_dir(TEST_PIN, GPIO_OUT);
    
    printf("// GPIO TOGGLE PERFORMANCE TEST //\n");
    printf("CONFIG | Pin=%d | ToggleCount=%d | TestRuns=%d\n", 
           TEST_PIN, COUNT, TEST_RUNS);
    
    for (int run = 0; run < TEST_RUNS; run++) {
        uint32_t start_time = time_us_32();
        
        // Perform rapid GPIO toggling
        for (int i = 0; i < COUNT; i++) {
            gpio_put(TEST_PIN, 1);  // Set high
            gpio_put(TEST_PIN, 0);  // Set low
        }
        
        uint32_t end_time = time_us_32();
        uint32_t total_time = end_time - start_time;
        float toggle_rate = (float)total_time / COUNT;
        
        // Report results
        printf("RUN[%d] | TotalTime=%lu µs | PerToggle=%.2f µs\n", 
               run, total_time, toggle_rate);
        
        // Brief pause between test runs
        sleep_ms(100);
    }
}