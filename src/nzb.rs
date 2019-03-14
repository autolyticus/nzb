//
// nzb.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//

pub static mut TOKEN: &str = "";

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct Task {
    name: String,
    completed: bool,
    next: bool,
    #[serde(rename = "_project_name")]
    project: String,

    #[serde(rename = "_datetime_s")]
    due: String,

    #[serde(rename = "_con_names")]
    categories: Vec<String>,
}

fn get_auth_token() -> Result<String, Box<std::error::Error>> {
    use std::fs::File;
    use std::io::prelude::*;
    unsafe {
        if TOKEN != "" {
            return Ok(TOKEN.to_owned());
        }
    }
    let mut file = File::open(
        dirs::home_dir()
            .unwrap()
            .to_str()
            .unwrap_or_default()
            .to_owned()
            + "/.local/.nozbe_token",
    )?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // Remove EOL (\n)
    contents.pop();
    Ok(contents)
}

fn get_tasks() -> Result<Vec<Task>, Box<std::error::Error>> {
    Ok(reqwest::Client::new()
        .get("https://api.nozbe.com:3000/list")
        .header("Authorization", get_auth_token()?.as_ref(): &str)
        .form(&[("type", "task")])
        .send()?
        .json()
        .expect("Invalid authentication?"))
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
            if task.next { "★" } else { "☆" }
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

pub fn print_next() -> Result<(), Box<std::error::Error>> {
    let next = &get_tasks()?
        .into_iter()
        .filter(|x| x.next)
        .collect::<Vec<_>>();
    print_tasks_grouped(&next);
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
    let mut projects = std::collections::BTreeMap::new();
    for task in all.iter() {
        projects
            .entry(task.project.to_uppercase())
            .or_insert_with(Vec::new)
            .push(task.clone());
    }
    let mut table = prettytable::Table::new();
    let now: Vec<_> = all.into_iter().filter(|x| x.next).collect();
    let side: Vec<_> = projects.get("SIDE").unwrap_or(&Vec::new()).to_vec();
    let next: Vec<_> = projects.get("2-NEXT").unwrap_or(&Vec::new()).to_vec();
    table.add_row(row![format!("{}{}\t\t\t1-NOW", yellow, alignc)]);
    table.add_row(row![hr]);
    add_tasks_grouped(&mut table, &now);
    table.add_row(row![hr.to_owned() + red]);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parse_task() -> serde_json::Result<()> {
        let data = r#"
        [
        {
        "_con_icons": [
            48,
            48
        ],
        "_con_icons_char": [
            "F",
            "F"
        ],
        "_con_icons_color": [
            "",
            ""
        ],
        "_con_names": [
            "home",
            "self"
        ],
        "_datetime_full_s": "29 Mar 2019 10:30",
        "_datetime_s": "29 Mar 10:30",
        "_project_name": "Music",
        "_recur_name": "Do not repeat",
        "completed": false,
        "datetime": "2019-03-29 10:30:00",
        "dateweek": null,
        "id": "wmLkkOwiJj4iXp9E",
        "name": "Task name",
        "next": false,
        "recur": 0,
        "time": "0",
        "ts": "1551888406.996968"
        }
        ]
        "#;
        let t: Vec<Task> = vec![Task {
            name: "Task name".to_owned(),
            project: "Music".to_owned(),
            completed: false,
            next: false,
            due: "29 Mar 10:30".to_owned(),
            categories: vec!["home".to_owned(), "self".to_owned()],
        }];
        let parsed: Vec<Task> = serde_json::from_str(data)?;
        assert_eq!(t, parsed);

        Ok(())
    }
}
