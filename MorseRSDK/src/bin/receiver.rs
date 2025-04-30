#![no_std]
#![no_main]

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

use panic_halt as _;
use core::fmt::Write;
use core::result::Result::{Ok, Err};
use core::iter::Iterator;
use core::convert::Into;
use core::write;

use rp2040_hal::{
    gpio::{bank0::Gpio25, Pin, FunctionSio, SioOutput, PullDown, FunctionI2c},
    pac,
    timer::Timer,
    clocks::{Clock, init_clocks_and_plls},
    watchdog::Watchdog,
    Sio,
    gpio::Pins,
    uart::{UartPeripheral, UartConfig, DataBits, Parity, StopBits},
    i2c::I2C,
};
use rp_pico::XOSC_CRYSTAL_FREQ;
use rp2040_hal::entry;
use cortex_m::delay::Delay;
use heapless::String;
use nb::block;
use embedded_hal::digital::OutputPin;
use embedded_hal_0_2::serial;
use embedded_hal_0_2::blocking::i2c;
use rp2040_hal::fugit::RateExtU32;

use morse_rsdk::{
    BAUD_RATE,
    MORSE_CODE, LCD_ADDRESS, LCD_BACKLIGHT, LCD_EN_BIT, LCD_RS_BIT, LCD_CLEARDISPLAY,
    LCD_RETURNHOME, LCD_ENTRYMODESET, LCD_DISPLAYCONTROL, LCD_FUNCTIONSET,
    LCD_SETDDRAMADDR, LCD_ENTRYLEFT, LCD_ENTRYSHIFTDECREMENT,
    LCD_DISPLAYON, LCD_CURSOROFF, LCD_BLINKOFF, LCD_4BITMODE, LCD_2LINE, LCD_5X8_DOTS,
    LCD_CHAR_WIDTH, MAX_MESSAGE_LENGTH, MAX_MORSE_LENGTH
};

// Timing constants
const INTER_CHAR_GAP: u32 = 1000; // 1 second for inter-character gap
const WORD_GAP: u32 = 2000; // 2 seconds for word gap

pub struct Receiver<UART, I2C> 
where
    UART: serial::Write<u8> + serial::Read<u8>,
    I2C: i2c::Write,
{
    uart: UART,
    i2c: I2C,
    led_pin: Pin<Gpio25, FunctionSio<SioOutput>, PullDown>,
    timer: Timer,
    delay: Delay,
    display_message: String<{MAX_MESSAGE_LENGTH}>,
    message_index: usize,
    actual_lcd_address: u8,
    lcd_available: bool,
}

