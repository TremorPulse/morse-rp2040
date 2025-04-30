#include <stdio.h>
#include <string.h>
#include "main.h"

// Message buffer to display on LCD
char display_message[MAX_MESSAGE_LENGTH] = "";
int message_index = 0;
uint8_t actual_lcd_address = LCD_ADDRESS;  // Will be updated during scan
bool lcd_available = false;                // Flag to track if LCD is working

// Morse code patterns for A-Z
const char* morse_code[] = {
    ".-",     // A
    "-...",   // B
    "-.-.",   // C
    "-..",    // D
    ".",      // E
    "..-.",   // F
    "--.",    // G
    "....",   // H
    "..",     // I
    ".---",   // J
    "-.-",    // K
    ".-..",   // L
    "--",     // M
    "-.",     // N
    "---",    // O
    ".--.",   // P
    "--.-",   // Q
    ".-.",    // R
    "...",    // S
    "-",      // T
    "..-",    // U
    "...-",   // V
    ".--",    // W
    "-..-",   // X
    "-.--",   // Y
    "--.."    // Z
};

// Segment encoding for A-Z (7-segment display)
const uint8_t segment_encoding[] = {
    0b1110111,  // A
    0b0011111,  // B
    0b1001110,  // C
    0b0111101,  // D
    0b1001111,  // E
    0b1000111,  // F
    0b1011110,  // G
    0b0110111,  // H
    0b0110000,  // I
    0b0111100,  // J
    0b1110110,  // K
    0b0001110,  // L
    0b1010101,  // M
    0b0010101,  // N
    0b1111110,  // O
    0b1100111,  // P
    0b1110011,  // Q
    0b0000101,  // R
    0b1011011,  // S
    0b0001111,  // T
    0b0111110,  // U
    0b0111110,  // V
    0b1111010,  // W
    0b0110111,  // X
    0b0111011,  // Y
    0b1101101   // Z
};

// Forward declarations
void uart_log(const char* message);

// I2C helper functions
bool i2c_scan_for_lcd() {
    printf("Scanning I2C bus for devices...\n");
    printf("Using SDA_PIN: %d, SCL_PIN: %d\n", SDA_PIN, SCL_PIN);
    
    bool device_found = false;
    
    for (uint8_t addr = 0; addr < 128; addr++) {
        // Skip reserved addresses
        if (addr >= 0x00 && addr <= 0x07) continue;
        if (addr >= 0x78 && addr <= 0x7F) continue;
        
        uint8_t dummy = 0;
        int result = i2c_write_blocking(i2c0, addr, &dummy, 1, false);
        
        if (result >= 0) {
            printf("I2C device found at address 0x%02X\n", addr);
            device_found = true;
            
            // If we find our expected LCD address, use it
            if (addr == LCD_ADDRESS) {
                printf("Found LCD at expected address 0x%02X\n", addr);
                actual_lcd_address = addr;
                return true;
            } else if (actual_lcd_address == LCD_ADDRESS) {
                // Otherwise use the first device we find
                printf("Assuming device at 0x%02X is the LCD\n", addr);
                actual_lcd_address = addr;
            }
        }
    }
    
    if (!device_found) {
        printf("No I2C devices found! Check connections.\n");
        return false;
    }
    
    printf("Using I2C address 0x%02X for LCD\n", actual_lcd_address);
    return true;
}

// Basic I2C write
bool lcd_write_byte(uint8_t byte_value) {
    int result = i2c_write_blocking(i2c0, actual_lcd_address, &byte_value, 1, false);
    return result == 1;
}

// LCD functions
void lcd_pulse_enable(uint8_t data) {
    // Send with enable bit high
    lcd_write_byte(data | LCD_EN_BIT | LCD_BACKLIGHT);
    sleep_us(500);  // Enable pulse must be >450ns
    
    // Send with enable bit low
    lcd_write_byte((data & ~LCD_EN_BIT) | LCD_BACKLIGHT);
    sleep_us(100);  // Commands need time to settle
}

