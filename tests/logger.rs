extern crate logger;

use logger::*;

#[test]
fn instantiate() {
    let log = Logger::new("./", "$date $time $msg");
    assert_eq!(log.format, "$date $time $msg");
    assert_eq!(log.path, "./");
}

#[test]
fn logging() {
    let log = Logger::new("./test.log", "$date $time $msg");
    let result = log.log_str("blah blah");
    assert_eq!(Ok(()), result);
}
