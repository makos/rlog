// Copyright (C) 2018 Mateusz Makowski

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! A minimal logging library.
//!
//! This is a tiny library used to write stuff happening in your program to a file.

#![doc(html_root_url = "https://docs.rs/rlog/0.2.0")]

use std::fs::OpenOptions;
use std::io::Write;

extern crate chrono;

// Tokens
const TIME: &str = "$time";
const DATE: &str = "$date";
const MESSAGE: &str = "$msg";

/// Main structure.
///
/// ````
/// pub struct Logger {
///     pub path: String,
///     pub format: String,
///     pub time_fmt: String,
///     pub date_fmt: String,
/// }
/// ````
///
/// * `path`: filepath to the log file you want to use, can be relative.
/// * `format`: date, time and message output order to the log.
/// * `time_fmt`: ISO8061-compliant time format string (i.e. `%H:%M`).
/// * `date_fmt`: ISO8061-compliant date format string (i.e. `%d-%m-%Y`).
///
/// Available format tokens:
/// * `$time`: logs time in HH:MM.SS (by default) format.
/// * `$date`: logs current date in DD-MM-YYYY Day (by default) format, where "Day" is a three letter abbreviation of week-day name.
/// * `$msg`: where the actual message will be output.
///
/// # Examples
///
/// ## Default timestamp formatting
///
/// ````
/// extern crate rlog;
/// use rlog::Logger;
/// # use std::fs::remove_file;
///
/// let log = Logger::new("my.log", "$date $time $msg");
/// // Method log() returns a bool to signal succes or failure.
/// assert!(log.log("Just testing"));
/// assert!(log.log("Another test"));
/// # remove_file("my.log").unwrap();
/// ````
///
/// Result is in file `my.log` in the root directory of your crate:
///
/// ````ignore
/// 10.08.2018 09:06.33 Just testing
/// 10.08.2018 09:06.34 Another test
/// ````
///
/// ## Custom formatting
///
/// rlog supports (thanks to [chrono](https://crates.io/crates/chrono)) [ISO 8061](https://en.wikipedia.org/wiki/ISO_8601) timestamp formatting.
///
/// ````
/// extern crate rlog;
/// use rlog::Logger;
/// # use std::fs::remove_file;
///
/// let mut log = Logger::new("my.log", "$date $time $msg"); // Note that this is a mutable instance.
/// log.time_fmt = String::from("%H");
/// log.date_fmt = String::from("%d-%m");
///
/// assert!(log.log("Custom format test"));
/// # remove_file("my.log").unwrap();
/// ````
///
/// Output:
///
/// `my.log`
/// ````ignore
/// 10-08 10 Custom format test
/// ````
///
pub struct Logger {
    pub path: String,
    pub format: String,
    pub time_fmt: String,
    pub date_fmt: String,
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
        let mut _format = "$date $time $msg";

        if format != "" {
            _format = format;
        }

        Logger {
            path: path.to_owned(),
            format: _format.to_owned(),
            time_fmt: String::from("%H:%M.%S"),
            date_fmt: String::from("%d-%m-%Y %a"),
        }
    }

    /// Log message `msg` to file.
    ///
    /// This method will create the log file if it does not exist.
    ///
    /// # Errors
    ///
    /// If there are problems opening a file, the method returns `false` and prints error to stderr.
    /// `Result<T, Err>` isn't used for the sake of simplicity.
    ///
    /// # Example
    ///
    /// ````
    /// extern crate rlog;
    /// use rlog::Logger;
    /// # use std::fs::remove_file;
    ///
    /// let log = Logger::new("my.log", "$date $time $msg");
    /// assert!(log.log("Just testing"));
    /// # remove_file("my.log").unwrap();
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
                        eprintln!("Logger::log() write error:\n{}\n", e);
                        false
                    }
                }
            }
            Err(e) => {
                eprintln!("Logger::log() open error:\n{}\n", e);
                false
            }
        }
    }

    fn parse_format(&self, msg: &str) -> String {
        let now = chrono::Local::now();
        let date_str = now.format(&self.date_fmt).to_string();
        let time_str = now.format(&self.time_fmt).to_string();
        self.format
            .replace(DATE, &date_str)
            .replace(TIME, &time_str)
            .replace(MESSAGE, msg) + "\n"
    }
}
