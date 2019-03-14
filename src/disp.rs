//
// disp.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//
use super::nzb::{get_auth_token, Task};

fn get_tasks() -> Result<Vec<Task>, Box<std::error::Error>> {
    Ok(reqwest::Client::new()
        .get("https://api.nozbe.com:3000/list")
        .header("Authorization", get_auth_token()?.as_ref(): &str)
        .form(&[("type", "task")])
        .send()?
        .json::<Vec<Task>>()
        .expect("Invalid authentication?")
        .into_iter()
        .filter(|x| x.completed == false)
        .collect())
}

fn add_project_to_table(table: &mut prettytable::Table, project: &str, tasks: &[Task]) {
    table.add_row(row![format!("{} ({})", project, tasks.iter().len())]);
    for task in tasks {
        table.add_row(row![
            format!(
                "{} {}",
                task.name,
                if task.due == "not set" {
                    String::new()
                } else {
                    format!("[{}]", task.due)
                }
            ),
            if task.now { "★" } else { "☆" }
        ]);
    }
}

fn add_tasks_grouped(table: &mut prettytable::Table, tasks: &[Task]) {
    let mut projects = std::collections::BTreeMap::new();
    for task in tasks {
        projects
            .entry(task.project.to_uppercase())
            .or_insert_with(Vec::new)
            .push(task.clone());
    }

    for (project, tasks) in projects.into_iter() {
        add_project_to_table(table, &project, &tasks);
        table.add_row(row![]);
    }
}

fn print_tasks_grouped(tasks: &[Task]) {
    let mut table = prettytable::Table::new();
    add_tasks_grouped(&mut table, &tasks);
    table.set_format(
        prettytable::format::FormatBuilder::new()
            .padding(0, 10)
            .build(),
    );
    table.printstd();
}

pub fn print_all() -> Result<(), Box<std::error::Error>> {
    let all = get_tasks()?;
    print_tasks_grouped(&all);
    Ok(())
}

pub fn print_inbox() -> Result<(), Box<std::error::Error>> {
    print_tasks_grouped(
        &get_tasks()?
            .into_iter()
            .filter(|x| x.project == "Inbox")
            .collect::<Vec<_>>(),
    );
    Ok(())
}

pub fn print_now() -> Result<(), Box<std::error::Error>> {
    let now = &get_tasks()?
        .into_iter()
        .filter(|x| x.now)
        .collect::<Vec<_>>();
    print_tasks_grouped(&now);
    Ok(())
}

pub fn print_conky() -> Result<(), Box<std::error::Error>> {
    let all = get_tasks()?;
    let red = "${color red}";
    let hr = "${hr 2}";
    let yellow = "${color yellow}";
    let blue = "${color4}";
    let alignc = "${alignc}";
    let default = "${color7}";
    let mut table = prettytable::Table::new();
    let now: Vec<_> = all.clone().into_iter().filter(|x| x.now).collect();
    let next: Vec<_> = all
        .clone()
        .into_iter()
        .filter(|x| x.project == "2-Next")
        .filter(|x| x.now == false)
        .collect();
    let side: Vec<_> = all.into_iter().filter(|x| x.project == "Side").collect();
    if !now.is_empty() {
        table.add_row(row![format!("{}{}\t\t1-NOW{}", yellow, alignc, hr)]);
        add_tasks_grouped(&mut table, &now);
        table.add_row(row![hr, red]);
    } else {
        table.add_row(row![
            "To have tasks show up here, please STAR them to mark them as NOW tasks",
            red
        ]);
        table.add_empty_row();
    }
    add_project_to_table(&mut table, "SIDE", &side);
    table.add_row(row![blue]);
    add_project_to_table(&mut table, "2-NEXT", &next);
    table.add_row(row![default]);
    table.set_format(
        prettytable::format::FormatBuilder::new()
            .padding(0, 10)
            .build(),
    );
    table.printstd();
    Ok(())
}
