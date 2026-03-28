#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        if *$crate::log::logger::DEBUG_ENABLED {
            $crate::log::logger::log(
                $crate::log::logger::Level::Debug,
                &format!($($arg)*)
            );
        }
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::log::logger::log(
            $crate::log::logger::Level::Info,
            &format!($($arg)*)
        );
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::log::logger::log(
            $crate::log::logger::Level::Warn,
            &format!($($arg)*)
        );
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::log::logger::log(
            $crate::log::logger::Level::Error,
            &format!($($arg)*)
        );
    }};
}
