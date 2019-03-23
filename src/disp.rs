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
        table.add_empty_row();
    }
}

fn print_tasks_grouped(tasks: &[Task]) {
    let mut table = prettytable::Table::new();
    if tasks.is_empty() {
        println!("No tasks match the specified filter");
        return;
    }
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

pub fn print_categories(cats: Vec<String>) -> Result<(), Box<std::error::Error>> {
    let tasks = get_tasks()?;
    let mut table = prettytable::Table::new();
    for cat in cats {
        let cat_tasks = tasks
            .iter()
            .filter(|x| x.categories.iter().any(|x| x.eq_ignore_ascii_case(&cat)))
            .cloned()
            .collect::<Vec<_>>();
        table.add_row(row![format!(
            "{} ({})",
            cat.to_uppercase(),
            cat_tasks.iter().len()
        )]);
        add_tasks_grouped(&mut table, &cat_tasks);
    }
    table.set_format(
        prettytable::format::FormatBuilder::new()
            .padding(0, 10)
            .build(),
    );
    table.printstd();
    Ok(())
}

pub fn print_debug() -> Result<(), Box<std::error::Error>> {
    let all = get_tasks()?;
    for task in all {
        println!("{:#?}", task);
    }
    Ok(())
}

pub fn print_inbox() -> Result<(), Box<std::error::Error>> {
    print_tasks_grouped(
        &get_tasks()?
            .iter()
            .filter(|x| x.project == "Inbox")
            .cloned()
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
                .filter(|x| x.project.eq_ignore_ascii_case(&list))
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    Ok(())
}

pub fn print_now() -> Result<(), Box<std::error::Error>> {
    let now = &get_tasks()?
        .iter()
        .filter(|x| x.now)
        .cloned()
        .collect::<Vec<_>>();
    print_tasks_grouped(&now);
    Ok(())
}

pub fn print_today() -> Result<(), Box<std::error::Error>> {
    let now = &get_tasks()?
        .iter()
        .filter(|x| x.due == "today")
        .cloned()
        .collect::<Vec<_>>();
    print_tasks_grouped(&now);
    Ok(())
}

pub fn print_overdue() -> Result<(), Box<std::error::Error>> {
    use chrono::prelude::*;
    let now = &get_tasks()?
        .iter()
        .filter(|x| {
            let t = x.get_datetime().unwrap_or(Local::now());
            t.date() < Local::today()
        })
        .cloned()
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
    let now: Vec<_> = all
        .iter()
        .filter(|x| x.now)
        .filter(|x| x.categories.iter().all(|x| *x != "Side"))
        .cloned()
        .collect();
    let now_side: Vec<_> = all
        .iter()
        .filter(|x| x.now)
        .filter(|x| x.categories.iter().any(|x| *x == "Side"))
        .cloned()
        .collect();
    let next: Vec<_> = all
        .iter()
        .filter(|x| x.project == "2-Next")
        .filter(|x| x.categories.iter().all(|x| *x != "Side"))
        .filter(|x| x.now == false)
        .cloned()
        .collect();
    let side: Vec<_> = all
        .iter()
        .filter(|x| x.project == "2-Next")
        .filter(|x| x.categories.iter().any(|x| *x == "Side"))
        .filter(|x| x.now == false)
        .cloned()
        .collect();
    if now.is_empty() && now_side.is_empty() {
        table.add_row(row![
            "To have tasks show up here, please STAR them to mark them as NOW tasks",
            red
        ]);
        table.add_empty_row();
    } else {
        table.add_row(row![format!("{}{}\t\t1-NOW", yellow, alignc)]);
        table.add_row(row![hr]);
        add_tasks_grouped(&mut table, &now);
        add_project_to_table(&mut table, "SIDE", &now_side);
        table.add_empty_row();
        table.add_row(row![hr, red]);
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
