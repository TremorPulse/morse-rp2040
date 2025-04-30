/**
 * @file pwm.c
 * @brief PWM Setup Time Benchmark for RP2040.
 *
 * Modified to use an LED instead of a buzzer for output verification.
 *
 * Wiring:
 *   - GPIO15 (pin 20) → LED with 220Ω resistor → GND
 *   - Or use built-in LED on GP25
 *
 * Output format:
 *   task,method,setup_time_us
 */
#include "pico/stdlib.h"
#include "hardware/pwm.h"
#include <stdio.h>

// You can use the built-in LED if external hardware is an issue
#define PWM_GPIO PICO_DEFAULT_LED_PIN  // On-board LED (usually GP25)
#define NUM_ITERATIONS 100  // Number of iterations for each benchmark


/**
 * @brief Run PWM setup benchmark.
 */
void benchmark_pwm(void) {
    #define PWM_GPIO PICO_DEFAULT_LED_PIN
    
    printf("task,method,iteration,setup_time_us\n");
    
    for (int iteration = 0; iteration < NUM_ITERATIONS; iteration++) {
        // Reset GPIO back to normal mode before each test
        gpio_init(PWM_GPIO);
        gpio_set_dir(PWM_GPIO, GPIO_OUT);
        gpio_put(PWM_GPIO, 0);
        
        // Set to PWM function
        gpio_set_function(PWM_GPIO, GPIO_FUNC_PWM);
        uint slice_num = pwm_gpio_to_slice_num(PWM_GPIO);
        
        uint32_t start = time_us_32();
        
        // Configure PWM peripheral
        pwm_config config = pwm_get_default_config();
        pwm_config_set_clkdiv(&config, 100.0f);
        pwm_init(slice_num, &config, false);
        
        // Set approximately 50% duty cycle
        pwm_set_gpio_level(PWM_GPIO, 32768);      // 50% of 65535
        pwm_set_enabled(slice_num, true);         // Activate PWM
        
        uint32_t end = time_us_32();
        uint32_t duration = end - start;
        
        // Output result
        printf("pwm,setup,%d,%lu\n", iteration, duration);
        
        // Run PWM briefly to show it works
        sleep_ms(100);
        
        // Disable PWM
        pwm_set_enabled(slice_num, false);
        
        // Short delay between iterations
        sleep_ms(100);
    }
}