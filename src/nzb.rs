//
// nzb.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
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

use std::fs::File;
use std::io::prelude::*;

fn get_auth_token() -> Result<String, Box<std::error::Error>> {
    let mut file = File::open(
        dirs::home_dir()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_owned()
            + "/.local/.nozbe_token",
    )?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // Remove EOL
    contents.pop();
    Ok(contents)
}

fn get_tasks() -> Result<Vec<Task>, Box<std::error::Error>> {
    Ok(reqwest::Client::new()
        .get("https://api.nozbe.com:3000/list")
        .header("Authorization", get_auth_token()?.as_ref(): &str)
        .form(&[("type", "task")])
        .send()?
        .json()?)
}

pub fn get_inbox() -> Result<Vec<Task>, Box<std::error::Error>> {
    Ok(get_tasks()?
        .into_iter()
        .filter(|x| x.project == "Inbox")
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parse_task() -> serde_json::Result<()> {
        let data = r#"
        {
        "_by_user": "j8ML3tmQfiaPiZR7",
        "_comment_count": 1,
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
        "_created_at": "6 Mar 19 18:16",
        "_created_at_org": "2019-03-06 18:16:42",
        "_datedone": "",
        "_datedone_s": "",
        "_datetime_day_s": "29 March",
        "_datetime_full_s": "29 Mar 2019 10:30",
        "_datetime_s": "29 Mar 10:30",
        "_dateweek_s": "29 Mar 10:30",
        "_delegation_list": [],
        "_is_evernote_reminder": "",
        "_name_d": "\tTask name",
        "_project_color": "",
        "_project_name": "Music",
        "_re_account_avatar": "",
        "_re_account_name": "YOU",
        "_recur_name": "Do not repeat",
        "_sort_cal": 0,
        "_sortc": [],
        "_sortn": 0,
        "_sortp": 7,
        "_time_s": "",
        "comment_unread": false,
        "comments": [
            {
                "_body_d": "(+) Sub-task 1\n(-) Sub-task 2",
                "_created_at": "2019-03-06 21:36:27",
                "_created_at_gmt": "2019-03-06T16:06:27+00:00",
                "_created_at_s": "6 Mar 2019 21:36",
                "_updated_at": "2019-03-06 21:36:43",
                "_updated_at_gmt": "2019-03-06T16:06:43+00:00",
                "_updated_at_s": "6 Mar 2019 21:36",
                "_user_name": "johndoe",
                "body": "(+) Sub-task 1\n(-) Sub-task 2",
                "deleted": false,
                "description": null,
                "id": "B4M2MIr3iKEG2Ozl",
                "pinned": false,
                "type": "checklist",
                "user_id": "j8ML3tmQfiaPiZR7"
            }
        ],
        "completed": false,
        "con_list": [],
        "datetime": "2019-03-29 10:30:00",
        "dateweek": null,
        "id": "wmLkkOwiJj4iXp9E",
        "name": "Task name",
        "next": false,
        "project_id": "EM7EdSXaMiPAVnN6",
        "re_user": null,
        "recur": 0,
        "time": "0",
        "ts": "1551888406.996968"
        }
        "#;
        let t: Task = Task {
            name: "Task name".to_owned(),
            _project_name: "Music".to_owned(),
            completed: false,
            next: false,
            _datetime_s: "29 Mar 10:30".to_owned(),
            _con_names: vec!["home".to_owned(), "self".to_owned()],
        };
        let parsed: Task = serde_json::from_str(data)?;
        assert_eq!(t, parsed);

        Ok(())
    }
}
