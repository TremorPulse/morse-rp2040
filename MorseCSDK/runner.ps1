# profile_morse.ps1

# Set this to your ARM toolchain bin directory
$ARM_TOOLCHAIN = "C:\VSARM\armcc\10 2021.10\bin"

# Ensure build directory exists and rebuild project
if (-not (Test-Path "build")) {
    Write-Host "Building project first..."
    mkdir build
}
Write-Host "Building all binaries..."
cmake -B build -S .
cmake --build build

Write-Host "`n==== FLASH SIZE (TRANSMITTER) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-size.exe" build/transmitter.elf

Write-Host "`n==== FLASH SIZE (RECEIVER) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-size.exe" build/receiver.elf

Write-Host "`n==== MEMORY SECTIONS (TRANSMITTER) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-size.exe" -A build/transmitter.elf | Select-String -Pattern "\.text|\.data|\.bss|\.rodata"

Write-Host "`n==== MEMORY SECTIONS (RECEIVER) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-size.exe" -A build/receiver.elf | Select-String -Pattern "\.text|\.data|\.bss|\.rodata"

Write-Host "`n==== LARGEST FUNCTIONS (TRANSMITTER) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-nm.exe" --print-size --size-sort build/transmitter.elf | Select-String -Pattern " [tT] " | Select-Object -Last 10

Write-Host "`n==== LARGEST FUNCTIONS (RECEIVER) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-nm.exe" --print-size --size-sort build/receiver.elf | Select-String -Pattern " [tT] " | Select-Object -Last 10

Write-Host "`n==== LARGEST STATIC VARIABLES (TRANSMITTER) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-nm.exe" --print-size --size-sort build/transmitter.elf | Select-String -Pattern " [bBdD] " | Select-Object -Last 10

Write-Host "`n==== LARGEST STATIC VARIABLES (RECEIVER) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-nm.exe" --print-size --size-sort build/receiver.elf | Select-String -Pattern " [bBdD] " | Select-Object -Last 10

Write-Host "`n==== BINARY INFO (TRANSMITTER) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-readelf.exe" -h build/transmitter.elf | Select-String -Pattern "Magic|Class|Data|OS|ABI|Type|Machine|Entry|Flags"

Write-Host "`n==== BINARY INFO (RECEIVER) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-readelf.exe" -h build/receiver.elf | Select-String -Pattern "Magic|Class|Data|OS|ABI|Type|Machine|Entry|Flags"

Write-Host "`n==== DETAILED MEMORY SUMMARY (TRANSMITTER) ===="
Write-Host "Static data breakdown:"
$textContent = & "$ARM_TOOLCHAIN\arm-none-eabi-size.exe" -A build/transmitter.elf | Out-String

$textSize = if ($textContent -match "\.text\s+(\d+)") { [int]$Matches[1] } else { 0 }
$rodataSize = if ($textContent -match "\.rodata\s+(\d+)") { [int]$Matches[1] } else { 0 }
$dataSize = if ($textContent -match "\.data\s+(\d+)") { [int]$Matches[1] } else { 0 }
$bssSize = if ($textContent -match "\.bss\s+(\d+)") { [int]$Matches[1] } else { 0 }

Write-Host ("{0,-15} {1,8:N0} bytes (code)" -f ".text", $textSize)
Write-Host ("{0,-15} {1,8:N0} bytes (read-only data)" -f ".rodata", $rodataSize)
Write-Host ("{0,-15} {1,8:N0} bytes (initialized data)" -f ".data", $dataSize)
Write-Host ("{0,-15} {1,8:N0} bytes (uninitialized data)" -f ".bss", $bssSize)

$flashTotal = $textSize + $rodataSize + $dataSize
$ramTotal = $dataSize + $bssSize

Write-Host ("{0,-15} {1,8:N0} bytes" -f "FLASH TOTAL", $flashTotal)
Write-Host ("{0,-15} {1,8:N0} bytes" -f "RAM TOTAL", $ramTotal)

Write-Host "`n==== DETAILED MEMORY SUMMARY (RECEIVER) ===="
Write-Host "Static data breakdown:"
$textContent = & "$ARM_TOOLCHAIN\arm-none-eabi-size.exe" -A build/receiver.elf | Out-String

$textSize = if ($textContent -match "\.text\s+(\d+)") { [int]$Matches[1] } else { 0 }
$rodataSize = if ($textContent -match "\.rodata\s+(\d+)") { [int]$Matches[1] } else { 0 }
$dataSize = if ($textContent -match "\.data\s+(\d+)") { [int]$Matches[1] } else { 0 }
$bssSize = if ($textContent -match "\.bss\s+(\d+)") { [int]$Matches[1] } else { 0 }

Write-Host ("{0,-15} {1,8:N0} bytes (code)" -f ".text", $textSize)
Write-Host ("{0,-15} {1,8:N0} bytes (read-only data)" -f ".rodata", $rodataSize)
Write-Host ("{0,-15} {1,8:N0} bytes (initialized data)" -f ".data", $dataSize)
Write-Host ("{0,-15} {1,8:N0} bytes (uninitialized data)" -f ".bss", $bssSize)

$rxFlashTotal = $textSize + $rodataSize + $dataSize
$rxRamTotal = $dataSize + $bssSize

Write-Host ("{0,-15} {1,8:N0} bytes" -f "FLASH TOTAL", $rxFlashTotal)
Write-Host ("{0,-15} {1,8:N0} bytes" -f "RAM TOTAL", $rxRamTotal)

Write-Host "`n==== STACK SIZE ESTIMATION ===="
Write-Host "RP2040 default stack size: 2,048 bytes (2 KB)"
Write-Host "Note: The stack is dynamically allocated at runtime and not directly visible in the binary"
Write-Host "To modify the stack size, you can set _stack_size in the linker script"

Write-Host "`n==== TOTAL RESOURCE USAGE ===="
Write-Host "Transmitter:"
Write-Host "  Flash usage: $flashTotal bytes of 2,048 KB ($('{0:F2}' -f ($flashTotal / 2097152 * 100))%)"
Write-Host "  Static RAM: $ramTotal bytes"
Write-Host "  Estimated stack: 2,048 bytes"
Write-Host "  Total RAM: $(2048 + $ramTotal) bytes of 264 KB ($('{0:F2}' -f ((2048 + $ramTotal) / 270336 * 100))%)"

Write-Host "`nReceiver:"
Write-Host "  Flash usage: $rxFlashTotal bytes of 2,048 KB ($('{0:F2}' -f ($rxFlashTotal / 2097152 * 100))%)"
Write-Host "  Static RAM: $rxRamTotal bytes"
Write-Host "  Estimated stack: 2,048 bytes"
Write-Host "  Total RAM: $(2048 + $rxRamTotal) bytes of 264 KB ($('{0:F2}' -f ((2048 + $rxRamTotal) / 270336 * 100))%)"