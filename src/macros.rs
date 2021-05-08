
macro_rules! println {
    () => ({
        print!("\n")
    });
    ($fmt:expr) => ({
	print!(concat!($fmt, "\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
	print!(concat!($fmt, "\n"), $($args)+)
    });
}

macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        let _ = write!(crate::CONSOLE, $($args)+);
    });
}
