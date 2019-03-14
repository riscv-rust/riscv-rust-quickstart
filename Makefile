TARGET       := riscv32imac-unknown-none-elf

build:
	cargo build --target $(TARGET)

run:
	cargo run --target $(TARGET)

clean:
	cargo clean

openocd:
	openocd -f openocd.cfg

.PHONY: build run clean openocd