// Write 4 bits to LCD
void lcd_write_4bits(uint8_t value) {
    uint8_t data = (value & 0xF0) | LCD_BACKLIGHT;  // High nibble, backlight on
    lcd_pulse_enable(data);
}

// Write a command to the LCD
void lcd_command(uint8_t command) {
    // High nibble with RS=0 (command mode)
    uint8_t high_nibble = (command & 0xF0) | LCD_BACKLIGHT;
    lcd_pulse_enable(high_nibble);
    
    // Low nibble with RS=0 (command mode)
    uint8_t low_nibble = ((command << 4) & 0xF0) | LCD_BACKLIGHT;
    lcd_pulse_enable(low_nibble);
    
    // Most commands need >37us to settle
    sleep_ms(2);
}

// Write data to the LCD
void lcd_data(uint8_t data) {
    // High nibble with RS=1 (data mode)
    uint8_t high_nibble = (data & 0xF0) | LCD_RS_BIT | LCD_BACKLIGHT;
    lcd_pulse_enable(high_nibble);
    
    // Low nibble with RS=1 (data mode)
    uint8_t low_nibble = ((data << 4) & 0xF0) | LCD_RS_BIT | LCD_BACKLIGHT;
    lcd_pulse_enable(low_nibble);
    
    sleep_us(100);
}

bool lcd_init() {
    printf("Initializing I2C for LCD...\n");
    
    // Initialize I2C at a slower speed for reliability
    i2c_init(i2c0, 100 * 1000);  // 100 kHz
    
    // Set up I2C pins
    gpio_set_function(SDA_PIN, GPIO_FUNC_I2C);
    gpio_set_function(SCL_PIN, GPIO_FUNC_I2C);
    gpio_pull_up(SDA_PIN);
    gpio_pull_up(SCL_PIN);
    
    // Scan for I2C devices
    if (!i2c_scan_for_lcd()) {
        printf("Failed to find LCD on I2C bus\n");
        return false;
    }
    
    // Wait for LCD to power up completely
    sleep_ms(50);
    
    // Turn on backlight
    lcd_write_byte(LCD_BACKLIGHT);
    sleep_ms(50);
    
    // According to datasheet, we need to send 8-bit mode command 3 times
    // Send 0011
    lcd_write_4bits(0x30);
    sleep_ms(5);
    
    // Send 0011 again
    lcd_write_4bits(0x30);
    sleep_ms(5);
    
    // Send 0011 third time
    lcd_write_4bits(0x30);
    sleep_ms(5);
    
    // Now switch to 4-bit mode
    lcd_write_4bits(0x20);
    sleep_ms(5);
    
    // Function set: 4-bit mode, 2 lines, 5x8 dots
    lcd_command(LCD_FUNCTIONSET | LCD_4BITMODE | LCD_2LINE | LCD_5x8DOTS);
    
    // Display control: display on, cursor off, blink off
    lcd_command(LCD_DISPLAYCONTROL | LCD_DISPLAYON | LCD_CURSOROFF | LCD_BLINKOFF);
    
    // Clear display
    lcd_command(LCD_CLEARDISPLAY);
    sleep_ms(5);  // Clear command needs extra time (>1.52ms)
    
    // Entry mode: increment cursor position, no display shift
    lcd_command(LCD_ENTRYMODESET | LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT);
    
    // Return home
    lcd_command(LCD_RETURNHOME);
    sleep_ms(5);  // Return home needs extra time
    
    // Test with a simple message
    const char* test_msg = "LCD Ready";
    for (int i = 0; test_msg[i] != '\0'; i++) {
        lcd_data(test_msg[i]);
    }
    
    lcd_available = true;
    printf("LCD initialization complete\n");
    return true;
}

void lcd_clear() {
    if (!lcd_available) return;
    lcd_command(LCD_CLEARDISPLAY);
    sleep_ms(5);  // Clear needs extra time
}

