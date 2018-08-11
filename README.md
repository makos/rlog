# rlog

[![Build Status](https://travis-ci.org/makos/rlog.svg?branch=master)](https://travis-ci.org/makos/rlog) [![Docs Badge](https://docs.rs/rlog/badge.svg)](https://docs.rs/rlog/) [![crates.io Badge](https://img.shields.io/badge/crates.io-1.0-orange.svg)](https://crates.io/crates/rlog)

Minimal file-based logging library.

# Usage

## Quickstart

`Cargo.toml`
````
[dependencies]
"rlog" = "1"
````

`src/main.rs`
````
extern crate rlog;
use rlog::Logger;

let log = Logger::new("./test.log", "");
log.log("Dear diary, today I wrote some Rust code!");
````

Output:
`test.log`
````
10.08.2018 09:47.12 Dear diary, today I wrote some Rust code!
````

## Available options

When instantiating a new logger instance, you can set the logfile `path` and desired log `format`.
`path` is a relative or absolute path to your log file, and `format` is an ISO8061-style timestamp (e.g. `%d-%m-%y %H:%M`).

# License

Licensed under GPL 3.0. See LICENSE for details.

&copy; 2018 by Mateusz Makowski <matmakos@gmail.com>