 #include <stdio.h>
 #include "pico/stdlib.h"
 #include "hardware/adc.h"
 #include "pico/time.h"
 #include "benchmarks.h"

 #define NUM_ITERATIONS 100  // Number of iterations for each benchmark
 
 void benchmark_adc(void) {
    const int NUM_READS = 1000;
    
    // Initialize ADC subsystem and GPIO26
    adc_init();
    adc_gpio_init(26);     // GP26 = ADC0
    adc_select_input(0);   // Select ADC0 as input
    
    printf("task,method,iteration,reads,avg_time_us\n");
    
    for (int iteration = 0; iteration < NUM_ITERATIONS; iteration++) {
        int64_t total_time = 0;
        
        for (int i = 0; i < NUM_READS; ++i) {
            absolute_time_t start = get_absolute_time();
            uint16_t value = adc_read();  // 12-bit result: 0–4095
            int64_t elapsed = absolute_time_diff_us(start, get_absolute_time());
            total_time += elapsed;
        }
        
        int64_t avg_time = total_time / NUM_READS;
        printf("adc,single_read,%d,%d,%lld\n", iteration, NUM_READS, avg_time);
        
        // Short delay between iterations
        sleep_ms(100);
    }
}