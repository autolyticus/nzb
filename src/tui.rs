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

pub fn project_picker(
    projects: Vec<Project>,
    args: Vec<String>,
    prompt: &str,
) -> Result<(Vec<Project>, Vec<usize>), Box<std::error::Error>> {
    let query = args.join("");
    let options = skim::SkimOptions::default()
        .height("10%")
        .prompt(prompt)
        .reverse(true)
        .query(&query);
    let project_string = projects
        .iter()
        .cloned()
        .map(|x| x.name)
        .collect::<Vec<_>>()
        .join("\n");
    let selected_items = skim::Skim::run_with(
        &options,
        Some(Box::new(std::io::Cursor::new(project_string))),
    )
    .map(|out| out.selected_items)
    .unwrap_or_default();
    Ok((
        projects,
        selected_items
            .iter()
            .cloned()
            .map(|x| x.get_index())
            .collect::<Vec<_>>(),
    ))
}

pub fn mv() -> Result<(), Box<std::error::Error>> {
    let (tx, rx) = std::sync::mpsc::channel::<_>();
    std::thread::spawn(move || {
        tx.send(
            reqwest::Client::new()
                .get(&format!("{}/list", URL))
                .header("Authorization", read_auth_from_file().unwrap().as_str())
                .form(&[("type", "project")])
                .send()
                .expect("Failure connecting")
                .json::<Vec<Project>>()
                .expect("Failure parsing json"),
        )
    });

    let tasks = get_tasks()?;
    let (tasks, task_indices) = task_picker(
        tasks,
        vec![],
        "Select task(s) to move (Multi-select w/ TAB) >> ",
    )?;

    if task_indices.len() == 0 {
        println!("No tasks selected!");
        std::process::exit(0);
    }

    let projects = rx.recv()?;
    let (projects, project_indices) =
        project_picker(projects, vec![], "Select project to move to >> ")?;

    if project_indices.len() == 0 {
        println!("No project selected!");
        std::process::exit(0);
    }
    move_to_project((
        tasks,
        task_indices.clone(),
        &projects[project_indices[0]].id,
    ))?;
    println!(
        "Moved {} task(s) into {:#?}",
        task_indices.len(),
        projects[project_indices[0]].name
    );
    Ok(())
}
