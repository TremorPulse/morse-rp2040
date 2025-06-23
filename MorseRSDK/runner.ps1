# profile_morse.ps1

# Set target directory for Rust builds
$TARGET_DIR = "target/thumbv6m-none-eabi/release"

# First, ensure we have release builds of all binaries
Write-Host "Building release versions of all binaries..."
cargo build --release --bin transmitter
cargo build --release --bin receiver
cargo build --release --bin benchmarks

Write-Host "`n==== FLASH SIZE (TRANSMITTER) ===="
cargo size --release --bin transmitter

Write-Host "`n==== FLASH SIZE (RECEIVER) ===="
cargo size --release --bin receiver

Write-Host "`n==== MEMORY SECTIONS (TRANSMITTER) ===="
cargo size --release --bin transmitter -- -A | Select-String -Pattern "\.text|\.data|\.bss|\.rodata"

Write-Host "`n==== MEMORY SECTIONS (RECEIVER) ===="
cargo size --release --bin receiver -- -A | Select-String -Pattern "\.text|\.data|\.bss|\.rodata"

Write-Host "`n==== LARGEST FUNCTIONS (TRANSMITTER) ===="
cargo nm --release --bin transmitter -- --print-size --size-sort | Select-String -Pattern " [tT] " | Select-Object -Last 10

Write-Host "`n==== LARGEST FUNCTIONS (RECEIVER) ===="
cargo nm --release --bin receiver -- --print-size --size-sort | Select-String -Pattern " [tT] " | Select-Object -Last 10

Write-Host "`n==== LARGEST STATIC VARIABLES (TRANSMITTER) ===="
cargo nm --release --bin transmitter -- --print-size --size-sort | Select-String -Pattern " [bBdD] " | Select-Object -Last 10

Write-Host "`n==== LARGEST STATIC VARIABLES (RECEIVER) ===="
cargo nm --release --bin receiver -- --print-size --size-sort | Select-String -Pattern " [bBdD] " | Select-Object -Last 10

Write-Host "`n==== BINARY INFO (TRANSMITTER) ===="
cargo readobj --release --bin transmitter -- --all-headers | Select-String -Pattern "FILE|Format|Arch|Start|Size" -Context 0,1

Write-Host "`n==== BINARY INFO (RECEIVER) ===="
cargo readobj --release --bin receiver -- --all-headers | Select-String -Pattern "FILE|Format|Arch|Start|Size" -Context 0,1

Write-Host "`n==== DETAILED MEMORY SUMMARY (TRANSMITTER) ===="
Write-Host "Static data breakdown:"
$textSize = (cargo size --release --bin transmitter -- -A | Select-String -Pattern "\.text").ToString() -match "\.text\s+(\d+)" | Out-Null
$textSize = if ($Matches) { [int]$Matches[1] } else { 0 }

$rodataSize = (cargo size --release --bin transmitter -- -A | Select-String -Pattern "\.rodata").ToString() -match "\.rodata\s+(\d+)" | Out-Null
$rodataSize = if ($Matches) { [int]$Matches[1] } else { 0 }

$dataSize = (cargo size --release --bin transmitter -- -A | Select-String -Pattern "\.data").ToString() -match "\.data\s+(\d+)" | Out-Null
$dataSize = if ($Matches) { [int]$Matches[1] } else { 0 }

$bssSize = (cargo size --release --bin transmitter -- -A | Select-String -Pattern "\.bss").ToString() -match "\.bss\s+(\d+)" | Out-Null
$bssSize = if ($Matches) { [int]$Matches[1] } else { 0 }

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
$textSize = (cargo size --release --bin receiver -- -A | Select-String -Pattern "\.text").ToString() -match "\.text\s+(\d+)" | Out-Null
$textSize = if ($Matches) { [int]$Matches[1] } else { 0 }

$rodataSize = (cargo size --release --bin receiver -- -A | Select-String -Pattern "\.rodata").ToString() -match "\.rodata\s+(\d+)" | Out-Null
$rodataSize = if ($Matches) { [int]$Matches[1] } else { 0 }

$dataSize = (cargo size --release --bin receiver -- -A | Select-String -Pattern "\.data").ToString() -match "\.data\s+(\d+)" | Out-Null
$dataSize = if ($Matches) { [int]$Matches[1] } else { 0 }

$bssSize = (cargo size --release --bin receiver -- -A | Select-String -Pattern "\.bss").ToString() -match "\.bss\s+(\d+)" | Out-Null
$bssSize = if ($Matches) { [int]$Matches[1] } else { 0 }

Write-Host ("{0,-15} {1,8:N0} bytes (code)" -f ".text", $textSize)
Write-Host ("{0,-15} {1,8:N0} bytes (read-only data)" -f ".rodata", $rodataSize)
Write-Host ("{0,-15} {1,8:N0} bytes (initialized data)" -f ".data", $dataSize)
Write-Host ("{0,-15} {1,8:N0} bytes (uninitialized data)" -f ".bss", $bssSize)

$flashTotal = $textSize + $rodataSize + $dataSize
$ramTotal = $dataSize + $bssSize

Write-Host ("{0,-15} {1,8:N0} bytes" -f "FLASH TOTAL", $flashTotal)
Write-Host ("{0,-15} {1,8:N0} bytes" -f "RAM TOTAL", $ramTotal)

Write-Host "`n==== STACK SIZE ESTIMATION ===="
Write-Host "RP2040 default stack size: 2,048 bytes (2 KB)"
Write-Host "Note: Stack size in Rust embedded projects is typically set in .cargo/config.toml"
Write-Host "Actual stack usage can only be determined at runtime with special instrumentation"

Write-Host "`n==== TOTAL RESOURCE USAGE ===="
Write-Host "Transmitter:"
Write-Host "  Flash usage: $flashTotal bytes of 2,048 KB ($('{0:F2}' -f ($flashTotal / 2097152 * 100))%)"
Write-Host "  Static RAM: $ramTotal bytes"
Write-Host "  Estimated stack: 2,048 bytes"
Write-Host "  Total RAM: $(2048 + $ramTotal) bytes of 264 KB ($('{0:F2}' -f ((2048 + $ramTotal) / 270336 * 100))%)"

Write-Host "`nReceiver:"
$rxFlashTotal = $textSize + $rodataSize + $dataSize
$rxRamTotal = $dataSize + $bssSize
Write-Host "  Flash usage: $rxFlashTotal bytes of 2,048 KB ($('{0:F2}' -f ($rxFlashTotal / 2097152 * 100))%)"
Write-Host "  Static RAM: $rxRamTotal bytes"
Write-Host "  Estimated stack: 2,048 bytes"
Write-Host "  Total RAM: $(2048 + $rxRamTotal) bytes of 264 KB ($('{0:F2}' -f ((2048 + $rxRamTotal) / 270336 * 100))%)"