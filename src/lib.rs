//! A crate providing the [`qemu_print`] and [`qemu_println`] macros to print strings to the
//! console using QEMU's serial port support.
//!
//! # Usage
//!
//! Add `-serial stdio` to the QEMU's commandline parameters.
//!
//! ```text
//! qemu-system-x86_64 ... -serial stdio
//! ```
//!
//! Invoke macros like as [`print!`] and [`println!`].
//!
//! [`print!`]: https://doc.rust-lang.org/std/macro.print.html
//! [`println!`]: https://doc.rust-lang.org/std/macro.println.html
//!
//! ```rust,no_run
//! use qemu_print::qemu_println;
//!
//! qemu_println!("This string will be printed to the console.");
//!
//! let x = 3;
//! qemu_println!("x = {}", x);
//! ```

#![no_std]

use conquer_once::spin::Lazy;
use core::fmt;
use core::fmt::Write;
use spinning_top::Spinlock;
use uart_16550::SerialPort;

#[cfg(not(any(feature = "stable", feature = "nightly")))]
compile_error!("Specify either `stable` or `nightly` feature.");

#[cfg(all(feature = "stable", feature = "nightly"))]
compile_error!("`stable` and `nightly` features cannot be enabled at the same time.");

static PORT: Lazy<Spinlock<SerialPort>> = Lazy::new(|| {
    let mut port = unsafe { SerialPort::new(0x3f8) };

    port.init();

    Spinlock::new(port)
});

/// Print a string to the console.
///
/// # Examples
///
/// ```rust,no_run
/// use qemu_print::qemu_print;
///
/// qemu_print!("hello world.");
/// ```
#[macro_export]
macro_rules! qemu_print{
    ($($arg:tt)*)=>{
        $crate::_print(format_args!($($arg)*));
    }
}

/// Print a string with newline to the console.
///
/// ```rust,no_run
/// use qemu_print::qemu_println;
///
/// qemu_println!("hello world.");
/// ```
#[macro_export]
macro_rules! qemu_println {
    () => {
        $crate::qemu_print!("\n");
    };
    ($($arg:tt)*) => {
        $crate::qemu_print!("{}\n", format_args!($($arg)*));
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments<'_>) {
    let _ = PORT.lock().write_fmt(args);
}
