//
// nzb.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//

static URL: &str = "https://api.nozbe.com:3000";
pub static mut TOKEN: &str = "";

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Task {
    #[serde(rename = "_con_names")]
    pub categories: Vec<String>,
    pub completed: bool,
    #[serde(rename = "_datetime_s")]
    pub due: String,
    pub id: String,
    pub name: String,
    // Starred Tasks = Chosen for 1-NOW
    #[serde(rename = "next")]
    pub now: bool,
    #[serde(rename = "_project_name")]
    pub project: String,
}

pub fn get_auth_token() -> Result<String, Box<std::error::Error>> {
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

pub fn get_tasks() -> Result<Vec<Task>, Box<std::error::Error>> {
    let tasks = reqwest::Client::new()
        .get(&format!("{}/list", URL))
        .header("Authorization", get_auth_token()?.as_ref(): &str)
        .form(&[("type", "task")])
        .send()?
        .json::<Vec<Task>>();
    if let Ok(x) = tasks {
        Ok(x.iter().filter(|x| x.completed == false).cloned().collect())
    } else {
        Err("Failed parsing JSON response. Invalid authentication?")?
    }
}

pub fn add_task(name: String) -> Result<(), Box<std::error::Error>> {
    if reqwest::Client::new()
        .post(&format!("{}/task", URL))
        .header("Authorization", get_auth_token()?.as_ref(): &str)
        .form(&[("name", name)])
        .send()?
        .status()
        .is_success()
    {
        Ok(())
    } else {
        Err("Status code: Failure. Invalid authentication?")?
    }
}

pub fn star((tasks, indices): (Vec<Task>, Vec<usize>)) -> Result<(), Box<std::error::Error>> {
    if indices.is_empty() {
        return Ok(());
    }
    let processed: Vec<_> = indices
        .iter()
        .map(|&index| {
            json!({
                "id": tasks[index].id,
                "next": true
            })
        })
        .collect();
    reqwest::Client::new()
        .put(&format!("{}/json/task", URL))
        .header("Authorization", get_auth_token()?.as_ref(): &str)
        .json(&processed)
        .send()?;
    Ok(())
}

pub fn mark_done((tasks, indices): (Vec<Task>, Vec<usize>)) -> Result<(), Box<std::error::Error>> {
    if indices.is_empty() {
        return Ok(());
    }
    let processed: Vec<_> = indices
        .iter()
        .map(|&index| {
            json!({
                "id": tasks[index].id,
                "completed": true
            })
        })
        .collect();
    reqwest::Client::new()
        .put(&format!("{}/json/task", URL))
        .header("Authorization", get_auth_token()?.as_ref(): &str)
        .json(&processed)
        .send()?;
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
            now: false,
            due: "29 Mar 10:30".to_owned(),
            categories: vec!["home".to_owned(), "self".to_owned()],
        }];
        let parsed: Vec<Task> = serde_json::from_str(data)?;
        assert_eq!(t, parsed);

        Ok(())
    }
}
