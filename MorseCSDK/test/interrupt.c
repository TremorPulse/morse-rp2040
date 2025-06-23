/*
 * RP2040 Interrupt Response Timing
 * 
 * Measures the time between software trigger and interrupt handler execution
 * to evaluate the microcontroller's interrupt processing performance.
 * 
 * Hardware connections:
 * - Button connected to GPIO16 (with internal pull-up)
 * - On-board LED used for visual confirmation
 */
#include "pico/stdlib.h"
#include "hardware/gpio.h"
#include <stdio.h>
#include <stdbool.h>

#define BUTTON_GPIO 16                    // Keep original pin
#define LED_GPIO PICO_DEFAULT_LED_PIN     // On-board LED

volatile uint32_t interrupt_start_time = 0;
volatile uint32_t response_time = 0;
volatile bool led_trigger = false;

/*
 * GPIO interrupt handler function
 * Records the time between trigger and execution
 */
void gpio_event_handler(uint gpio, uint32_t events) {
    response_time = time_us_32() - interrupt_start_time;
    printf("EVENT | Pin=%d | ResponseTime=%lu µs\n", gpio, response_time);
    
    led_trigger = true;
}

/*
 * Main interrupt performance evaluation routine
 */
void measure_interrupt_latency(void) {
    stdio_init_all();
    sleep_ms(3000);  // Allow time for USB connection

    // Configure button input with pull-up
    gpio_init(BUTTON_GPIO);
    gpio_set_dir(BUTTON_GPIO, GPIO_IN);
    gpio_pull_up(BUTTON_GPIO);

    // Setup LED for visual feedback
    gpio_init(LED_GPIO);
    gpio_set_dir(LED_GPIO, GPIO_OUT);
    gpio_put(LED_GPIO, 0);

    // Register interrupt handler
    gpio_set_irq_enabled_with_callback(
        BUTTON_GPIO,
        GPIO_IRQ_EDGE_FALL,  // Trigger on button press
        true,
        &gpio_event_handler
    );

    printf("// INTERRUPT LATENCY MEASUREMENT //\n");
    printf("SETUP | Button=GPIO%d | LED=GPIO%d\n", BUTTON_GPIO, LED_GPIO);
    printf("Press button to measure interrupt response time...\n");

    while (true) {
        // Update timestamp before potential interrupt
        interrupt_start_time = time_us_32();

        // Visual feedback when interrupt occurs
        if (led_trigger) {
            gpio_put(LED_GPIO, 1);
            sleep_ms(100);
            gpio_put(LED_GPIO, 0);
            led_trigger = false;
        }

        sleep_ms(10);  // Reduce CPU usage
    }
}