use chrono::prelude::*;
use colored::*;

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (info(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (warn(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (error(format_args!($($arg)*)));
}

pub fn info(msg: std::fmt::Arguments) {
    println!(
        "{} {}: {}",
        "[INFO]".green().bold(),
        format!("[{}]", Local::now().format("%Y-%m-%d %H:%M:%S")).yellow(),
        msg
    );
}

pub fn warn(msg: std::fmt::Arguments) {
    println!(
        "{} {}: {}",
        "[WARN]".purple().bold(),
        format!("[{}]", Local::now().format("%Y-%m-%d %H:%M:%S")).yellow(),
        msg
    );
}

pub fn error(msg: std::fmt::Arguments) {
    println!(
        "{} {}: {}",
        "[ERROR]".red().bold(),
        format!("[{}]", Local::now().format("%Y-%m-%d %H:%M:%S")).yellow(),
        msg
    );
}
