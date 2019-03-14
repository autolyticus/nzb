//
// done.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//

use super::nzb::{get_tasks, mark_done};
use std::io::Cursor;

pub fn done(args: Vec<String>) -> Result<(), Box<std::error::Error>> {
    let query = args.join("");
    let options = skim::SkimOptions::default()
        .height("10%")
        .multi(true)
        .prompt("Select a task to mark as done (Multi-select w/ TAB) >> ")
        .query(&query);
    // .reverse(true);
    let tasks = get_tasks()?;
    let task_string = tasks
        .clone()
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<_>>()
        .join("\n");
    let selected_items = skim::Skim::run_with(&options, Some(Box::new(Cursor::new(task_string))))
        .map(|out| out.selected_items)
        .unwrap_or_default();
    mark_done(
        tasks,
        selected_items
            .into_iter()
            .map(|x| x.get_index())
            .collect::<Vec<_>>(),
    )?;
    Ok(())
}
