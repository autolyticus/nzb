//
// main.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//

#![feature(type_ascription)]
#[macro_use]
extern crate serde_derive;
extern crate dirs;
extern crate docopt;

mod args;
mod nzb;

pub fn main() -> Result<(), Box<std::error::Error>> {
    let args = args::parse_args();
    println!("{:?}", nzb::get_inbox()?);
    Ok(())
}
