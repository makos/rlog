extern crate chrono;
extern crate rlog;

use std::fs::{remove_file, File};
use std::io::Read;

pub fn check_and_delete(msg: &str, path: &str, fmt: &str) -> bool {
    let mut contents = String::new();
    // Get a current timestamp and append the message to it
    // to emulate logging output.
    let mut msg = msg.to_owned();
    msg = chrono::Local::now().format(fmt).to_string() + " " + &msg;

    if let Ok(mut file) = File::open(path) {
        file.read_to_string(&mut contents)
            .expect("check_and_delete(): error reading file");
    }

    match contents.find(&msg) {
        Some(_) => {
            del_file(path);
            return true;
        }
        None => {
            return false;
        }
    };
}

fn del_file(path: &str) {
    remove_file(path).expect("check_and_delete(): error removing file");
}
