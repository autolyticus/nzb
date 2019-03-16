# Nzb

[![crates.io](https://img.shields.io/crates/v/nzb.svg?style=flat)](https://crates.io/crates/nzb) [![crates.io](https://img.shields.io/crates/d/nzb.svg?style=flat)](https://crates.io/crates/nzb) [![Build Status](https://gitlab.com/reisub0/nzb/badges/master/build.svg)](https://gitlab.com/reisub0/nzb/badges/master/build.svg?style=flat)



A beautiful CLI front-end for Nozbe written in Rust.

The core functionality of the excellent [Wunderline](https://github.com/wayneashleyberry/wunderline) app for Wunderlist is already present.



## Features

- Extremely usable interface

- Takes literally 10 seconds to set up

- Add tasks to your Nozbe inbox in 2 seconds flat

- Mark multiple tasks as done, or star them, with Fuzzy Search. All in a couple of jiffies! Made possible by the [skim library](https://github.com/lotabout/skim)

- Conky integration for printing a nice summary (an Android widget but for your desktop)



![2019-03-16-195651_1366x768_scrot](https://user-images.githubusercontent.com/25099244/54476743-e2a60900-4826-11e9-8085-19a6d6e35d23.png)

## Usage

```
$ nzb help
Nozbe front-end written in Rust.

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
  add                          Add a task to your Nozbe Inbox
  all                          View all of your tasks (This is the default action)
  conky                        A conky-friendly, colourful summary of all your tasks
  done                         Mark task(s) as done with fuzzy search
  help                         Show this screen
  inbox                        View your inbox
  list [<list>...]             Show specific lists
  login                        Login to Nozbe
  now | priority | starred     View starred tasks
  open                         Open Nozbe in your browser
  star                         Star task(s) with fuzzy search
  unstar                       Unstar task(s) with fuzzy search
```


### Adding tasks
Simply run `nzb add <task title>`

[![asciicast](https://asciinema.org/a/234104.svg)](https://asciinema.org/a/234104)

### Starring/Unstarring/Marking tasks as done

Running `nzb <star|unstar|done> [query]` opens a fuzzy search window with all the tasks that match the optional query. Multi-select tasks with Tab and select any one with Enter.

[![asciicast](https://asciinema.org/a/234102.svg)](https://asciinema.org/a/234102)

Click on the link above to see a demo.

### Conky integration

In your `conky.conf`, find the `conky.text` section. add
```
${texecpi 60 nzb conky}
```
where 60 is how often(in seconds) you want the view to be updated.



## Installation

### From Binaries

Binary releases can be found at the [Releases](https://gitlab.com/reisub0/nzb/tags) page.  Select the version number, click the download icon and download `build`. Unzip `artifacts.zip` to obtain your binary at `target/release/nzb`. 

**Note: Currently only `linux-x86_64` is supported; support for other targets is planned once some issues with GitLab CI Cross compilation have been ironed out.**

### From Crates.io

```bash
cargo install nzb
```
### From Source

```bash
git clone https://gitlab.com/reisub0/nzb
cargo install --path nzb
```



## WIP:

1. Get Cross compilation to Windows working
2. Filter tasks by context
3. Add feature for adding links directly to comments + task name



## License

Nzb is licensed under the [MIT License](https://choosealicense.com/licenses/mit/).
