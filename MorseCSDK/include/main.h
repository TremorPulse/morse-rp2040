#ifndef MAIN_H
#define MAIN_H
#include "pico/stdlib.h"
#include "hardware/gpio.h"
#include "hardware/adc.h"
#include "hardware/uart.h"
#include "hardware/i2c.h"

#define UART_ID uart0
#define BAUD_RATE 115200
#define UART_TX_PIN 0
#define UART_RX_PIN 1
#define DOT_DURATION 250
#define DASH_DURATION 750
#define INTRA_CHAR_GAP 250    // Gap between dots/dashes within character (1 unit)
#define INTER_CHAR_GAP 750    // Gap between characters (3 units)
#define WORD_GAP 1750         // Gap between words (7 units)
#define MIN_SIGNAL_GAP 150    // Minimum time between valid signals
#define MAX_CHAR_TIME 3000    // Maximum time to build a character
#define BUTTON_PIN 16
#define LED_PIN 15
#define SPEAKER_PIN 21
#define ADC_PIN 26
#define DOT_THRESHOLD_MS 250
#define DASH_THRESHOLD_MS 750
#define DEBOUNCE_TIME_MS 50
#define RELEASE_DEBOUNCE_MS 100
#define ADC_NOISE_THRESHOLD 100
#define TONE_DETECTION_THRESHOLD 500
#define DOT_FREQ 800
#define DASH_FREQ 400
#define SYNC_PATTERN "...---..."  // SOS pattern for sync
#define MAX_MORSE_LENGTH 32
#define MAX_MESSAGE_LENGTH 128
#define FREQ_SAMPLE_WINDOW 50  // ms to sample frequency
#define MAX_SAMPLES 100

// New I2C LCD definitions
#define SDA_PIN 4  // I2C data pin
#define SCL_PIN 5  // I2C clock pin

// I2C LCD Constants
#define LCD_ADDRESS 0x27            // Default PCF8574 address, will scan for actual
#define LCD_CHAR_WIDTH 16           // Characters per line
#define LCD_NUM_LINES 2             // Number of display lines

// PCF8574 bit assignments for LCD control
#define LCD_RS_BIT      0x01        // Register select bit
#define LCD_RW_BIT      0x02        // Read/Write bit
#define LCD_EN_BIT      0x04        // Enable bit
#define LCD_BACKLIGHT   0x08        // Backlight control bit
#define LCD_DATA_BITS   0xF0        // Data bits (4-7)

// LCD Commands
#define LCD_CLEARDISPLAY   0x01
#define LCD_RETURNHOME     0x02
#define LCD_ENTRYMODESET   0x04
#define LCD_DISPLAYCONTROL 0x08
#define LCD_CURSORSHIFT    0x10
#define LCD_FUNCTIONSET    0x20
#define LCD_SETCGRAMADDR   0x40
#define LCD_SETDDRAMADDR   0x80

// LCD Entry mode
#define LCD_ENTRYRIGHT          0x00
#define LCD_ENTRYLEFT           0x02
#define LCD_ENTRYSHIFTINCREMENT 0x01
#define LCD_ENTRYSHIFTDECREMENT 0x00

// LCD Display control
#define LCD_DISPLAYON  0x04
#define LCD_DISPLAYOFF 0x00
#define LCD_CURSORON   0x02
#define LCD_CURSOROFF  0x00
#define LCD_BLINKON    0x01
#define LCD_BLINKOFF   0x00

// LCD Function set
#define LCD_8BITMODE   0x10
#define LCD_4BITMODE   0x00
#define LCD_2LINE      0x08
#define LCD_1LINE      0x00
#define LCD_5x10DOTS   0x04
#define LCD_5x8DOTS    0x00

void init_transmitter(void);
void init_receiver(void);
void transmit_dot(void);
void transmit_dash(void);
void transmit_gap(uint32_t duration);
void transmit_sync(void);
void signal_tone(bool is_dot);
char decode_morse(const char* morse);
void uart_log(const char* message);  
void display_letter(char letter);
bool detect_sync(void);
void handle_spaces(uint32_t gap_duration);

// LCD functions
bool lcd_init(void);
void lcd_clear(void);
void lcd_home(void);
void lcd_set_cursor(uint8_t col, uint8_t row);
void lcd_print(const char* text);
void update_lcd_display(void);
void add_to_message(char letter);

// Morse code array for letters A-Z
extern const char* morse_code[];    
#endif