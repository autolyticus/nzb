//
// tui.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//
use super::nzb::*;

pub fn task_picker(
    tasks: Vec<Task>,
    args: Vec<String>,
    prompt: &str,
) -> Result<(Vec<Task>, Vec<usize>), Box<std::error::Error>> {
    println!("Pressing ENTER selects the current task, and also accepts selection");
    let query = args.join("");
    let options = skim::SkimOptions::default()
        .height("10%")
        .multi(true)
        .prompt(prompt)
        .reverse(true)
        .query(&query);
    let task_string = tasks
        .iter()
        .cloned()
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
            .iter()
            .cloned()
            .map(|x| x.get_index())
            .collect::<Vec<_>>(),
    ))
}
