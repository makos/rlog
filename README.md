# rlog

[![Build Status](https://travis-ci.org/makos/rlog.svg?branch=master)](https://travis-ci.org/makos/rlog) [![Docs Badge](https://docs.rs/rlog/badge.svg)](https://docs.rs/rlog/) [![crates.io Badge](https://img.shields.io/badge/crates.io-0.1-orange.svg)](https://crates.io/crates/rlog)

Minimal file-based logging library.

# Usage

## Quickstart

`Cargo.toml`
````
[dependencies]
"rlog" = "0.1"
````

`src/main.rs`
````
extern crate rlog;
use rlog::Logger;

let log = Logger::new("./test.log", "$date $time $msg");
log.log("Dear diary, today I wrote some Rust code!");
````

Output:
`test.log`
````
10.08.2018 09:47.12 Dear diary, today I wrote some Rust code!
````

## Available options

When instantiating a new logger instance, you can set the logfile `path` and desired log `format`.
Available format tokens are: `$date $time $timeshort $msg`. Those are pretty self-explanatory. 
`$time` is normal HH:MM.SS format, `$timeshort` is only HH:MM.

# License

Licensed under GPL 3.0. See LICENSE for details.

&copy; 2018 by Mateusz Makowski <matmakos@gmail.com>