void lcd_set_cursor(uint8_t col, uint8_t row) {
    if (!lcd_available) return;
    
    static const uint8_t row_offsets[] = {0x00, 0x40, 0x14, 0x54};
    lcd_command(LCD_SETDDRAMADDR | (col + row_offsets[row % 4]));
}

void lcd_print(const char* text) {
    if (!lcd_available) return;
    
    while (*text) {
        lcd_data(*text++);
    }
}

void add_to_message(char letter) {
    if (message_index >= MAX_MESSAGE_LENGTH - 1) {
        // Shift message to make room for new character
        memmove(display_message, display_message + 1, MAX_MESSAGE_LENGTH - 2);
        message_index = MAX_MESSAGE_LENGTH - 2;
    }
    
    display_message[message_index++] = letter;
    display_message[message_index] = '\0';
    
    update_lcd_display();
}

void update_lcd_display() {
    if (!lcd_available) return;
    
    // Calculate how much of the message to show (last 16 chars)
    int start_pos = 0;
    if (strlen(display_message) > LCD_CHAR_WIDTH) {
        start_pos = strlen(display_message) - LCD_CHAR_WIDTH;
    }
    
    // Display title and message
    lcd_clear();
    lcd_set_cursor(0, 0);
    lcd_print("Morse Receiver");
    
    lcd_set_cursor(0, 1);
    lcd_print(display_message + start_pos);
    
    // Log what we're displaying
    char debug_msg[64];
    sprintf(debug_msg, "LCD Display: %s", display_message);
    uart_log(debug_msg);
}

void init_receiver() {
    stdio_init_all();
    sleep_ms(2000);  // Give time for USB to initialize
    
    printf("\n\n==== Morse Code Receiver with LCD ====\n");
    
    // Initialize UART
    uart_init(UART_ID, BAUD_RATE);
    gpio_set_function(UART_RX_PIN, GPIO_FUNC_UART);
    gpio_set_function(UART_TX_PIN, GPIO_FUNC_UART);
    
    // Initialize onboard LED
    gpio_init(PICO_DEFAULT_LED_PIN);
    gpio_set_dir(PICO_DEFAULT_LED_PIN, GPIO_OUT);
    
    // Initialize the LCD
    if (lcd_init()) {
        printf("LCD successfully initialized\n");
    } else {
        printf("LCD initialization failed - will continue without LCD\n");
    }
    
    printf("Receiver initialized and ready\n");
}

void uart_log(const char* message) {
    printf("%s\n", message);
}

char decode_morse(const char* morse) {
    if (!morse || strlen(morse) == 0) return '?';
    
    for (int i = 0; i < 26; i++) {
        if (strcmp(morse_code[i], morse) == 0) {
            return 'A' + i;
        }
    }
    return '?';
}

void display_letter(char letter) {
    if (letter == ' ') {
        uart_log("Detected: SPACE");
        add_to_message(' ');
        return;
    }
    
    int index = letter - 'A';
    if (index >= 0 && index < 26) {
        char msg[32];
        sprintf(msg, "Decoded: %c", letter);
        uart_log(msg);
        
        // Display on LCD
        add_to_message(letter);
    } else {
        uart_log("Invalid letter detected");
    }
}