impl<UART, I2C> Receiver<UART, I2C>
where
    UART: serial::Write<u8> + serial::Read<u8>,
    I2C: i2c::Write,
{
    pub fn new(
        uart: UART,
        i2c: I2C,
        led_pin: Pin<Gpio25, FunctionSio<SioOutput>, PullDown>,
        timer: Timer,
        delay: Delay,
    ) -> Self {
        Self {
            uart,
            i2c,
            led_pin,
            timer,
            delay,
            display_message: String::new(),
            message_index: 0,
            actual_lcd_address: LCD_ADDRESS,
            lcd_available: false,
        }
    }

    pub fn init(&mut self) {
        // Test LED first
        self.led_pin.set_high().unwrap();
        self.delay.delay_ms(500);
        self.led_pin.set_low().unwrap();
        
        // Test UART
        self.uart_log("UART test - if you see this, UART works");
        
        // Test I2C LCD
        if self.lcd_init() {
            self.uart_log("LCD initialized successfully");
            self.lcd_command(LCD_CLEARDISPLAY);
            self.lcd_print("Hello World!");
        } else {
            self.uart_log("LCD init failed");
        }
        
        self.uart_log("System ready");
    }

    fn lcd_write_byte(&mut self, byte_value: u8) -> bool {
        self.i2c.write(self.actual_lcd_address, &[byte_value]).is_ok()
    }

    fn lcd_pulse_enable(&mut self, data: u8) {
        self.lcd_write_byte(data | LCD_EN_BIT | LCD_BACKLIGHT);
        self.delay.delay_us(500);
        self.lcd_write_byte((data & !LCD_EN_BIT) | LCD_BACKLIGHT);
        self.delay.delay_us(100);
    }

    fn lcd_write_4bits(&mut self, value: u8) {
        let data = (value & 0xF0) | LCD_BACKLIGHT;
        self.lcd_pulse_enable(data);
    }

    fn lcd_command(&mut self, command: u8) {
        let high_nibble = (command & 0xF0) | LCD_BACKLIGHT;
        self.lcd_pulse_enable(high_nibble);
        
        let low_nibble = ((command << 4) & 0xF0) | LCD_BACKLIGHT;
        self.lcd_pulse_enable(low_nibble);
        
        self.delay.delay_ms(2);
    }

    fn lcd_data(&mut self, data: u8) {
        let high_nibble = (data & 0xF0) | LCD_RS_BIT | LCD_BACKLIGHT;
        self.lcd_pulse_enable(high_nibble);
        
        let low_nibble = ((data << 4) & 0xF0) | LCD_RS_BIT | LCD_BACKLIGHT;
        self.lcd_pulse_enable(low_nibble);
        
        self.delay.delay_us(100);
    }

    fn lcd_init(&mut self) -> bool {
        self.uart_log("Initializing I2C for LCD...");
        
        self.delay.delay_ms(50);
        self.lcd_write_byte(LCD_BACKLIGHT);
        self.delay.delay_ms(50);
        
        self.lcd_write_4bits(0x30);
        self.delay.delay_ms(5);
        self.lcd_write_4bits(0x30);
        self.delay.delay_ms(5);
        self.lcd_write_4bits(0x30);
        self.delay.delay_ms(5);
        
        self.lcd_write_4bits(0x20);
        self.delay.delay_ms(5);
        
        self.lcd_command(LCD_FUNCTIONSET | LCD_4BITMODE | LCD_2LINE | LCD_5X8_DOTS);
        self.lcd_command(LCD_DISPLAYCONTROL | LCD_DISPLAYON | LCD_CURSOROFF | LCD_BLINKOFF);
        self.lcd_command(LCD_CLEARDISPLAY);
        self.delay.delay_ms(5);
        self.lcd_command(LCD_ENTRYMODESET | LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT);
        self.lcd_command(LCD_RETURNHOME);
        self.delay.delay_ms(5);
        
        let test_msg = "LCD Ready";
        for c in test_msg.chars() {
            self.lcd_data(c as u8);
        }
        
        self.lcd_available = true;
        self.uart_log("LCD initialization complete");
        true
    }

    fn lcd_clear(&mut self) {
        if !self.lcd_available { return; }
        self.lcd_command(LCD_CLEARDISPLAY);
        self.delay.delay_ms(5);
    }

    fn lcd_set_cursor(&mut self, col: u8, row: u8) {
        if !self.lcd_available { return; }
        
        let row_offsets = [0x00, 0x40, 0x14, 0x54];
        self.lcd_command(LCD_SETDDRAMADDR | (col + row_offsets[row as usize % 4]));
    }

    fn lcd_print(&mut self, text: &str) {
        if !self.lcd_available { return; }
        
        for c in text.chars() {
            self.lcd_data(c as u8);
        }
    }

    fn add_to_message(&mut self, letter: char) {
        if self.message_index >= MAX_MESSAGE_LENGTH - 1 {
            let mut temp = String::<{MAX_MESSAGE_LENGTH}>::new();
            let _ = write!(&mut temp, "{}", &self.display_message[1..]);
            self.display_message.clear();
            let _ = self.display_message.push_str(temp.as_str());
            self.message_index = MAX_MESSAGE_LENGTH - 2;
        }
        
        let _ = self.display_message.push(letter);
        self.message_index += 1;
        self.update_lcd_display();
    }

    fn update_lcd_display(&mut self) {
        if !self.lcd_available { return; }
        
        let start_pos = if self.display_message.len() > LCD_CHAR_WIDTH {
            self.display_message.len() - LCD_CHAR_WIDTH
        } else {
            0
        };
        
        let display_text = {
            let mut temp = String::<{LCD_CHAR_WIDTH}>::new();
            let _ = write!(&mut temp, "{}", &self.display_message[start_pos..]);
            temp
        };
        
        self.lcd_clear();
        self.lcd_set_cursor(0, 0);
        self.lcd_print("Morse Receiver");
        
        self.lcd_set_cursor(0, 1);
        self.lcd_print(display_text.as_str());
        
        let mut log_msg = String::<128>::new();
        let _ = write!(log_msg, "LCD Display: {}", display_text);
        self.uart_log(log_msg.as_str());
    }

    pub fn uart_log(&mut self, message: &str) {
        for byte in message.as_bytes() {
            let _ = block!(self.uart.write(*byte));
        }
        let _ = block!(self.uart.write(b'\r'));
        let _ = block!(self.uart.write(b'\n'));
    }

    fn decode_morse(&self, morse: &str) -> char {
        if morse.is_empty() { return '?'; }
        
        for (i, code) in MORSE_CODE.iter().enumerate() {
            if *code == morse {
                return (b'A' + i as u8) as char;
            }
        }
        '?'
    }

    fn display_letter(&mut self, letter: char) {
        if letter == ' ' {
            self.uart_log("Detected: SPACE");
            self.add_to_message(' ');
            return;
        }
        
        if letter.is_ascii_uppercase() {
            let mut message = String::<32>::new();
            let _ = write!(&mut message, "Decoded: {}", letter);
            self.uart_log(message.as_str());
            self.add_to_message(letter);
        } else {
            self.uart_log("Invalid letter detected");
        }
    }

    pub fn run(&mut self) {
        let mut morse_buffer = [0u8; MAX_MORSE_LENGTH];
        let mut buffer_index = 0;
        let mut last_signal_time = 0u64;
        let mut last_event_time = 0u64;
        let mut in_character = false;
        let mut in_word = false;
        let mut space_added = false;
        
        self.uart_log("Starting Morse reception...");
        
        if self.lcd_available {
            self.lcd_clear();
            self.lcd_set_cursor(0, 0);
            self.lcd_print("Morse Receiver");
            self.lcd_set_cursor(0, 1);
            self.lcd_print("Waiting...");
        }
        
        loop {
            let current_time = self.timer.get_counter().ticks();
            
            if let Ok(c) = block!(self.uart.read()) {
                self.led_pin.set_high().unwrap();
                self.delay.delay_ms(5);
                self.led_pin.set_low().unwrap();
                
                if c == b'.' || c == b'-' {
                    if !in_character {
                        buffer_index = 0;
                        morse_buffer = [0u8; MAX_MORSE_LENGTH];
                        in_character = true;
                        space_added = false;
                    }
                    
                    if buffer_index < MAX_MORSE_LENGTH - 1 {
                        morse_buffer[buffer_index] = c;
                        buffer_index += 1;
                        
                        let mut message = String::<32>::new();
                        let _ = write!(&mut message, "Received signal: {}", c as char);
                        self.uart_log(message.as_str());
                    }
                    
                    last_signal_time = current_time;
                    last_event_time = current_time;
                    in_word = true;
                }
                else if c == b'C' || c == b'c' {
                    if buffer_index > 0 && in_character {
                        let morse_str = core::str::from_utf8(&morse_buffer[..buffer_index]).unwrap_or("");
                        let decoded = self.decode_morse(morse_str);
                        if decoded != '?' {
                            self.display_letter(decoded);
                            let mut message = String::<64>::new();
                            let _ = write!(&mut message, "Decoded character: {} ({})", decoded, morse_str);
                            self.uart_log(message.as_str());
                        } else {
                            let mut message = String::<64>::new();
                            let _ = write!(&mut message, "Failed to decode: ({})", morse_str);
                            self.uart_log(message.as_str());
                        }
                        
                        morse_buffer = [0u8; MAX_MORSE_LENGTH];
                        buffer_index = 0;
                        in_character = false;
                    }
                    last_event_time = current_time;
                }
                else if c == b'W' || c == b'w' {
                    if in_word && !space_added {
                        self.display_letter(' ');
                        in_word = false;
                        space_added = true;
                        self.uart_log("Word gap detected - adding space");
                    }
                    last_event_time = current_time;
                }
                else if c == b'H' && in_character {
                    if buffer_index > 0 {
                        let morse_str = core::str::from_utf8(&morse_buffer[..buffer_index]).unwrap_or("");
                        let decoded = self.decode_morse(morse_str);
                        if decoded != '?' {
                            self.display_letter(decoded);
                        }
                        morse_buffer = [0u8; MAX_MORSE_LENGTH];
                        buffer_index = 0;
                        in_character = false;
                    }
                }
                else if c == b'O' && in_word {
                    if !space_added {
                        self.display_letter(' ');
                        space_added = true;
                    }
                    in_word = false;
                }
            }
            
            if in_character && (current_time - last_signal_time > INTER_CHAR_GAP.into()) {
                if buffer_index > 0 {
                    let morse_str = core::str::from_utf8(&morse_buffer[..buffer_index]).unwrap_or("");
                    let decoded = self.decode_morse(morse_str);
                    if decoded != '?' {
                        self.display_letter(decoded);
                        let mut message = String::<64>::new();
                        let _ = write!(&mut message, "Auto-decoded by timeout: {} ({})", decoded, morse_str);
                        self.uart_log(message.as_str());
                    } else {
                        let mut message = String::<64>::new();
                        let _ = write!(&mut message, "Failed to auto-decode: ({})", morse_str);
                        self.uart_log(message.as_str());
                    }
                    
                    morse_buffer = [0u8; MAX_MORSE_LENGTH];
                    buffer_index = 0;
                }
                in_character = false;
            }
            
            if in_word && !space_added && (current_time - last_event_time > WORD_GAP.into()) {
                self.display_letter(' ');
                space_added = true;
                in_word = false;
                self.uart_log("Auto word gap - adding space");
            }
            
            self.delay.delay_ms(5);
        }
    }
}

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();
    
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    
    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    
    let uart = UartPeripheral::new(
        pac.UART0,
        (pins.gpio0.into_function(), pins.gpio1.into_function()),
        &mut pac.RESETS,
    )
    .enable(
        UartConfig::new(BAUD_RATE.Hz(), DataBits::Eight, core::prelude::v1::Some(Parity::Odd), StopBits::One),
        clocks.peripheral_clock.freq(),
    )
    .unwrap();
    
    let i2c = I2C::i2c0(
        pac.I2C0,
        pins.gpio4.into_pull_up_input().into_function::<FunctionI2c>(),
        pins.gpio5.into_pull_up_input().into_function::<FunctionI2c>(),
        100_000.Hz(),
        &mut pac.RESETS,
        clocks.system_clock.freq(),
    );
    
    let led_pin = pins.gpio25.into_push_pull_output();
    
    let mut receiver = Receiver::new(
        uart,
        i2c,
        led_pin,
        timer,
        delay,
    );
    
    receiver.init();
    receiver.run();
    
    loop {}
}