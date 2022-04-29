// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    // unimplemented!()
    format!("[{}]: {}", format!("{:?}", level).to_uppercase(), message)
}
pub fn info(message: &str) -> String {
    // unimplemented!()
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    // unimplemented!()
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    // unimplemented!()
    log(LogLevel::Error, message)
}
