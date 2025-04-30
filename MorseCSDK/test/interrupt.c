/**
 * @file interrupt.c
 * @brief GPIO Interrupt Latency Benchmark for RP2040.
 *
 * Modified to use the onboard LED instead of buzzer
 *
 * Wiring:
 *   - GPIO14 (pin 19) → One leg of button
 *   - Other button leg → GND (we'll use internal pull-up)
 *   - Uses built-in LED on GP25 for visual indicator
 */
#include "pico/stdlib.h"
#include "hardware/gpio.h"
#include <stdio.h>
#include <stdbool.h>

#define BUTTON_GPIO 16                    // GPIO16 
#define LED_GPIO PICO_DEFAULT_LED_PIN     // On-board LED (usually GP25)

volatile uint32_t irq_start_time = 0;
volatile uint32_t irq_latency = 0;
volatile bool trigger_led = false;

/**
 * @brief GPIO interrupt callback function.
 */
void gpio_irq_callback(uint gpio, uint32_t events) {
    irq_latency = time_us_32() - irq_start_time;
    printf("interrupt,triggered,%lu\n", irq_latency);
    
    trigger_led = true;
}

/**
 * @brief Run the interrupt latency benchmark using a button press.
 */
void benchmark_interrupt(void) {
    stdio_init_all();
    sleep_ms(3000);  // Give USB time to connect

    gpio_init(BUTTON_GPIO);
    gpio_set_dir(BUTTON_GPIO, GPIO_IN);
    gpio_pull_up(BUTTON_GPIO);  // Use pull-up instead of pull-down

    gpio_init(LED_GPIO);
    gpio_set_dir(LED_GPIO, GPIO_OUT);
    gpio_put(LED_GPIO, 0);

    gpio_set_irq_enabled_with_callback(
        BUTTON_GPIO,
        GPIO_IRQ_EDGE_FALL,  // Trigger on falling edge (button press)
        true,
        &gpio_irq_callback
    );

    printf("Benchmark: Interrupt Latency\n");
    printf("task,method,latency_us\n");
    printf("Press the button to trigger interrupt...\n");

    while (true) {
        // Capture timestamp before polling for event
        irq_start_time = time_us_32();

        // If ISR was triggered, flash LED briefly
        if (trigger_led) {
            gpio_put(LED_GPIO, 1);
            sleep_ms(100);
            gpio_put(LED_GPIO, 0);
            trigger_led = false;
        }

        sleep_ms(10);  // Slight delay to reduce CPU usage
    }
}