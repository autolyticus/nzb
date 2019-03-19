//
// disp.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//
use super::nzb::{get_tasks, Task};

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

    for (project, tasks) in projects.iter() {
        add_project_to_table(table, project, tasks);
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
            .iter()
            .cloned()
            .filter(|x| x.project == "Inbox")
            .collect::<Vec<_>>(),
    );
    Ok(())
}

pub fn print_lists(lists: Vec<String>) -> Result<(), Box<std::error::Error>> {
    let tasks = get_tasks()?;
    for list in lists {
        print_tasks_grouped(
            &tasks
                .iter()
                .cloned()
                .filter(|x| x.project.eq_ignore_ascii_case(&list))
                .collect::<Vec<_>>(),
        );
    }
    Ok(())
}

pub fn print_now() -> Result<(), Box<std::error::Error>> {
    let now = &get_tasks()?
        .iter()
        .cloned()
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
    let now: Vec<_> = all.iter().cloned().filter(|x| x.now).collect();
    let next: Vec<_> = all
        .iter()
        .cloned()
        .filter(|x| x.project == "2-Next")
        .filter(|x| !x.categories.contains(&"Side".to_owned()))
        .filter(|x| x.now == false)
        .collect();
    let side: Vec<_> = all
        .iter()
        .cloned()
        .filter(|x| x.project == "2-Next")
        .filter(|x| x.categories.contains(&"Side".to_owned()))
        .filter(|x| x.now == false)
        .collect();
    if !now.is_empty() {
        table.add_row(row![format!("{}{}\t\t1-NOW", yellow, alignc)]);
        table.add_row(row![hr]);
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
