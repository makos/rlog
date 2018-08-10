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

use rlog::Logger;
use std::fs::remove_file;

const TESTLOG: &str = "test.log";
const FORMAT: &str = "$date $timeshort $msg";

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
    remove_file("logging_str.log").expect("delete_leftover(): error removing file");
}

#[test]
fn logging_string() {
    let log = Logger::new("logging_string.log", FORMAT);

    assert!(log.log(&String::from("logging_string()")));
    remove_file("logging_string.log").expect("delete_leftover(): error removing file");
}

#[test]
fn logging_reverse_format() {
    let log = Logger::new("logging_reverse_format.log", "$timeshort $date $msg");

    assert!(log.log("logging_reverse_format()"));
    remove_file("logging_reverse_format.log").expect("delete_leftover(): error removing file");
}
