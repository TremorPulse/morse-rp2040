# CMAKE generated file: DO NOT EDIT!
# Generated by "MinGW Makefiles" Generator, CMake Version 3.31

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

SHELL = cmd.exe

# The CMake executable.
CMAKE_COMMAND = C:\Users\gokua\.pico-sdk\cmake\v3.31.5\bin\cmake.exe

# The command to remove a file.
RM = C:\Users\gokua\.pico-sdk\cmake\v3.31.5\bin\cmake.exe -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = "C:\Users\gokua\OneDrive\Documents\Programming\C and C++\MorseCSDK"

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = "C:\Users\gokua\OneDrive\Documents\Programming\C and C++\MorseCSDK"

# Include any dependencies generated for this target.
include pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/compiler_depend.make

# Include the progress variables for this target.
include pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/progress.make

# Include the compile flags for this target's objects.
include pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/flags.make

pico-sdk/src/rp2040/boot_stage2/bs2_default_padded_checksummed.S: pico-sdk/src/rp2040/boot_stage2/bs2_default.bin
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --blue --bold --progress-dir="C:\Users\gokua\OneDrive\Documents\Programming\C and C++\MorseCSDK\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_1) "Generating bs2_default_padded_checksummed.S"
	cd /d C:\Users\gokua\OneDrive\DOCUME~1\PROGRA~1\CANDC_~1\MORSEC~1\pico-sdk\src\rp2040\BOOT_S~1 && C:\Users\gokua\AppData\Local\Programs\Python\Python312\python.exe C:/Users/gokua/.pico-sdk/sdk/2.1.1/src/rp2040/boot_stage2/pad_checksum -s 0xffffffff "C:/Users/gokua/OneDrive/Documents/Programming/C and C++/MorseCSDK/pico-sdk/src/rp2040/boot_stage2/bs2_default.bin" "C:/Users/gokua/OneDrive/Documents/Programming/C and C++/MorseCSDK/pico-sdk/src/rp2040/boot_stage2/bs2_default_padded_checksummed.S"

pico-sdk/src/rp2040/boot_stage2/bs2_default.bin: pico-sdk/src/rp2040/boot_stage2/bs2_default.elf
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --blue --bold --progress-dir="C:\Users\gokua\OneDrive\Documents\Programming\C and C++\MorseCSDK\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_2) "Generating bs2_default.bin"
	cd /d C:\Users\gokua\OneDrive\DOCUME~1\PROGRA~1\CANDC_~1\MORSEC~1\pico-sdk\src\rp2040\BOOT_S~1 && C:\Users\gokua\.pico-sdk\toolchain\14_2_Rel1\bin\arm-none-eabi-objcopy.exe -Obinary "C:/Users/gokua/OneDrive/Documents/Programming/C and C++/MorseCSDK/pico-sdk/src/rp2040/boot_stage2/bs2_default.elf" "C:/Users/gokua/OneDrive/Documents/Programming/C and C++/MorseCSDK/pico-sdk/src/rp2040/boot_stage2/bs2_default.bin"

pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/codegen:
.PHONY : pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/codegen

pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/bs2_default_padded_checksummed.S.obj: pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/flags.make
pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/bs2_default_padded_checksummed.S.obj: pico-sdk/src/rp2040/boot_stage2/bs2_default_padded_checksummed.S
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir="C:\Users\gokua\OneDrive\Documents\Programming\C and C++\MorseCSDK\CMakeFiles" --progress-num=$(CMAKE_PROGRESS_3) "Building ASM object pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/bs2_default_padded_checksummed.S.obj"
	cd /d C:\Users\gokua\OneDrive\DOCUME~1\PROGRA~1\CANDC_~1\MORSEC~1\pico-sdk\src\rp2040\BOOT_S~1 && C:\Users\gokua\.pico-sdk\toolchain\14_2_Rel1\bin\arm-none-eabi-gcc.exe $(ASM_DEFINES) $(ASM_INCLUDES) $(ASM_FLAGS) -o CMakeFiles\bs2_default_library.dir\bs2_default_padded_checksummed.S.obj   -c "C:\Users\gokua\OneDrive\Documents\Programming\C and C++\MorseCSDK\pico-sdk\src\rp2040\boot_stage2\bs2_default_padded_checksummed.S"

pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/bs2_default_padded_checksummed.S.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing ASM source to CMakeFiles/bs2_default_library.dir/bs2_default_padded_checksummed.S.i"
	cd /d C:\Users\gokua\OneDrive\DOCUME~1\PROGRA~1\CANDC_~1\MORSEC~1\pico-sdk\src\rp2040\BOOT_S~1 && C:\Users\gokua\.pico-sdk\toolchain\14_2_Rel1\bin\arm-none-eabi-gcc.exe $(ASM_DEFINES) $(ASM_INCLUDES) $(ASM_FLAGS) -E "C:\Users\gokua\OneDrive\Documents\Programming\C and C++\MorseCSDK\pico-sdk\src\rp2040\boot_stage2\bs2_default_padded_checksummed.S" > CMakeFiles\bs2_default_library.dir\bs2_default_padded_checksummed.S.i

pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/bs2_default_padded_checksummed.S.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling ASM source to assembly CMakeFiles/bs2_default_library.dir/bs2_default_padded_checksummed.S.s"
	cd /d C:\Users\gokua\OneDrive\DOCUME~1\PROGRA~1\CANDC_~1\MORSEC~1\pico-sdk\src\rp2040\BOOT_S~1 && C:\Users\gokua\.pico-sdk\toolchain\14_2_Rel1\bin\arm-none-eabi-gcc.exe $(ASM_DEFINES) $(ASM_INCLUDES) $(ASM_FLAGS) -S "C:\Users\gokua\OneDrive\Documents\Programming\C and C++\MorseCSDK\pico-sdk\src\rp2040\boot_stage2\bs2_default_padded_checksummed.S" -o CMakeFiles\bs2_default_library.dir\bs2_default_padded_checksummed.S.s

bs2_default_library: pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/bs2_default_padded_checksummed.S.obj
bs2_default_library: pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/build.make
.PHONY : bs2_default_library

# Rule to build all files generated by this target.
pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/build: bs2_default_library
.PHONY : pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/build

pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/clean:
	cd /d C:\Users\gokua\OneDrive\DOCUME~1\PROGRA~1\CANDC_~1\MORSEC~1\pico-sdk\src\rp2040\BOOT_S~1 && $(CMAKE_COMMAND) -P CMakeFiles\bs2_default_library.dir\cmake_clean.cmake
.PHONY : pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/clean

pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/depend: pico-sdk/src/rp2040/boot_stage2/bs2_default.bin
pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/depend: pico-sdk/src/rp2040/boot_stage2/bs2_default_padded_checksummed.S
	$(CMAKE_COMMAND) -E cmake_depends "MinGW Makefiles" "C:\Users\gokua\OneDrive\Documents\Programming\C and C++\MorseCSDK" C:\Users\gokua\.pico-sdk\sdk\2.1.1\src\rp2040\boot_stage2 "C:\Users\gokua\OneDrive\Documents\Programming\C and C++\MorseCSDK" "C:\Users\gokua\OneDrive\Documents\Programming\C and C++\MorseCSDK\pico-sdk\src\rp2040\boot_stage2" "C:\Users\gokua\OneDrive\Documents\Programming\C and C++\MorseCSDK\pico-sdk\src\rp2040\boot_stage2\CMakeFiles\bs2_default_library.dir\DependInfo.cmake" "--color=$(COLOR)"
.PHONY : pico-sdk/src/rp2040/boot_stage2/CMakeFiles/bs2_default_library.dir/depend

