# == DO NOT EDIT THE FOLLOWING LINES for the Raspberry Pi Pico VS Code Extension to work ==
if(WIN32)
    set(USERHOME $ENV{USERPROFILE})
else()
    set(USERHOME $ENV{HOME})
endif()
set(sdkVersion 2.1.1)
set(toolchainVersion 14_2_Rel1)
set(picotoolVersion 2.1.1)
set(picoVscode ${USERHOME}/.pico-sdk/cmake/pico-vscode.cmake)
if (EXISTS ${picoVscode})
    include(${picoVscode})
endif()
# ====================================================================================
set(PICO_BOARD pico CACHE STRING "Board type")
cmake_minimum_required(VERSION 3.12)
# Pull in Pico SDK
include(pico_sdk_import.cmake)
project(MorseCSDK C CXX ASM)

# Initialize the Pico SDK
pico_sdk_init()

# Disable all optimizations
add_compile_options(-O0)
add_compile_options(-O0 -fstack-usage)

# Add include directory
include_directories(${CMAKE_SOURCE_DIR}/include)
include_directories(${CMAKE_SOURCE_DIR}/test)

# Define the transmitter executable
add_executable(transmitter
    src/transmitter.c
)

# Define the receiver executable
add_executable(receiver
    src/receiver.c
)

add_executable(benchmarks
    src/benchmarks.c
    test/gpio.c
    test/adc.c
    test/interrupt.c
    test/pwm.c
    test/uart.c
)

# Define benchmark executables
add_executable(benchmark_gpio
    src/benchmarks.c
    test/gpio.c
)

add_executable(benchmark_adc
    src/benchmarks.c
    test/adc.c
)

add_executable(benchmark_interrupt
    src/benchmarks.c
    test/interrupt.c
)

add_executable(benchmark_pwm
    src/benchmarks.c
    test/pwm.c
)

add_executable(benchmark_uart
    src/benchmarks.c
    test/uart.c
)

# Define preprocessor macros for each executable
target_compile_definitions(transmitter PRIVATE TRANSMITTER)
target_compile_definitions(receiver PRIVATE RECEIVER)
target_compile_definitions(benchmarks PRIVATE HARDWARE)
target_compile_definitions(benchmark_gpio PRIVATE BENCHMARK_MODE=1)
target_compile_definitions(benchmark_pwm PRIVATE BENCHMARK_MODE=2)
target_compile_definitions(benchmark_adc PRIVATE BENCHMARK_MODE=3)
target_compile_definitions(benchmark_interrupt PRIVATE BENCHMARK_MODE=4)
target_compile_definitions(benchmark_uart PRIVATE BENCHMARK_MODE=5)

# Enable stdio (USB) for all executables
pico_enable_stdio_usb(transmitter 1)
pico_enable_stdio_usb(receiver 1)
pico_enable_stdio_usb(benchmarks 1)
pico_enable_stdio_usb(benchmark_gpio 1)
pico_enable_stdio_usb(benchmark_adc 1)
pico_enable_stdio_usb(benchmark_interrupt 1)
pico_enable_stdio_usb(benchmark_pwm 1)
pico_enable_stdio_usb(benchmark_uart 1)

# Disable UART stdio for all USB executables
pico_enable_stdio_uart(transmitter 0)
pico_enable_stdio_uart(receiver 0)
pico_enable_stdio_uart(benchmarks 0)
pico_enable_stdio_uart(benchmark_gpio 0)
pico_enable_stdio_uart(benchmark_adc 0)
pico_enable_stdio_uart(benchmark_interrupt 0)
pico_enable_stdio_uart(benchmark_pwm 0)
pico_enable_stdio_uart(benchmark_uart 0)

# Link necessary libraries
target_link_libraries(transmitter pico_stdlib hardware_gpio hardware_adc hardware_uart hardware_pwm hardware_irq hardware_i2c hardware_timer)
target_link_libraries(receiver pico_stdlib hardware_gpio hardware_adc hardware_uart hardware_pwm hardware_irq hardware_i2c hardware_timer)
target_link_libraries(benchmarks pico_stdlib hardware_gpio hardware_adc hardware_uart hardware_pwm hardware_irq hardware_i2c)
target_link_libraries(benchmark_gpio pico_stdlib hardware_gpio hardware_i2c hardware_adc)
target_link_libraries(benchmark_adc pico_stdlib hardware_adc hardware_i2c)
target_link_libraries(benchmark_interrupt pico_stdlib hardware_gpio hardware_irq hardware_i2c hardware_adc)
target_link_libraries(benchmark_pwm pico_stdlib hardware_pwm hardware_i2c hardware_adc)
target_link_libraries(benchmark_uart pico_stdlib hardware_uart hardware_i2c hardware_adc)

# Create .uf2 files for all executables
pico_add_extra_outputs(transmitter)
pico_add_extra_outputs(receiver)
pico_add_extra_outputs(benchmarks)
pico_add_extra_outputs(benchmark_gpio)
pico_add_extra_outputs(benchmark_adc)
pico_add_extra_outputs(benchmark_interrupt)
pico_add_extra_outputs(benchmark_pwm)
pico_add_extra_outputs(benchmark_uart)