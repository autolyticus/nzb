//
// ui.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//
use super::nzb::{get_tasks, Task};
pub fn picker(
    args: Vec<String>,
    prompt: &str,
) -> Result<(Vec<Task>, Vec<usize>), Box<std::error::Error>> {
    let tasks = get_tasks()?;
    let query = args.join("");
    let options = skim::SkimOptions::default()
        .height("10%")
        .multi(true)
        .prompt(prompt)
        .query(&query);
    // .reverse(true);
    let task_string = tasks
        .clone()
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<_>>()
        .join("\n");
    let selected_items =
        skim::Skim::run_with(&options, Some(Box::new(std::io::Cursor::new(task_string))))
            .map(|out| out.selected_items)
            .unwrap_or_default();
    Ok((
        tasks,
        selected_items
            .into_iter()
            .map(|x| x.get_index())
            .collect::<Vec<_>>(),
    ))
}
