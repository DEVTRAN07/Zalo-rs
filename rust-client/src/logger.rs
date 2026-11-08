//! Simple leveled logger for the Zalo client.

use std::sync::atomic::{AtomicU8, Ordering};

static LOG_LEVEL: AtomicU8 = AtomicU8::new(3); // info by default

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum LogLevel {
    Silent = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Verbose = 4,
}

pub fn set_log_level(level: LogLevel) {
    LOG_LEVEL.store(level as u8, Ordering::Relaxed);
}

pub fn current_level() -> LogLevel {
    match LOG_LEVEL.load(Ordering::Relaxed) {
        0 => LogLevel::Silent,
        1 => LogLevel::Error,
        2 => LogLevel::Warn,
        3 => LogLevel::Info,
        _ => LogLevel::Verbose,
    }
}

#[macro_export]
macro_rules! zlog_error {
    ($($arg:tt)*) => {
        if $crate::logger::current_level() >= $crate::logger::LogLevel::Error {
            eprintln!("\x1b[31mERROR\x1b[0m {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! zlog_warn {
    ($($arg:tt)*) => {
        if $crate::logger::current_level() >= $crate::logger::LogLevel::Warn {
            eprintln!("\x1b[33mWARN\x1b[0m {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! zlog_info {
    ($($arg:tt)*) => {
        if $crate::logger::current_level() >= $crate::logger::LogLevel::Info {
            eprintln!("\x1b[34mINFO\x1b[0m {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! zlog_verbose {
    ($($arg:tt)*) => {
        if $crate::logger::current_level() >= $crate::logger::LogLevel::Verbose {
            eprintln!("\x1b[35mVERBOSE\x1b[0m {}", format!($($arg)*));
        }
    };
}
