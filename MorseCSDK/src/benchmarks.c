#include "pico/stdlib.h"
#include "main.h"
#include <stdio.h>
#include "benchmarks.h"

#ifndef BENCHMARK_MODE
#define BENCHMARK_MODE 1
#endif

int main() {
    stdio_init_all();
    sleep_ms(3000);  // Allow time for USB connection
   
    printf("========================================\n");
    printf("RP2040 Performance Evaluation Framework\n");
    printf("Test Mode: %d\n", BENCHMARK_MODE);
    printf("========================================\n");
   
    #if BENCHMARK_MODE == 1
    measure_gpio_performance();
    #elif BENCHMARK_MODE == 2
    measure_pwm_setup_time();
    #elif BENCHMARK_MODE == 3
    measure_adc_performance();
    #elif BENCHMARK_MODE == 4
    measure_interrupt_latency();
    #elif BENCHMARK_MODE == 5
    measure_uart_throughput();
    #else
    printf("ERROR: Invalid test configuration\n");
    #endif
   
    // Keep program running
    while (true) {
        sleep_ms(1000);
    }
   
    return 0;
}