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

extern crate rlog;

mod testenv;

use rlog::Logger;

const TESTLOG: &str = "test.log";
const FORMAT: &str = "%d-%m-%Y %a %H:%M";

#[test]
fn instantiate() {
    let log = Logger::new(TESTLOG, FORMAT);
    assert_eq!(log.get_fmt(), FORMAT);
    assert_eq!(log.get_path(), TESTLOG);
}

#[test]
fn logging_str() {
    let log = Logger::new("logging_str.log", FORMAT);

    assert!(log.log("logging_str()"));

    assert!(testenv::check_and_delete(
        "logging_str()",
        "logging_str.log",
        FORMAT
    ));
}

#[test]
fn logging_string() {
    let log = Logger::new("logging_string.log", FORMAT);

    assert!(log.log(&String::from("logging_string()")));

    assert!(testenv::check_and_delete(
        "logging_string()",
        "logging_string.log",
        FORMAT
    ));
}

#[test]
fn logging_reverse_format() {
    let log = Logger::new("logging_reverse_format.log", FORMAT);

    assert!(log.log("logging_reverse_format()"));

    assert!(testenv::check_and_delete(
        "logging_reverse_format()",
        "logging_reverse_format.log",
        FORMAT
    ));
}

#[test]
fn bad_file_name() {
    let log = Logger::new("", FORMAT);

    assert!(!log.log("bad_file_name()"));
}

#[test]
fn empty_format() {
    let log = Logger::new("empty_format.log", "");

    assert!(log.log("empty_format()"));

    assert!(testenv::check_and_delete(
        "empty_format()",
        "empty_format.log",
        "%d-%m-%Y %a %H:%M.%S"
    ));
}

#[test]
fn custom_format() {
    let log = Logger::new("custom_format.log", "%Y %H");

    assert!(log.log("custom_format()"));

    assert!(testenv::check_and_delete(
        "custom_format()",
        "custom_format.log",
        "%Y %H"
    ));
}

#[test]
fn test_get_fmt() {
    let log = Logger::new("get_fmt.log", FORMAT);

    assert_eq!(FORMAT, log.get_fmt());
}

#[test]
fn test_get_path() {
    let log = Logger::new("get_path.log", FORMAT);

    assert_eq!("get_path.log", log.get_path());
}
