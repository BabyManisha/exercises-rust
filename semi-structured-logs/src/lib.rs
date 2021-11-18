// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

fn error_type(err: LogLevel) -> String {
    match err {
        LogLevel::Info => "INFO".to_string(),
        LogLevel::Warning => String::from("WARNING"),
        LogLevel::Error => "ERROR".to_string(),
    }
}


/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    // unimplemented!()
    let msg = format!("[{}]: {}", error_type(level), message);
    msg
    // concat!("[", level, "]: ", message.to_string())
}
pub fn info(message: &str) -> String {
    // unimplemented!()
    let msg = format!("[INFO]: {}", message);
    msg
}
pub fn warn(message: &str) -> String {
    // unimplemented!()
    let msg = format!("[WARNING]: {}", message);
    msg
}
pub fn error(message: &str) -> String {
    // unimplemented!()
    let msg = format!("[ERROR]: {}", message);
    msg
}
