TARGET       := riscv32imac-unknown-none
OPENOCD_CFG  := $(RISCV_RUST_TOOLCHAIN)/openocd.cfg

build:
	xargo build --target $(TARGET)

run:
	xargo run --target $(TARGET)

clean:
	xargo clean

openocd:
	openocd -f $(OPENOCD_CFG)

.PHONY: build run clean openocd
