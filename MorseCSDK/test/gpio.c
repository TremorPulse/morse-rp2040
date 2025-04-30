 #include "pico/stdlib.h"
 #include "hardware/gpio.h"
 #include <stdio.h>
 
 #define TOGGLE_PIN 2
 #define TOGGLE_COUNT 1000
 #define NUM_ITERATIONS 100  // Number of iterations for each benchmark
 
 // Uncomment for probe/buzzer verification
 //#define SLOW_TOGGLE
 
 
 void benchmark_gpio_toggle(void) {
    #define TOGGLE_PIN 2
    #define TOGGLE_COUNT 1000
    
    gpio_init(TOGGLE_PIN);
    gpio_set_dir(TOGGLE_PIN, GPIO_OUT);
    
    printf("task,method,iteration,toggles,total_time_us,avg_toggle_us\n");
    
    for (int iteration = 0; iteration < NUM_ITERATIONS; iteration++) {
        uint32_t start = time_us_32();
        
        for (int i = 0; i < TOGGLE_COUNT; i++) {
            gpio_put(TOGGLE_PIN, 1);
            gpio_put(TOGGLE_PIN, 0);
        }
        
        uint32_t end = time_us_32();
        uint32_t duration = end - start;
        float avg = (float)duration / TOGGLE_COUNT;
        
        // Output benchmark results
        printf("gpio,toggle,%d,%d,%lu,%.2f\n", iteration, TOGGLE_COUNT, duration, avg);
        
        // Short delay between iterations
        sleep_ms(100);
    }
}