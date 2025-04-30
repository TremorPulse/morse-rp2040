#include "pico/stdlib.h"
#include "benchmarks.h"
#include <stdio.h>

#ifndef BENCHMARK_MODE
#define BENCHMARK_MODE 1
#endif

int main() {
    stdio_init_all();
    sleep_ms(3000);  // Give USB time to connect
   
    printf("Raspberry Pi Pico Benchmark Suite\n");
    printf("Benchmark Mode: %d\n", BENCHMARK_MODE);
    printf("---------------------------------\n");
   
    #if BENCHMARK_MODE == 1
    benchmark_gpio_toggle();
    #elif BENCHMARK_MODE == 2
    benchmark_pwm();
    #elif BENCHMARK_MODE == 3
    benchmark_adc();
    #elif BENCHMARK_MODE == 4
    benchmark_interrupt();
    #elif BENCHMARK_MODE == 5
    benchmark_uart();
    #else
    printf("Invalid benchmark mode selected\n");
    #endif
   
    // Keep the program running
    while (true) {
        sleep_ms(1000);
    }
   
    return 0;
}