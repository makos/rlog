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
//!
//! Changelog for current version can be found in the [repository](https://github.com/makos/rlog).

#![doc(html_root_url = "https://docs.rs/rlog/1.0.0")]

use std::fs::OpenOptions;
use std::io::Write;

extern crate chrono;

/// Main structure.
///
/// ````
/// pub struct Logger {
///     path: String,
///     format: String,
/// }
/// ````
///
/// * `path`: filepath to the log file you want to use, can be relative.
/// * `format`: timestamp in [ISO8061](https://en.wikipedia.org/wiki/ISO_8601) format.
///
/// Please note that the log file isn't created after creating a new instance of Logger struct,
/// but rather after the first call to `log()` method. That is also the time when filepath given is validated.    
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
/// let log = Logger::new("my.log", ""); // Note the empty string literal as second parameter.
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
/// let log = Logger::new("my.log", "%d-%m %H:%M");
///
/// assert!(log.log("Custom format test"));
/// # remove_file("my.log").unwrap();
/// ````
///
/// Output:
///
/// `my.log`
/// ````ignore
/// 10-08 10:46 Custom format test
/// ````

pub struct Logger {
    path: String,
    format: String,
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
    /// let log = Logger::new("test.log", "%a %H:%M");
    /// ````
    pub fn new(path: &str, format: &str) -> Logger {
        let mut _format = "%d-%m-%Y %a %H:%M.%S";

        if format != "" {
            _format = format;
        }

        Logger {
            path: path.to_owned(),
            format: _format.to_owned(),
        }
    }

    /// Format setter method.
    pub fn fmt(&mut self, fmt: &str) {
        self.format = fmt.to_owned();
    }

    /// Format getter method.
    pub fn get_fmt(&self) -> &str {
        &self.format
    }

    /// Path setter method.
    pub fn path(&mut self, path: &str) {
        self.path = path.to_owned();
    }

    /// Path getter method.
    pub fn get_path(&self) -> &str {
        &self.path
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
    /// let log = Logger::new("my.log", "");
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
        now.format(&self.format).to_string() + " " + msg + "\n"
    }
}
