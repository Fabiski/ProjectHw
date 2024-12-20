# Commandes
.PHONY: all build compile upload clean

# Default target
all: build compile upload

# Build the project for the AVR target
build:
	cargo +nightly build -Z build-std=core --target avr-unknown-gnu-atmega328 --release
	@echo "Rust compilation finished for atmega328."

	cargo +nightly build -Z build-std=core --target thumbv7m-none-eabi --release
	@echo "Rust compilation finished for cortex-3m."

# Compile the generated ELF to HEX
compile:
	@files=$$(Get-Content -Path "files.txt"); \
	"C:/Users/clemr/AppData/Local/Arduino15/packages/arduino/tools/avr-gcc/7.3.0-atmel3.6.1-arduino7/bin/avr-gcc.exe" -mmcu=atmega328 -o output.elf $$files; \
	"C:/Users/clemr/AppData/Local/Arduino15/packages/arduino/tools/avr-gcc/7.3.0-atmel3.6.1-arduino7/bin/avr-objcopy.exe" -O ihex ./output.elf output.hex
	@echo "Compiled to output.hex."

# Upload the HEX file to the AVR microcontroller
upload:
	"C:/Users/clemr/AppData/Local/Arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/bin/avrdude.exe" -C "C:/Users/clemr/AppData/Local/Arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/etc/avrdude.conf" -v -p atmega328p -c arduino -P COM7 -b 115200 -U flash:w:output.hex:i
	@echo "Upload to the microcontroller finished."

# Clean up generated files
clean:
	del output.elf output.hex
	@echo "Cleaned up generated files."
