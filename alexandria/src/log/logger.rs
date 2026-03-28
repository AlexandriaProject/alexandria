#![allow(dead_code)]
#![allow(unused)]

use std::env;
use once_cell::sync::Lazy;

pub static DEBUG_ENABLED: Lazy<bool> = Lazy::new(|| {
    env::var("DEBUG").map(|v| v == "1").unwrap_or(false)
});

#[derive(Copy, Clone)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

pub fn log(level: Level, msg: &str) {
    let (label, color) = match level {
        Level::Debug => ("DEBUG", "\x1b[36m"),
        Level::Info  => ("INFO",  "\x1b[34m"),
        Level::Warn  => ("WARN",  "\x1b[33m"),
        Level::Error => ("ERROR", "\x1b[31m"),
    };

    println!("{color}[{label}]\x1b[00m {msg}");
}
