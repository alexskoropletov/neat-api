use std::env;

use chrono::prelude::*;
use colored::*;

#[derive(PartialEq, PartialOrd)]
pub enum LogType {
    Info,
    Debug,
    Warn,
    Error,
    Success,
    Fail,
}

impl LogType {
    pub fn from_string(string: &str) -> LogType {
        match string {
            "info" => LogType::Info,
            "debug" => LogType::Debug,
            "warn" => LogType::Warn,
            "error" => LogType::Error,
            "success" => LogType::Success,
            "fail" => LogType::Fail,
            _ => LogType::Warn,
        }
    }
}

pub fn hello() {
    println!(
        "[====== {} {} {} ======]",
        "Backend".cyan(),
        "API".red(),
        "started".green()
    );
}

pub fn debug<T>(message: &str, data: T)
where
    T: std::fmt::Debug,
{
    log(LogType::Debug, message, data);
}

pub fn info<T>(message: &str, data: T)
where
    T: std::fmt::Debug,
{
    log(LogType::Info, message, data);
}

pub fn error<T>(message: &str, data: T)
where
    T: std::fmt::Debug,
{
    log(LogType::Error, message, data);
}

pub fn warn<T>(message: &str, data: T)
where
    T: std::fmt::Debug,
{
    log(LogType::Warn, message, data);
}

pub fn success<T>(message: &str, data: T)
where
    T: std::fmt::Debug,
{
    log(LogType::Success, message, data);
}

pub fn fail<T>(message: &str, data: T)
where
    T: std::fmt::Debug,
{
    log(LogType::Fail, message, data);
}

pub fn log<T>(log_type: LogType, message: &str, data: T)
where
    T: std::fmt::Debug,
{
    let sign = match log_type {
        LogType::Debug => "[!]".blue(),
        LogType::Info => "[>]".green(),
        LogType::Warn => "[?]".yellow(),
        LogType::Error => "[!]".red(),
        LogType::Success => "[+]".green(),
        LogType::Fail => "[-]".red(),
    };

    let level: LogType = match env::var("LOG_LEVEL") {
        Ok(value) => LogType::from_string(&value),
        Err(_) => LogType::Warn,
    };
    if level > log_type {
        return;
    }

    match log_type {
        LogType::Debug => println!("{} {} | {} {:#?}", sign, Utc::now(), message.yellow(), data),
        _ => println!("{} {} | {} {:?}", sign, Utc::now(), message.cyan(), data),
    };
}
