#![no_std]

use conquer_once::spin::Lazy;
use core::fmt;
use core::fmt::Write;
use spinning_top::Spinlock;
use uart_16550::SerialPort;

static PORT: Lazy<Spinlock<SerialPort>> = Lazy::new(|| {
    let mut port = unsafe { SerialPort::new(0x3f8) };

    port.init();

    Spinlock::new(port)
});

#[macro_export]
macro_rules! qemu_print{
    ($($arg:tt)*)=>{
        $crate::_print(format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! qemu_println {
    () => {
        $crate::qemu_print!("\n");
    };
    ($($arg:tt)*) => {
        $crate::qemu_print!("{}\n", format_args!($arg));
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments<'_>) {
    let _ = PORT.lock().write_fmt(args);
}
