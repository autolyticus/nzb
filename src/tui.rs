//
// ui.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//
use super::nzb::Task;
pub fn picker(
    tasks: Vec<Task>,
    args: Vec<String>,
    prompt: &str,
) -> Result<(Vec<Task>, Vec<usize>), Box<std::error::Error>> {
    let query = args.join("");
    let options = skim::SkimOptions::default()
        .height("10%")
        .multi(true)
        .prompt(prompt)
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

pub fn login() -> Result<(String, String), Box<std::error::Error>> {
    use std::io::prelude::*;
    println!("\nPlease login to Nozbe: ");
    print!("Nozbe Username/Email: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username)?;
    let password = rpassword::prompt_password_stdout("Password: ").expect("Error reading password");
    Ok((username, password))
}
