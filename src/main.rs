//
// main.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//

#![feature(type_ascription)]
extern crate dirs;
extern crate docopt;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prettytable;

mod args;
mod nzb;

pub fn main() -> Result<(), Box<std::error::Error>> {
    let a = args::parse_args();
    // println!("{:?}", a);
    if let Some(s) = a.flag_auth {
        unsafe {
            nzb::TOKEN = Box::leak(Box::new(s));
        }
    }
    if let Some(x) = a.arg_command {
        match x {
            args::Command::All => nzb::print_all()?,
            args::Command::Conky => unimplemented!(),
            args::Command::Inbox => nzb::print_inbox()?,
            args::Command::Next => unimplemented!(),
        }
    } else {
        // Default action = Print all
        nzb::print_all()?;
        // unimplemented!();
    }
    Ok(())
}
