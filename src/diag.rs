use core::fmt;

use crate::lexer::{Token, TokenKind};

pub enum LogLevel {
    Info,
    Warning,
    Error,
}
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            LogLevel::Info => "[INFO]",
            LogLevel::Warning => "[WARN]",
            LogLevel::Error => "[ERROR]",
        };
        write!(f, "{}", out)
    }
}
pub struct Diagnoster {}

impl Diagnoster {
    pub fn report(&self, level: LogLevel, msg: &str) {
        eprintln!("{}:  {}", level, msg);
    }
}
