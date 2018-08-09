extern crate logger;

use logger::*;
use std::fs::File;
use std::io::Read;

const TESTLOG: &str = "test.log";
const FORMAT: &str = "$datetime $msg";

#[test]
fn instantiate() {
    let log = Logger::new(TESTLOG, FORMAT);
    assert_eq!(log.format, FORMAT);
    assert_eq!(log.path, TESTLOG);
}

#[test]
fn logging_str() {
    let log = Logger::new(TESTLOG, FORMAT);
    if let Ok(val) = log.log("logging_str()") {
        assert!(val);
    }
}

#[test]
fn logging_string() {
    let log = Logger::new(TESTLOG, FORMAT);
    if let Ok(val) = log.log_string(String::from("logging_string()")) {
        assert!(val);
    }
}

#[test]
fn logging_reverse_format() {
    let log = Logger::new(TESTLOG, "$msg $datetime");
    if let Ok(val) = log.log_string(String::from("logging_reverse_format()")) {
        assert!(val);
    }
    check_log_file("dupa");
}

fn check_log_file(msg: &str) -> bool {
    let mut contents = String::new();
    if let Ok(mut file) = File::open(TESTLOG) {
        file.read_to_string(&mut contents);
    }
    println!("{}", contents);
    true
}
