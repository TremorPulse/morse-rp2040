#ifndef BENCHMARKS_H
#define BENCHMARKS_H

// GPIO digital output performance test
void measure_gpio_performance(void);

// PWM configuration timing test
void measure_pwm_setup_time(void);

// ADC sampling performance evaluation
void measure_adc_performance(void);

// Interrupt response latency measurement
void measure_interrupt_latency(void);

// UART transmission throughput test
void measure_uart_throughput(void);

#endif // BENCHMARKS_H