# `riscv-rust-quickstart`

> A template for building Rust applications for HiFive1 boards

This project is developed and maintained by the [RISC-V team][team].

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.36 or a newer toolchain. e.g. `rustup default stable`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the RISC-V target. Run:

``` console
$ rustup target add riscv32imac-unknown-none-elf
```

- [RISC-V toolchain for SiFive boards](https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.1.0-2019.01.0-x86_64-linux-ubuntu14.tar.gz)
  * Extract this package.
  * See "Set up `gdb`", below.

- Programmer software
  * HiFive1 Rev B: [Segger JLink software & documentation pack for Linux](https://www.segger.com/downloads/jlink/)
  * HiFive1: [OpenOCD from SiFive](https://static.dev.sifive.com/dev-tools/riscv-openocd-0.10.0-2019.02.0-x86_64-linux-ubuntu14.tar.gz) 

## Using this template

**NOTE**: This is the very short version that only covers building programs. For
the long version, which additionally covers flashing, running and debugging
programs, check [the embedded Rust book](https://rust-embedded.github.io/book).

1. Instantiate the template.

``` console
$ cargo generate --git https://github.com/riscv-rust/riscv-rust-quickstart
 Project Name: app
 Creating project called `app`...
 Done! New project created /tmp/app

$ cd app
```

2. If you have an old HiFive1 board, edit `Cargo.toml`:
replace `board-hifive1-revb` with `board-hifive1`.

3. Set up `gdb`:
  * Open `.cargo/config`
  * Edit the line that reads `runner = "riscv32imac-unknown-elf-gdb -q -x gdb_init"` to read:
  ```
  runner = "/path/to/riscv64-unknown-elf-gcc-8.1.0-2019.01.0-x86_64-linux-ubuntu14/bin/riscv64-unknown-elf-gdb -q -x gdb_init"
  ```
  or ensure that `riscv64-unknown-elf-gdb` is on your `$PATH`.
  (`riscv64-unknown-elf-gdb` was downloaded and extracted in the `RISC-V toolchain for SiFive
  boards` step, above.)

4. Run the programmer software.
  * HiFive1 Rev B:
```sh
/path/to/JLinkGDBServer -device FE310 -if JTAG -speed 4000 -port 3333
```
  * HiFive1:
```sh
/path/to/openocd -f board/sifive-hifive1.cfg
```

5. Build the template application or one of the examples.

``` console
$ cargo build
or
$ cargo build --example leds_blink
```

5. Run the template application or one of the examples.

``` console
$ cargo run
or
$ cargo run --example leds_blink
```


## License
Copyright 2017-2019 [RISC-V team][team]

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
