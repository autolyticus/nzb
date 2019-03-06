//
// main.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//

#[macro_use]
extern crate serde_derive;
extern crate docopt;

mod args;
mod nzb;

pub fn main() {
    let args = args::parse_args();
    println!("{:?}", args);
}
