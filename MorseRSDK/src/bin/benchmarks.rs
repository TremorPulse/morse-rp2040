#![no_std]
#![no_main]

use rp2040_hal::entry;
use cortex_m::delay::Delay;
use nb::block;
use rp2040_hal::{
    Adc,
    Sio,
    Watchdog,
    clocks::init_clocks_and_plls,
    gpio::{
        bank0,
        Pin,
        FunctionSio,
        SioOutput,
        SioInput,
        PullNone,
        PullUp,
    },
    pwm::{
        Slice,
        SliceId,
        FreeRunning,
        PwmPin,
    },
    uart::{
        UartPeripheral,
        Enabled,
        ValidUartPinout,
    },
    Clock,
    adc::AdcPin,
};
use rp_pico::XOSC_CRYSTAL_FREQ;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::adc::OneShot;

// Define constants that were missing
const NUM_ITERATIONS: u32 = 10;
const TOGGLE_COUNT: u32 = 1000;
const NUM_READS: u32 = 100;

macro_rules! print_str {
    ($uart:expr, $str:expr) => {
        for byte in $str.as_bytes() {
            let _ = block!($uart.write(*byte));
        }
        let _ = block!($uart.write(b'\r'));
        let _ = block!($uart.write(b'\n'));
    };
}

macro_rules! print_fmt {
    ($uart:expr, $fmt:expr, $($arg:tt)*) => {{
        let mut s: heapless::String<128> = heapless::String::new();
        core::fmt::write(&mut s, format_args!($fmt, $($arg)*)).unwrap();
        print_str!($uart, &s);
    }};
}

pub fn benchmark_gpio_toggle<U>(
    mut pin: Pin<bank0::Gpio2, FunctionSio<SioOutput>, PullNone>,
    delay: &mut Delay,
    uart: &mut U,
)
where
    U: embedded_hal::serial::Write<u8>,
{
    print_str!(uart, "task,method,iteration,toggles,total_time_us,avg_toggle_us");
    
    for iteration in 0..NUM_ITERATIONS {
        let start = cortex_m::peripheral::SYST::get_current();
        
        for _ in 0..TOGGLE_COUNT {
            pin.set_high().unwrap();
            pin.set_low().unwrap();
        }
        
        let end = cortex_m::peripheral::SYST::get_current();
        let duration = end.wrapping_sub(start) / 72;
        let avg = duration as f32 / TOGGLE_COUNT as f32;
        
        print_fmt!(uart, "gpio,toggle,{},{},{},{}", iteration, TOGGLE_COUNT, duration, avg);
        
        delay.delay_ms(100);
    }
}

pub fn benchmark_pwm<U, S>(
    mut pwm_slice: Slice<S, FreeRunning>, 
    delay: &mut Delay,
    uart: &mut U,
)
where
    U: embedded_hal::serial::Write<u8>,
    S: SliceId,
{
    print_str!(uart, "task,method,iteration,setup_time_us");
    
    for iteration in 0..NUM_ITERATIONS {
        let start = cortex_m::peripheral::SYST::get_current();
        
        let _config = pwm_slice.default_config();
        pwm_slice.set_div_int(100);
        pwm_slice.set_top(65535);
        pwm_slice.channel_a.set_duty(32768);
        pwm_slice.enable();
        
        let end = cortex_m::peripheral::SYST::get_current();
        let duration = end.wrapping_sub(start) / 72;
        
        print_fmt!(uart, "pwm,setup,{},{}", iteration, duration);
        
        delay.delay_ms(100);
        pwm_slice.disable();
        delay.delay_ms(100);
    }
}

pub fn benchmark_adc<U>(
    adc_pin: Pin<bank0::Gpio26, FunctionSio<SioInput>, PullNone>,
    resets: &mut pac::RESETS,
    delay: &mut Delay,
    uart: &mut U,
)
where
    U: embedded_hal::serial::Write<u8>,
{
    print_str!(uart, "task,method,iteration,reads,avg_time_us");
    
    let adc_peripheral = pac::Peripherals::take().unwrap().ADC;
    let mut adc = Adc::new(adc_peripheral, resets);
    let mut adc_pin = AdcPin::new(adc_pin.into_floating_input());
    
    for iteration in 0..NUM_ITERATIONS {
        let mut total_time = 0;
        
        for _ in 0..NUM_READS {
            let start = cortex_m::peripheral::SYST::get_current();
            let _value: u16 = adc.read(&mut adc_pin).unwrap();
            let end = cortex_m::peripheral::SYST::get_current();
            total_time += end.wrapping_sub(start);
        }
        
        let avg_time = (total_time / NUM_READS as u32) / 72;
        print_fmt!(uart, "adc,single_read,{},{},{}", iteration, NUM_READS, avg_time);
        
        delay.delay_ms(100);
    }
}

