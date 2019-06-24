# `riscv-rust-quickstart`

> A template for building Rust applications for HiFive1 boards

This project is developed and maintained by the [RISC-V team][team].

## Getting started

1. Download [toolchain for SiFive boards](https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.1.0-2019.01.0-x86_64-linux-ubuntu14.tar.gz)

2. Download programmer software
  * HiFive1 Rev B: [Segger JLink software & documentation pack for Linux](https://www.segger.com/downloads/jlink/)
  * HiFive1: [OpenOCD from SiFive](https://static.dev.sifive.com/dev-tools/riscv-openocd-0.10.0-2019.02.0-x86_64-linux-ubuntu14.tar.gz)

3. Install `riscv32imac-unknown-none-elf` target
```sh
rustup target add riscv32imac-unknown-none-elf
```

4. If you have an old HiFive1 board, fix `Cargo.toml`: replace `board-hifive1-revb` with `board-hifive1`

5. Run the programmer software
  * HiFive1 Rev B:
```sh
/path/to/JLinkGDBServer -device FE310 -if JTAG -speed 4000 -port 3333
```
  * HiFive1:
```sh
/path/to/openocd -f board/sifive-hifive1.cfg
```

6. Connect your board and open a serial console (if you need serial output)
```sh
screen /dev/ttyACM0 115200  # /dev/ttyUSB1 for HiFive1
```

7. Build and run the example
```sh
cargo build --example hello_world
cargo run --example hello_world
```


## License
Copyright 2017 David Craven

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted, provided that the above copyright notice
and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.

[team]: https://github.com/rust-embedded/wg#the-riscv-team
