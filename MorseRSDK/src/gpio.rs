use cortex_m::delay::Delay;
use embedded_hal_0_2::digital::v2::OutputPin;
use rp2040_hal::Timer;
use rtt_target::rprintln;

pub fn benchmark_gpio_toggle<P: rp2040_hal::gpio::PinId, PT: rp2040_hal::gpio::PullType>(
    timer: &Timer,
    pin: &mut rp2040_hal::gpio::Pin<P, rp2040_hal::gpio::FunctionSio<rp2040_hal::gpio::SioOutput>, PT>,
    delay: &mut Delay,
) {
    // Header matches the C version
    rprintln!("task,method,iteration,toggles,total_time_us,avg_toggle_us");

    const NUM_ITERATIONS: usize = 100;
    const TOGGLE_COUNT: usize = 1000; // One toggle = set high + set low

    for i in 0..NUM_ITERATIONS {
        let start = timer.get_counter().ticks();

        for _ in 0..TOGGLE_COUNT {
            // Use the rp2040-hal's direct pin methods for optimal performance
            let _ = pin.set_high();
            let _ = pin.set_low();
        }

        let end = timer.get_counter().ticks();
        let duration_us = end.wrapping_sub(start);

        let avg_us = if TOGGLE_COUNT > 0 {
            duration_us as f32 / TOGGLE_COUNT as f32
        } else {
            0.0
        };

        rprintln!(
            "gpio,toggle,{},{},{},{:.2}",
            i,
            TOGGLE_COUNT,
            duration_us,
            avg_us
        );

        delay.delay_ms(100);
    }
    rprintln!("GPIO benchmark finished.");
}

pub fn benchmark_gpio_toggle_raw(
    timer: &Timer,
    pin_number: u8,  // GPIO pin number
    delay: &mut Delay,
) {
    // Header matches the C version
    rprintln!("task,method,iteration,toggles,total_time_us,avg_toggle_us");

    const NUM_ITERATIONS: usize = 100;
    const TOGGLE_COUNT: usize = 1000; // One toggle = set high + set low

    // Get direct pointers to SIO registers
    let sio = unsafe { &*rp2040_hal::pac::SIO::ptr() };
    
    // Configure the pin as output (equivalent to gpio_init and gpio_set_dir in C)
    let pin_mask = 1u32 << pin_number;
    unsafe {
        // Set GPIO direction to output using the proper method
        sio.gpio_oe_set().write(|w| unsafe { w.bits(pin_mask) });
    }

    for i in 0..NUM_ITERATIONS {
        let start = timer.get_counter().ticks();

        for _ in 0..TOGGLE_COUNT {
            // Direct register manipulation - equivalent to gpio_put in C SDK
            unsafe {
                // Set high (equivalent to gpio_put(pin, 1) in C)
                sio.gpio_out_set().write(|w| unsafe { w.bits(pin_mask) });
                
                // Set low (equivalent to gpio_put(pin, 0) in C)
                sio.gpio_out_clr().write(|w| unsafe { w.bits(pin_mask) });
            }
        }

        let end = timer.get_counter().ticks();
        let duration_us = end.wrapping_sub(start);

        let avg_us = if TOGGLE_COUNT > 0 {
            duration_us as f32 / TOGGLE_COUNT as f32
        } else {
            0.0
        };

        rprintln!(
            "gpio,toggle_raw,{},{},{},{:.2}",
            i,
            TOGGLE_COUNT,
            duration_us,
            avg_us
        );

        delay.delay_ms(100);
    }
    rprintln!("GPIO raw benchmark finished.");
}