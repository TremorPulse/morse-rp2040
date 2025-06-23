/*
 * RP2040 Analog Input Performance Test
 * 
 * Evaluates the ADC conversion speeds across multiple samples
 * and provides statistical performance metrics.
 */
#include <stdio.h>
#include "pico/stdlib.h"
#include "hardware/adc.h"
#include "pico/time.h"
#include "benchmarks.h"

#define SAMPLE_COUNT 1000    // Number of samples per test cycle
#define TEST_CYCLES 100      // Number of test repetitions

void measure_adc_performance(void) {
    // Initialize the ADC hardware and select input
    adc_init();
    adc_gpio_init(27);     // Keep using GP26 (ADC1)
    adc_select_input(0);   // Select ADC0 input channel
    
    printf("// RP2040 ADC PERFORMANCE METRICS //\n");
    printf("TEST_INFO | Samples=%d | Cycles=%d\n", SAMPLE_COUNT, TEST_CYCLES);
    
    for (int cycle = 0; cycle < TEST_CYCLES; cycle++) {
        int64_t total_duration = 0;
        
        for (int i = 0; i < SAMPLE_COUNT; ++i) {
            absolute_time_t begin_time = get_absolute_time();
            uint16_t adc_value = adc_read();  // Perform 12-bit conversion
            int64_t sample_time = absolute_time_diff_us(begin_time, get_absolute_time());
            total_duration += sample_time;
        }
        
        int64_t avg_sample_time = total_duration / SAMPLE_COUNT;
        printf("CYCLE[%d] | Avg_Time=%lld µs | Total_Samples=%d\n", 
               cycle, avg_sample_time, SAMPLE_COUNT);
        
        // Pause between test cycles
        sleep_ms(100);
    }
}