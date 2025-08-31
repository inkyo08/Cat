use std::sync::Mutex;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

// Log levels
pub const LOG_TRACE: i32 = 0;
pub const LOG_DEBUG: i32 = 1;
pub const LOG_INFO: i32 = 2;
pub const LOG_WARN: i32 = 3;
pub const LOG_ERROR: i32 = 4;
pub const LOG_FATAL: i32 = 5;

static LOG_LEVEL: Mutex<i32> = Mutex::new(LOG_TRACE);

// ANSI color codes
const COLORS: [&str; 6] = [
    "\x1b[94m", // TRACE - bright blue
    "\x1b[36m", // DEBUG - cyan
    "\x1b[32m", // INFO - green
    "\x1b[33m", // WARN - yellow
    "\x1b[31m", // ERROR - red
    "\x1b[35m", // FATAL - magenta
];

const LEVEL_NAMES: [&str; 6] = ["TRACE", "DEBUG", "INFO", "WARN", "ERROR", "FATAL"];

fn format_time() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let hours = (secs / 3600) % 24;
            let minutes = (secs / 60) % 60;
            let seconds = secs % 60;
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        }
        Err(_) => "00:00:00".to_string(),
    }
}

pub fn set_level(level: i32) {
    if let Ok(mut current_level) = LOG_LEVEL.lock() {
        *current_level = level;
    }
}

fn log_message(level: i32, file: &str, line: u32, message: &str) {
    if let Ok(current_level) = LOG_LEVEL.lock() {
        if level < *current_level {
            return;
        }
    }
    
    let time_str = format_time();
    let level_name = if level >= 0 && level < LEVEL_NAMES.len() as i32 {
        LEVEL_NAMES[level as usize]
    } else {
        "UNKNOWN"
    };
    
    let color = if level >= 0 && level < COLORS.len() as i32 {
        COLORS[level as usize]
    } else {
        ""
    };
    
    println!(
        "{} {}{:<5}\x1b[0m \x1b[90m{}:{}:\x1b[0m {}",
        time_str, color, level_name, file, line, message
    );
    
    io::stdout().flush().unwrap_or(());
}

pub fn log_trace(file: &str, line: u32, message: &str) {
    log_message(LOG_TRACE, file, line, message);
}

pub fn log_debug(file: &str, line: u32, message: &str) {
    log_message(LOG_DEBUG, file, line, message);
}

pub fn log_info(file: &str, line: u32, message: &str) {
    log_message(LOG_INFO, file, line, message);
}

pub fn log_warn(file: &str, line: u32, message: &str) {
    log_message(LOG_WARN, file, line, message);
}

pub fn log_error(file: &str, line: u32, message: &str) {
    log_message(LOG_ERROR, file, line, message);
}

pub fn log_fatal(file: &str, line: u32, message: &str) {
    log_message(LOG_FATAL, file, line, message);
}