int main() {
    init_receiver();
    
    char morse_buffer[MAX_MORSE_LENGTH] = {0};
    int buffer_index = 0;
    uint32_t last_signal_time = 0;
    uint32_t last_event_time = 0;
    bool in_character = false;
    bool in_word = false;
    bool space_added = false;
    
    uart_log("Starting Morse reception...");
    
    // Display welcome message on LCD
    if (lcd_available) {
        lcd_clear();
        lcd_set_cursor(0, 0);
        lcd_print("Morse Receiver");
        lcd_set_cursor(0, 1);
        lcd_print("Waiting...");
    }
    
    while (true) {
        uint32_t current_time = to_ms_since_boot(get_absolute_time());
        
        // Check for incoming data
        if (uart_is_readable(UART_ID)) {
            char c = uart_getc(UART_ID);
            
            // Flash LED when receiving data
            gpio_put(PICO_DEFAULT_LED_PIN, 1);
            sleep_ms(5);
            gpio_put(PICO_DEFAULT_LED_PIN, 0);
            
            if (c == '.' || c == '-') {
                // Start a new character if we're not in one
                if (!in_character) {
                    buffer_index = 0;
                    memset(morse_buffer, 0, sizeof(morse_buffer));
                    in_character = true;
                    space_added = false;
                }
                
                // Add the dot or dash to our buffer
                if (buffer_index < MAX_MORSE_LENGTH - 1) {
                    morse_buffer[buffer_index++] = c;
                    morse_buffer[buffer_index] = '\0';
                    
                    char msg[32];
                    sprintf(msg, "Received signal: %c", c);
                    uart_log(msg);
                }
                
                last_signal_time = current_time;
                last_event_time = current_time;
                in_word = true;
            }
            // Character gap marker directly from transmitter
            else if (strncmp(&c, "C", 1) == 0 || strncmp(&c, "c", 1) == 0) {
                if (strlen(morse_buffer) > 0 && in_character) {
                    // We have a complete character to decode
                    char decoded = decode_morse(morse_buffer);
                    if (decoded != '?') {
                        display_letter(decoded);
                        char msg[64];
                        sprintf(msg, "Decoded character: %c (%s)", decoded, morse_buffer);
                        uart_log(msg);
                    } else {
                        char msg[64];
                        sprintf(msg, "Failed to decode: (%s)", morse_buffer);
                        uart_log(msg);
                    }
                    
                    // Reset the buffer
                    memset(morse_buffer, 0, MAX_MORSE_LENGTH);
                    buffer_index = 0;
                    in_character = false;
                }
                last_event_time = current_time;
            }
            // Word gap marker directly from transmitter
            else if (strncmp(&c, "W", 1) == 0 || strncmp(&c, "w", 1) == 0) {
                if (in_word && !space_added) {
                    display_letter(' ');
                    in_word = false;
                    space_added = true;
                    uart_log("Word gap detected - adding space");
                }
                last_event_time = current_time;
            }
            // Handle entire CHAR GAP or WORD GAP strings
            else if (c == 'H' && in_character) { // Part of "CHAR GAP"
                // Finish current character if we have content
                if (strlen(morse_buffer) > 0) {
                    char decoded = decode_morse(morse_buffer);
                    if (decoded != '?') {
                        display_letter(decoded);
                    }
                    memset(morse_buffer, 0, MAX_MORSE_LENGTH);
                    buffer_index = 0;
                    in_character = false;
                }
            }
            else if (c == 'O' && in_word) { // Part of "WORD GAP"
                if (!space_added) {
                    display_letter(' ');
                    space_added = true;
                }
                in_word = false;
            }
        }
        
        // Automatic timeout detection for incomplete characters
        if (in_character && (current_time - last_signal_time > INTER_CHAR_GAP)) {
            if (strlen(morse_buffer) > 0) {
                char decoded = decode_morse(morse_buffer);
                if (decoded != '?') {
                    display_letter(decoded);
                    char msg[64];
                    sprintf(msg, "Auto-decoded by timeout: %c (%s)", decoded, morse_buffer);
                    uart_log(msg);
                } else {
                    char msg[64];
                    sprintf(msg, "Failed to auto-decode: (%s)", morse_buffer);
                    uart_log(msg);
                }
                
                memset(morse_buffer, 0, MAX_MORSE_LENGTH);
                buffer_index = 0;
            }
            in_character = false;
        }
        
        // Detect word gaps automatically if no events for a while
        if (in_word && !space_added && (current_time - last_event_time > WORD_GAP)) {
            display_letter(' ');
            space_added = true;
            in_word = false;
            uart_log("Auto word gap - adding space");
        }
        
        sleep_ms(5);  // Prevent tight polling
    }
    
    return 0;
}