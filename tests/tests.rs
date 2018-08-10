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
const FORMAT: &str = "$date $time $msg";

#[test]
fn instantiate() {
    let log = Logger::new(TESTLOG, FORMAT);
    assert_eq!(log.format, FORMAT);
    assert_eq!(log.path, TESTLOG);
}

#[test]
fn logging_str() {
    let log = Logger::new("logging_str.log", FORMAT);

    assert!(log.log("logging_str()"));

    assert!(testenv::check_and_delete(
        "logging_str()",
        "logging_str.log"
    ));
}

#[test]
fn logging_string() {
    let log = Logger::new("logging_string.log", FORMAT);

    assert!(log.log(&String::from("logging_string()")));

    assert!(testenv::check_and_delete(
        "logging_string()",
        "logging_string.log"
    ));
}

#[test]
fn logging_reverse_format() {
    let log = Logger::new("logging_reverse_format.log", "$time $date $msg");

    assert!(log.log("logging_reverse_format()"));

    assert!(testenv::check_and_delete(
        "logging_reverse_format()",
        "logging_reverse_format.log"
    ));
}

#[test]
fn logging_timeshort_format() {
    let log = Logger::new("logging_timeshort_format.log", "$date $timeshort $msg");

    assert!(log.log("logging_timeshort_format()"));

    assert!(testenv::check_and_delete(
        "logging_timeshort_format()",
        "logging_timeshort_format.log"
    ));
}

#[test]
fn logging_time_and_timeshort() {
    let log = Logger::new(
        "logging_time_and_timeshort.log",
        "$date $time $timeshort $msg",
    );

    assert!(log.log("logging_time_and_timeshort()"));

    assert!(testenv::check_and_delete(
        "logging_time_and_timeshort()",
        "logging_time_and_timeshort.log"
    ));
}

#[test]
fn bad_file_name() {
    let log = Logger::new("", "$time $date $msg");

    assert!(!log.log("bad_file_name()"));
}

#[test]
fn bad_format() {
    let log = Logger::new("bad_format.log", "");

    assert!(log.log("bad_file_name()"));

    assert!(testenv::check_and_delete(
        "bad_file_name()",
        "bad_format.log"
    ));
}
