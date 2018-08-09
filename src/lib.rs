use std::fs::OpenOptions;
use std::io;
use std::io::Write;

extern crate chrono;

// Tokens
// const TIME: &str = "$time";
const DATETIME: &str = "$datetime";
const MESSAGE: &str = "$msg";

pub struct Logger {
    pub path: String,
    pub format: String,
}

impl Logger {
    pub fn new(path: &str, format: &str) -> Logger {
        Logger {
            path: path.to_string(),
            format: format.to_string(),
        }
    }

    pub fn log(&self, msg: &str) -> io::Result<bool> {
        match OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
        {
            Ok(mut file) => {
                let final_msg = self.parse_format(msg) + "\n";
                match file.write(final_msg.as_bytes()) {
                    Ok(_) => Ok(true),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn log_string(&self, msg: String) -> io::Result<bool> {
        match OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
        {
            Ok(mut file) => {
                let final_msg = self.parse_format_string(msg) + "\n";
                match file.write(final_msg.as_bytes()) {
                    Ok(_) => Ok(true),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn parse_format(&self, msg: &str) -> String {
        let now = chrono::Local::now();
        let datetime_str = now.format("%d-%m-%Y %a [%H:%M.%S] ");
        self.format
            .replace(DATETIME, &datetime_str.to_string()[..])
            .replace(MESSAGE, msg)
    }

    fn parse_format_string(&self, msg: String) -> String {
        let now = chrono::Local::now();
        let datetime_str = now.format("%d-%m-%Y %a [%H:%M.%S] ");
        self.format
            .replace(DATETIME, &datetime_str.to_string())
            .replace(MESSAGE, &msg)
    }
}
