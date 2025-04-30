#include <stdio.h>
#include "main.h"

const char* morse_code[] = {
    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..",
    ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.",
    "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."
};

void init_uart() {
    stdio_init_all();
    sleep_ms(2000);  // Give USB time to initialize
    
    uart_init(UART_ID, BAUD_RATE);
    gpio_set_function(UART_TX_PIN, GPIO_FUNC_UART);
    gpio_set_function(UART_RX_PIN, GPIO_FUNC_UART);
    
    printf("UART Initialized\n");
}

void uart_log(const char* message) {
    uart_puts(UART_ID, message);
    uart_puts(UART_ID, "\r\n");
    printf("Sent: %s\n", message);
}

void init_transmitter() {
    init_uart();
    
    gpio_init(BUTTON_PIN);
    gpio_set_dir(BUTTON_PIN, GPIO_IN);
    gpio_pull_up(BUTTON_PIN);
    
    gpio_init(SPEAKER_PIN);
    gpio_set_dir(SPEAKER_PIN, GPIO_OUT);
    
    gpio_init(PICO_DEFAULT_LED_PIN);
    gpio_set_dir(PICO_DEFAULT_LED_PIN, GPIO_OUT);
    
    printf("Transmitter initialized.\n");
}

void generate_tone(uint32_t freq_hz, uint32_t duration_ms) {
    uint32_t period_us = 1000000 / freq_hz;
    uint32_t cycles = (duration_ms * 1000) / period_us;
    
    for (uint32_t i = 0; i < cycles; i++) {
        gpio_put(SPEAKER_PIN, 1);
        sleep_us(period_us / 2);
        gpio_put(SPEAKER_PIN, 0);
        sleep_us(period_us / 2);
    }
}

void transmit_dot() {
    gpio_put(PICO_DEFAULT_LED_PIN, 1);
    generate_tone(DOT_FREQ, DOT_DURATION);
    gpio_put(PICO_DEFAULT_LED_PIN, 0);
    uart_log(".");
    transmit_gap(INTRA_CHAR_GAP);
}

void transmit_dash() {
    gpio_put(PICO_DEFAULT_LED_PIN, 1);
    generate_tone(DASH_FREQ, DASH_DURATION);
    gpio_put(PICO_DEFAULT_LED_PIN, 0);
    uart_log("-");
    transmit_gap(INTRA_CHAR_GAP);
}

void transmit_gap(uint32_t duration) {
    sleep_ms(duration);
}

void transmit_sync() {
    uart_log("Transmitting sync pattern...");
    for (const char* p = SYNC_PATTERN; *p; p++) {
        if (*p == '.') {
            transmit_dot();
        } else if (*p == '-') {
            transmit_dash();
        }
    }
    uart_log("Sync pattern transmitted");
}

void transmit_morse_input() {
    uint32_t press_start = 0;
    uint32_t last_release_time = 0;
    uint32_t last_log_time = 0;
    bool button_was_pressed = false;
    bool in_word = false;

    uart_log("Starting Morse transmission...");
    printf("Ready for input\n");

    transmit_sync();

    while (true) {
        uint32_t current_time = to_ms_since_boot(get_absolute_time());
        bool button_state = !gpio_get(BUTTON_PIN);  // Inverted because of pull-up

        // Button just pressed
        if (button_state && !button_was_pressed) {
            if (current_time - last_release_time > DEBOUNCE_TIME_MS) {
                press_start = current_time;
                button_was_pressed = true;
            }
        }
        // Button just released
        else if (!button_state && button_was_pressed) {
            if (current_time - press_start > DEBOUNCE_TIME_MS) {
                uint32_t press_duration = current_time - press_start;

                if (press_duration <= DOT_THRESHOLD_MS) {
                    transmit_dot();
                } else if (press_duration <= DASH_THRESHOLD_MS) {
                    transmit_dash();
                }
                last_release_time = current_time;
                in_word = true;
            }
            button_was_pressed = false;
        }
        // Handle gaps
        else if (!button_state && in_word) {
            uint32_t gap_duration = current_time - last_release_time;

            if (gap_duration > WORD_GAP) {
                uart_log("WORD GAP");
                in_word = false;
            } else if (gap_duration > INTER_CHAR_GAP && gap_duration <= WORD_GAP) {
                if (current_time - last_log_time > INTER_CHAR_GAP) {  // Prevent repeated logging
                    uart_log("CHAR GAP");
                    last_log_time = current_time;
                }
            }
        }

        sleep_ms(10);  // Prevent tight polling
    }
}

int main() {
    init_transmitter();
    printf("Starting morse code transmission program\n");
    transmit_morse_input();
    return 0;
}