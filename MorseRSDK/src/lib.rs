#![no_std]

use embedded_time::duration::Milliseconds;

pub mod adc;
pub mod gpio;
pub mod interrupt;
pub mod pwm;
pub mod uart;

// Constants
pub const UART_ID: u8 = 0;
pub const BAUD_RATE: u32 = 115200;
pub const UART_TX_PIN: u8 = 0;
pub const UART_RX_PIN: u8 = 1;

pub const DOT_DURATION: Milliseconds<u32> = Milliseconds(250);
pub const DASH_DURATION: Milliseconds<u32> = Milliseconds(750);
pub const INTRA_CHAR_GAP: Milliseconds<u32> = Milliseconds(250);
pub const INTER_CHAR_GAP: Milliseconds<u32> = Milliseconds(750);
pub const WORD_GAP: Milliseconds<u32> = Milliseconds(1750);
pub const MIN_SIGNAL_GAP: Milliseconds<u32> = Milliseconds(150);
pub const MAX_CHAR_TIME: Milliseconds<u32> = Milliseconds(3000);

pub const BUTTON_PIN: u8 = 16;
pub const LED_PIN: u8 = 15;
pub const SPEAKER_PIN: u8 = 21;
pub const ADC_PIN: u8 = 26;

pub const DOT_THRESHOLD_MS: u32 = 250;
pub const DASH_THRESHOLD_MS: u32 = 750;
pub const DEBOUNCE_TIME_MS: u64 = 50; 
pub const RELEASE_DEBOUNCE_MS: u32 = 100;
pub const ADC_NOISE_THRESHOLD: u16 = 100;
pub const TONE_DETECTION_THRESHOLD: u16 = 500;

pub const DOT_FREQ: u32 = 800;
pub const DASH_FREQ: u32 = 400;
pub const SYNC_PATTERN: &str = "...---...";
pub const MAX_MORSE_LENGTH: usize = 32;
pub const MAX_MESSAGE_LENGTH: usize = 128;
pub const FREQ_SAMPLE_WINDOW: u32 = 50;

// I2C LCD configuration
pub const SDA_PIN: u8 = 4;
pub const SCL_PIN: u8 = 5;
pub const LCD_ADDRESS: u8 = 0x27;
pub const LCD_CHAR_WIDTH: usize = 16;
pub const LCD_NUM_LINES: usize = 2;
// LCD Commands
pub const LCD_CLEARDISPLAY: u8 = 0x01;
pub const LCD_RETURNHOME: u8 = 0x02;
pub const LCD_ENTRYMODESET: u8 = 0x04;
pub const LCD_DISPLAYCONTROL: u8 = 0x08;
pub const LCD_CURSORSHIFT: u8 = 0x10;
pub const LCD_FUNCTIONSET: u8 = 0x20;
pub const LCD_SETCGRAMADDR: u8 = 0x40;
pub const LCD_SETDDRAMADDR: u8 = 0x80;

// LCD flags for display entry mode
pub const LCD_ENTRYRIGHT: u8 = 0x00;
pub const LCD_ENTRYLEFT: u8 = 0x02;
pub const LCD_ENTRYSHIFTINCREMENT: u8 = 0x01;
pub const LCD_ENTRYSHIFTDECREMENT: u8 = 0x00;

// LCD flags for display control
pub const LCD_DISPLAYON: u8 = 0x04;
pub const LCD_DISPLAYOFF: u8 = 0x00;
pub const LCD_CURSORON: u8 = 0x02;
pub const LCD_CURSOROFF: u8 = 0x00;
pub const LCD_BLINKON: u8 = 0x01;
pub const LCD_BLINKOFF: u8 = 0x00;

// LCD flags for function set
pub const LCD_8BITMODE: u8 = 0x10;
pub const LCD_4BITMODE: u8 = 0x00;
pub const LCD_2LINE: u8 = 0x08;
pub const LCD_1LINE: u8 = 0x00;
pub const LCD_5X10_DOTS: u8 = 0x04;
pub const LCD_5X8_DOTS: u8 = 0x00;

// PCF8574 bit assignments
pub const LCD_RS_BIT: u8 = 0x01;
pub const LCD_RW_BIT: u8 = 0x02;
pub const LCD_EN_BIT: u8 = 0x04;
pub const LCD_BACKLIGHT: u8 = 0x08;
pub const LCD_DATA_BITS: u8 = 0xF0;

pub const MORSE_CODE: [&str; 26] = [
    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", 
    ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", 
    "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."
];