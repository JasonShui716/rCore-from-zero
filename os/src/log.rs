macro_rules! info {
    ($level: item, $fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[34m", $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    }
}