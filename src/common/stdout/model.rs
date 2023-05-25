use colored::*;
use chrono::prelude::*;

pub enum LogType {
    Debug,
    Info,
    Error,
    Warn,
    Success,
    Fail,
}

pub fn hello() {
    println!("[====== {} {} {} ======]", "Backend".cyan(),  "API".red(),  "started".green());
}

pub fn debug<T>(message: &str, data: T) where T: std::fmt::Debug {
    log(LogType::Debug, message, data);
}

pub fn info<T>(message: &str, data: T) where T: std::fmt::Debug {
    log(LogType::Info, message, data);
}

pub fn error<T>(message: &str, data: T) where T: std::fmt::Debug {
    log(LogType::Error, message, data);
}

pub fn warn<T>(message: &str, data: T) where T: std::fmt::Debug {
    log(LogType::Warn, message, data);
}

pub fn success<T>(message: &str, data: T) where T: std::fmt::Debug {
    log(LogType::Success, message, data);
}

pub fn fail<T>(message: &str, data: T) where T: std::fmt::Debug {
    log(LogType::Fail, message, data);
}

pub fn log<T>(log_type: LogType, message: &str, data: T) where T: std::fmt::Debug {
    let sign = match log_type {
        LogType::Debug => "[!]".blue(),
        LogType::Info => "[>]".green(),
        LogType::Error => "[!]".red(),
        LogType::Warn => "[?]".yellow(),
        LogType::Success => "[+]".green(),
        LogType::Fail => "[-]".red(),
    };
    
    match log_type {
        LogType::Debug => println!("{} {} | {} {:#?}", sign, Utc::now(), message.yellow(), data),
        _ => println!("{} {} | {} {:?}", sign, Utc::now(), message.cyan(), data),
    };  
}
