#![allow(dead_code)]
#![allow(unused)]

use std::env;
use once_cell::sync::Lazy;

static DEBUG_ENABLED: Lazy<bool> = Lazy::new(|| {
    env::var("DEBUG").unwrap_or_else(|_| "0".to_string()) == "1"
});

pub fn debug(msg: &str) {
    if *DEBUG_ENABLED {
        println!("\x1b[36m[DEBUG]\x1b[00m {msg}");
    }
}

pub fn info(msg: &str) {
    println!("\x1b[34m[INFO]\x1b[00m {msg}");
}

pub fn warn(msg: &str) {
    println!("\x1b[33m[WARN]\x1b[00m {msg}");
}

pub fn error(msg: &str) {
    println!("\x1b[31m[ERROR]\x1b[00m {msg}");
}
