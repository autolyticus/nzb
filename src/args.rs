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
  -a <token> --auth=<token>    Specify an alternate Nozbe authentication token (Refer Nozbe API Documentation)
                               (Note: The default authentication token is at $HOME/.local/.nozbe_token)
  -h --help                    Show this screen
  -V --version                 Show version

Commands:
  add <name>                   Add a task to your Nozbe Inbox
  all                          View all of your tasks (This is the default action)
  cat <category>               View all tasks in a category
  conky                        A conky-friendly, colourful summary of all your tasks
  done                         Mark task(s) as done with fuzzy search
  help                         Show this screen
  inbox                        View your inbox
  link <link>                  Add a link to your inbox (adds a comment with link)
  list [<list>...]             Show specific lists (projects)
  login                        Login to Nozbe
  mv                           Move tasks between projects
  now | priority | starred     View starred tasks
  open                         Open Nozbe in your browser
  overdue                      View tasks that are overdue
  star                         Star task(s) with fuzzy search
  today                        View tasks that are due today
  unstar                       Unstar task(s) with fuzzy search
";

#[derive(Debug, Deserialize)]
pub struct Args {
    pub arg_command: Option<Command>,
    pub arg_args: Vec<String>,
    pub flag_auth: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum Command {
    Add,
    All,
    Cat,
    Conky,
    Debug,
    Done,
    Help,
    Inbox,
    Link,
    List,
    Login,
    Mv,
    Now,
    Open,
    Overdue,
    Priority,
    Star,
    Starred,
    Today,
    Unstar,
}

pub fn parse_args() -> Args {
    Docopt::new(USAGE)
        .unwrap_or_else(|e| e.exit())
        .options_first(true)
        .version(Some("Nzb: A Nozbe client (v0.4.3)".to_owned()))
        .deserialize()
        .unwrap_or_else(|e| e.exit())
}

pub fn print_help() {
    print!("{}", USAGE);
}
