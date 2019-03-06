//
// args.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//

use docopt::Docopt;

const USAGE: &'static str = "
Nozbe front-end written in Rust.

Usage:
  nzb inbox
  nzb all
  nzb next
  nzb (-h | --help)
  nzb --version

Options:
  -h --help     Show this screen.
  --version     Show version.

Commands:
  inbox View your inbox
  all View all your tasks

";

#[derive(Debug, Deserialize)]
pub struct Args {
    cmd_inbox: bool,
    cmd_all: bool,
    cmd_next: bool,
}

pub fn parse_args() -> Args {
    Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
}
