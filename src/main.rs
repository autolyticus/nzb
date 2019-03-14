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
mod disp;
mod done;
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
            args::Command::All => disp::print_all()?,
            args::Command::Conky => disp::print_conky()?,
            args::Command::Done => done::done(a.arg_args)?,
            args::Command::Help => args::print_help(),
            args::Command::Inbox => disp::print_inbox()?,
            args::Command::List => {
                if a.arg_args.is_empty() {
                    disp::print_all()?
                } else {
                    disp::print_lists(a.arg_args)?
                }
            }
            args::Command::Now | args::Command::Priority | args::Command::Starred => {
                disp::print_now()?
            }
        }
    } else {
        // Default action = Print all
        disp::print_all()?;
        // unimplemented!();
    }
    Ok(())
}