pub fn benchmark_interrupt<U>(
    mut button_pin: Pin<bank0::Gpio16, FunctionSio<SioInput>, PullUp>,
    mut led_pin: Pin<bank0::Gpio25, FunctionSio<SioOutput>, PullNone>,
    delay: &mut Delay,
    uart: &mut U,
)
where
    U: embedded_hal::serial::Write<u8>,
{
    print_str!(uart, "Benchmark: Interrupt Latency");
    print_str!(uart, "task,method,latency_us");
    print_str!(uart, "Press the button to trigger interrupt...");
    
    button_pin.set_interrupt_enabled(rp2040_hal::gpio::Interrupt::EdgeLow, true);
    
    loop {
        if button_pin.interrupt_status(rp2040_hal::gpio::Interrupt::EdgeLow) {
            button_pin.clear_interrupt(rp2040_hal::gpio::Interrupt::EdgeLow);
            let latency = cortex_m::peripheral::SYST::get_current() / 72;
            print_fmt!(uart, "interrupt,triggered,{}", latency);
            
            led_pin.set_high().unwrap();
            delay.delay_ms(100);
            led_pin.set_low().unwrap();
        }
        
        delay.delay_ms(10);
    }
}

pub fn benchmark_uart<P, U>(
    test_uart: UartPeripheral<Enabled, pac::UART0, P>,
    delay: &mut Delay,
    uart: &mut U,
)
where
    P: ValidUartPinout<pac::UART0>,
    U: embedded_hal::serial::Write<u8>,
{
    print_str!(uart, "task,method,iteration,bytes,time_us,bytes_per_sec");
    
    let test_sizes = [10, 50, 100, 250];
    let mut test_message = [b'A'; 256];
    
    for &msg_size in &test_sizes {
        test_message[msg_size - 1] = b'\0';
        
        for iteration in 0..NUM_ITERATIONS {
            let start = cortex_m::peripheral::SYST::get_current();
            test_uart.write_full_blocking(&test_message[..msg_size]);
            while !test_uart.uart_is_writable() {}
            
            let end = cortex_m::peripheral::SYST::get_current();
            
            let duration = end.wrapping_sub(start) / 72;
            let bytes_per_sec = (msg_size as f32 * 1_000_000.0) / duration as f32;
            
            print_fmt!(uart, "uart,tx,{},{},{},{}", iteration, msg_size, duration, bytes_per_sec);
            
            delay.delay_ms(50);
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
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    
    // Initialize a UART for printing messages
    use rp2040_hal::uart::{UartConfig, DataBits, StopBits};
    
    let uart_pins = (
        pins.gpio0.into_function::<rp2040_hal::gpio::FunctionUart>(),
        pins.gpio1.into_function::<rp2040_hal::gpio::FunctionUart>(),
    );
    
    let mut uart = UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(115200u32.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();
    
    let _led_pin = pins.led.into_push_pull_output();

    #[cfg(feature = "benchmark_gpio")]
    {
        let toggle_pin = pins.gpio2.into_push_pull_output();
        benchmark_gpio_toggle(toggle_pin, &mut delay, &mut uart);
    }

    #[cfg(feature = "benchmark_pwm")]
    {
        use rp2040_hal::gpio::FunctionPwm;
        let pwm_pin = pins.gpio15.into_function::<FunctionPwm>();
        let pwm_slice = rp2040_hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS).pwm7;
        benchmark_pwm(pwm_slice, &mut delay, &mut uart);
    }

    #[cfg(feature = "benchmark_adc")]
    {
        let adc_pin = pins.gpio26.into_floating_input();
        benchmark_adc(adc_pin, &mut pac.RESETS, &mut delay, &mut uart);
    }

    #[cfg(feature = "benchmark_interrupt")]
    {
        let button_pin = pins.gpio16.into_pull_up_input();
        let led_pin = pins.led.into_push_pull_output();
        benchmark_interrupt(button_pin, led_pin, &mut delay, &mut uart);
    }

    #[cfg(feature = "benchmark_uart")]
    {
        use rp2040_hal::uart::{UartPeripheral, UartConfig, DataBits, StopBits};
        let uart_pins = (
            pins.gpio4.into_function::<rp2040_hal::gpio::FunctionUart>(),
            pins.gpio5.into_function::<rp2040_hal::gpio::FunctionUart>(),
        );
        let test_uart = UartPeripheral::new(pac.UART1, uart_pins, &mut pac.RESETS)
            .enable(
                UartConfig::new(115200u32.Hz(), DataBits::Eight, None, StopBits::One),
                clocks.peripheral_clock.freq(),
            )
            .unwrap();
        benchmark_uart(test_uart, &mut delay, &mut uart);
    }

    // Add a panic handler
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        loop {}
    }

    loop {}
}