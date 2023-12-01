#[cfg(feature = "debug")]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        eprintln!($($arg)*);
    };
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($($arg:tt)*) => {};
}
