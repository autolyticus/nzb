//
// nzb.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//

pub static mut TOKEN: &str = "";

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

fn add_tasks_to_table(table: &mut prettytable::Table, tasks: &Vec<&Task>) {
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

fn print_tasks_grouped(tasks: &Vec<Task>) {
    let mut projects = std::collections::BTreeMap::new();
    for task in tasks {
        projects
            .entry(task.project.to_uppercase())
            .or_insert_with(Vec::new)
            .push(task);
    }

    let mut table = prettytable::Table::new();
    for (project, tasks) in projects.into_iter() {
        table.add_row(row![format!("{} ({})", project, tasks.iter().len())]);
        add_tasks_to_table(&mut table, &tasks);
        table.add_row(row![]);
    }
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
    let inbox = get_tasks()?
        .into_iter()
        .filter(|x| x.project == "Inbox")
        .collect();
    print_tasks_grouped(&inbox);
    Ok(())
}

pub fn print_next() -> Result<(), Box<std::error::Error>> {
    let next = get_tasks()?.into_iter().filter(|x| x.next).collect();
    print_tasks_grouped(&next);
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
