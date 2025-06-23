/*
 * RP2040 PWM Configuration Timing Test
 *
 * Evaluates the time required to initialize and configure
 * a PWM channel on the RP2040 microcontroller.
 *
 * Hardware:
 *   - Uses the built-in LED for visual confirmation
 */
#include "pico/stdlib.h"
#include "hardware/pwm.h"
#include <stdio.h>

#define PWM_OUTPUT PICO_DEFAULT_LED_PIN  // Keep using built-in LED
#define TEST_ITERATIONS 100

/*
 * Main PWM setup timing evaluation function
 */
void measure_pwm_setup_time(void) {
    printf("// PWM CONFIGURATION TIMING TEST //\n");
    printf("OUTPUT | Pin=%d | Tests=%d\n", PWM_OUTPUT, TEST_ITERATIONS);
    
    for (int test = 0; test < TEST_ITERATIONS; test++) {
        // Reset pin to standard GPIO mode
        gpio_init(PWM_OUTPUT);
        gpio_set_dir(PWM_OUTPUT, GPIO_OUT);
        gpio_put(PWM_OUTPUT, 0);
        
        // Configure for PWM operation
        gpio_set_function(PWM_OUTPUT, GPIO_FUNC_PWM);
        uint pwm_slice = pwm_gpio_to_slice_num(PWM_OUTPUT);
        
        // Measure PWM setup time
        uint32_t start_time = time_us_32();
        
        // Configure and enable PWM
        pwm_config cfg = pwm_get_default_config();
        pwm_config_set_clkdiv(&cfg, 100.0f);
        pwm_init(pwm_slice, &cfg, false);
        pwm_set_gpio_level(PWM_OUTPUT, 32768);   // 50% duty cycle
        pwm_set_enabled(pwm_slice, true);        // Start PWM output
        
        uint32_t end_time = time_us_32();
        uint32_t setup_time = end_time - start_time;
        
        // Report measurement
        printf("TEST[%d] | SetupTime=%lu µs\n", test, setup_time);
        
        // Demonstrate PWM is working
        sleep_ms(100);
        
        // Disable PWM
        pwm_set_enabled(pwm_slice, false);
        
        // Pause between tests
        sleep_ms(100);
    }
}