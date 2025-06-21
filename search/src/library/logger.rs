use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

// Path to log file
use crate::LOG_PATH;

/// List of different types of log headers.
#[allow(dead_code)]
pub enum Header {
    SUCCESS,
    INFO,
    WARNING,
    ERROR
}

/// Logs a message to the console.
pub fn log(header: Header, message: &str) {
    let log_path = LOG_PATH;
    let log_path = log_path.get().expect("LOG_PATH is not set");
    // Type of message to log
    let header = match header {
        Header::SUCCESS => "SUCCESS",
        Header::INFO => "INFO",
        Header::WARNING => "WARNING",
        Header::ERROR => "ERROR"
    };

    // Print the log to the console
    println!("[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message);

    // Write the log to a file
    if Path::new(log_path).exists() {
        let mut log_file = OpenOptions::new().append(true).open(log_path).unwrap();
        writeln!(log_file, "[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message).unwrap();
    } else {
        let mut log_file = OpenOptions::new().create_new(true).append(true).open(log_path).unwrap();
        writeln!(log_file, "[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message).unwrap();
    }
}
