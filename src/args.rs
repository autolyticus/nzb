//
// args.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//

use docopt::Docopt;

const USAGE: &'static str = "Nozbe front-end written in Rust.

Usage:
  nzb [options] [<command> [<args>...]]
  nzb -h | --help
  nzb --version

Options:
  -a <token> --auth=<token>    Specify the Nozbe authentication token (Refer Nozbe API Documentation)
                               (Note: The default authentication token is assumed to be at $HOME/.local/.nozbe_token)
  -h --help                    Show this screen
  -V --version                 Show version

Commands:
  all                          View all your tasks (This is the default action)
  conky                        A conky-friendly, colourful summary of all your tasks
  inbox                        View your inbox
  next                         View next(starred) tasks
";

#[derive(Debug, Deserialize)]
pub struct Args {
    pub arg_command: Option<Command>,
    pub arg_args: Vec<String>,
    pub flag_auth: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum Command {
    All,
    Conky,
    Inbox,
    Next,
}

pub fn parse_args() -> Args {
    Docopt::new(USAGE)
        .unwrap_or_else(|e| e.exit())
        .options_first(true)
        .version(Some("Nzb: A Nozbe client (v0.1.1)".to_owned()))
        .deserialize()
        .unwrap_or_else(|e| e.exit())
}
