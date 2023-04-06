use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}
impl From<usize> for LogLevel {
    fn from(i: usize) -> Self {
        match i {
            0 => LogLevel::Error,
            1 => LogLevel::Warning,
            2 => LogLevel::Info,
            3 => LogLevel::Debug,
            _ => panic!("Invalid log level"),
        }
    }
}
impl From<LogLevel> for usize {
    fn from(l: LogLevel) -> Self {
        match l {
            LogLevel::Error => 0,
            LogLevel::Warning => 1,
            LogLevel::Info => 2,
            LogLevel::Debug => 3,
        }
    }
}
impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Warning
    }
}

fn get_log_level() -> LogLevel {
    crate::PERS_DATA.with(|perm_data| perm_data.borrow().log_level)
}

pub fn extern_log(s: &str, log_level: LogLevel) {
    if log_level > get_log_level() {
        return;
    }
    #[cfg(feature = "wasm")]
    crate::console_log!("{}", s);
    #[cfg(not(feature = "wasm"))]
    println!("{}", s);
}

pub fn log(s: &str, log_level: usize) {
    extern_log(s, log_level.into())
}
