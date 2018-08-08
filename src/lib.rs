use std::io;

// Tokens
const TIME: &str = "$time";
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

    pub fn log_str(&self, msg: &str) -> Result<(), Error> {
        Ok(())
    }
}
