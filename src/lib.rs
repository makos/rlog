use std::fs::OpenOptions;
use std::io;
use std::io::Write;

extern crate chrono;

// Tokens
const TIME: &str = "$time";
const TIMESHORT: &str = "$timeshort";
const DATE: &str = "$date";
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

    pub fn log(&self, msg: &str) -> bool {
        match OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
        {
            Ok(mut file) => {
                let final_msg = self.parse_format(msg);
                match file.write(final_msg.as_bytes()) {
                    Ok(_) => true,
                    Err(e) => {
                        eprintln!("{}", e);
                        false
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        }
    }

    pub fn parse_format(&self, msg: &str) -> String {
        let now = chrono::Local::now();
        let date_str = now.format("%d-%m-%Y %a").to_string();
        let time_str = now.format("[%H:%M.%S]").to_string();
        let timeshort_str = now.format("[%H:%M]").to_string();
        self.format
            .replace(DATE, &date_str)
            .replace(TIME, &time_str)
            .replace(TIMESHORT, &timeshort_str)
            .replace(MESSAGE, msg) + "\n"
    }
}
