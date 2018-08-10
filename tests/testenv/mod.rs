use std::fs::{remove_file, File};
use std::io::Read;

extern crate chrono;

pub fn check_and_delete(msg: &str, path: &str) -> bool {
    let mut contents = String::new();

    if let Ok(mut file) = File::open(path) {
        file.read_to_string(&mut contents)
            .expect("check_and_delete(): error reading file");
    }

    match contents.find(msg) {
        Some(i) => {
            if i == 26 {
                remove_file(path).expect("check_and_delete(): error removing file");
                return true;
            } else {
                remove_file(path).expect("check_and_delete(): error removing file");
                return false;
            }
        }
        None => {
            remove_file(path).expect("check_and_delete(): error removing file");
            return false;
        }
    };
}
