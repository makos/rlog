extern crate logger;

use logger::*;
use std::fs::{remove_file, File};
use std::io::Read;

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

    assert!(check_and_delete(
        &log.parse_format("logging_str()"),
        "logging_str.log"
    ));
}

#[test]
fn logging_string() {
    let log = Logger::new("logging_string.log", FORMAT);

    assert!(log.log(&String::from("logging_string()")));

    assert!(check_and_delete(
        &log.parse_format(&String::from("logging_string()")),
        "logging_string.log"
    ));
}

#[test]
fn logging_reverse_format() {
    let log = Logger::new("logging_reverse_format.log", "$timeshort $date $msg");

    assert!(log.log("logging_reverse_format()"));

    assert!(check_and_delete(
        &log.parse_format("logging_reverse_format()"),
        "logging_reverse_format.log"
    ));
}

fn check_and_delete(msg: &str, path: &str) -> bool {
    let mut contents = String::new();

    if let Ok(mut file) = File::open(path) {
        file.read_to_string(&mut contents).unwrap();
    }

    if msg == contents {
        remove_file(path).expect("check_and_delete(): error removing file");
        return true;
    }
    return false;
}
