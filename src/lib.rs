//! A minimal logging library.
//!
//! This is a tiny library used to write stuff happening in your program to a file.
use std::fs::OpenOptions;
// use std::io;
use std::io::Write;

extern crate chrono;

// Tokens
const TIME: &str = "$time";
const TIMESHORT: &str = "$timeshort";
const DATE: &str = "$date";
const MESSAGE: &str = "$msg";

/// Main structure.
///
/// ````
/// pub struct Logger {
///     pub path: String,
///     pub format: String,
/// }
/// ````
/// * `path`: filepath to the log file you want to use, can be relative.
/// * `format`: output format to be used.
///
/// Available format tokens:
/// * `$time`: logs time in HH:MM.SS format.
/// * `$timeshort`: logs time in HH:MM format.
/// * `$date`: logs current date in DD-MM-YYYY Day format, where "Day" is a three letter abbreviation of week-day name.
/// * '$msg': where the actual message will be output.
///
/// # Example
///
/// ```` ignore
/// extern crate rlog;
/// use rlog::Logger;
///
/// let log = Logger::new("my.log", "$date $time $msg");
/// assert!(log.log("Just testing"));
/// assert!(log.log("Another test"));
/// ````
///
/// Result is in file `my.log` in the root directory of your crate:
///
/// `10.08.2018 09:06.33 Just testing`
///
/// `10.08.2018 09:06.34 Another test`
///
pub struct Logger {
    pub path: String,
    pub format: String,
}

impl Logger {
    /// Create new instance.
    ///
    /// # Example
    ///
    /// ````
    /// extern crate rlog;
    /// use rlog::Logger;
    ///
    /// let log = Logger::new("test.log", "$msg"); // This format will just log messages without any timestamps.
    /// ````
    pub fn new(path: &str, format: &str) -> Logger {
        Logger {
            path: path.to_string(),
            format: format.to_string(),
        }
    }

    /// Log message `msg` to file.
    ///
    /// Returns `true` on succesful write, `false` otherwise and prints error message to stderr.
    ///
    /// # Example
    ///
    /// ```` ignore
    /// extern crate rlog;
    /// use rlog::Logger;
    ///
    /// let log = Logger::new("my.log", "$date $time $msg");
    /// assert!(log.log("Just testing"));
    /// ````
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

    fn parse_format(&self, msg: &str) -> String {
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
