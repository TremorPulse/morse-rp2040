#ifndef BENCHMARKS_H
#define BENCHMARKS_H

// GPIO benchmark: Measure GPIO toggle performance
void benchmark_gpio_toggle(void);

// PWM benchmark: Measure PWM setup time
void benchmark_pwm(void);

// ADC benchmark: Measure ADC read performance
void benchmark_adc(void);

// Interrupt benchmark: Measure interrupt latency
void benchmark_interrupt(void);

// UART benchmark: Measure UART transmission performance
void benchmark_uart(void);

#endif // BENCHMARKS_H