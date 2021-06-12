# `qemu_print`

A Rust library to print strings to a console using QEMU's serial port support.

## Usage

Add `-serial stdio` to the QEMU's command line parameters.

```
qemu-system-x86_64 -serial stdio /* other parameters... */
```

Invoke macros like as [`print!`](https://doc.rust-lang.org/std/macro.print.html) and [`println!`](https://doc.rust-lang.org/std/macro.println.html).

```rust
use qemu_print::qemu_println;

qemu_println!("This string will be printed to the console.");

let x = 3;
qemu_println!("x = {}", x);
```

## Features

This crate has two features:

- `nightly`: This is enabled by default. The Nightly Rust must be enabled.
- `stable`: Use this feature if you do not use the Nightly Rust.  Add `--no-default-features, --features = ["stable"]` to `Cargo.toml`'s dependency option. This feature uses [`cc`](https://github.com/alexcrichton/cc-rs) crate as a build-time dependency. You need to install [the compile-time requirements](https://github.com/alexcrichton/cc-rs#compile-time-requirements). This feature only supports targets which use System V AMD64 ABI.

Enable one of these features. If both, or none of them are specified, this crate will emit a compile error.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
