#[macro_export]
macro_rules! csl_info {
    ($($arg:tt)*) => {
        print!("[\x1b[38;2;0;255;255mINFO\x1b[0m]\u{0020}");
        println!($($arg)*);
    };
}
#[macro_export]
macro_rules! csl_debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            print!("[DEBUG]\u{0020}");
            println!($($arg)*);
        }
    };
}
#[macro_export]
macro_rules! csl_warn {
    ($($arg:tt)*) => {
        print!("[\x1b[38;2;255;47;0mWARN\x1b[0m]\u{0020}");
        println!($($arg)*);
    };
}
#[macro_export]
macro_rules! csl_err {
    ($($arg:tt)*) => {
        eprint!("[\x1b[38;2;237;19;0mINFO\x1b[0m]\u{0020}");
        eprintln!($($arg)*);
    };
}
