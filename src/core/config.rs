use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use std::io::{Error, ErrorKind, Write};
use std::process;
use std::{fs::File, io::Read};

use chrono::Local;

use crate::core::profiles::Importance;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    #[serde(default = "config_true")]
    pub show_importance: bool,
    #[serde(default)]
    pub numerate_from_zero: bool,
    #[serde(default = "config_importance")]
    pub default_importance: usize,
    #[serde(default)]
    pub write_task_logs: bool,
    #[serde(default)]
    pub ignore_recurrence: bool,
    #[serde(default)]
    pub ignore_expire: bool,
}

impl Config {
    pub fn load(path: Option<String>) -> Self {
        let path = match path {
            Some(p) => p,
            None => dirs::preference_dir()
                .expect("Can not find configs dir")
                .join("tedo")
                .join("config.toml")
                .into_string()
                .expect("Can not parse path to the configs directory"),
        };

        let file = File::open(path);

        let mut file = match file {
            Ok(f) => f,
            Err(_) => return Config::default(),
        };

        let mut config = String::new();
        file.read_to_string(&mut config)
            .expect("Can not read config file!");

        match toml::from_str(&config) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Can not parse config:\n{e}\nUsing default config.\n\n");
                Config::default()
            }
        }
    }
}

fn config_true() -> bool {
    true
}

fn config_importance() -> usize {
    2
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskEvent {
    // Tasks
    AddTask {
        profile: String,
        name: String,
        importance: Importance,
    },
    RemoveTask {
        profile: String,
        name: String,
    },
    ChangeTask {
        profile: String,
        name: String,
        complete: bool,
    },

    RemoveCompleted {
        profile: String,
    },

    Clear {
        profile: String,
    },

    CompleteAll {
        profile: String,
    },

    DecompleteAll {
        profile: String,
    },

    ReverseAll {
        profile: String,
    },

    // Profiles:
    SetProfile {
        profile: String,
    },

    RemoveProfile {
        profile: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logs {
    logs: Vec<TaskEvent>,
}

impl Logs {
    pub fn new() -> Self {
        Self { logs: vec![] }
    }
    pub fn write_logs(&self) -> Result<(), Error> {
        if self.logs.is_empty() {
            return Ok(());
        }

        let mut logs = String::with_capacity(50);

        for i in &self.logs {
            match i {
                TaskEvent::AddTask {
                    profile,
                    name,
                    importance,
                } => logs.push_str(&format!(
                    "{} Added task '{}' with importance {} in the {} profile!\n",
                    Local::now().format("[%Y.%-m.%-d %H:%M]"),
                    name,
                    importance,
                    profile
                )),
                TaskEvent::RemoveTask { profile, name } => logs.push_str(&format!(
                    "{} Removed task '{}' from the {} profile!\n",
                    Local::now().format("[%Y.%-m.%-d %H:%M]"),
                    name,
                    profile
                )),
                TaskEvent::ChangeTask {
                    profile,
                    name,
                    complete,
                } => logs.push_str(&format!(
                    "{} Changed task '{}' to the {} state in the {} profile!\n",
                    Local::now().format("[%Y.%-m.%-d %H:%M]"),
                    name,
                    complete,
                    profile
                )),
                TaskEvent::Clear { profile } => logs.push_str(&format!(
                    "{} Cleared the {} profile!\n",
                    Local::now().format("[%Y.%-m.%-d %H:%M]"),
                    profile
                )),
                TaskEvent::CompleteAll { profile } => logs.push_str(&format!(
                    "{} Complete all tasks the {} profile!\n",
                    Local::now().format("[%Y.%-m.%-d %H:%M]"),
                    profile
                )),
                TaskEvent::DecompleteAll { profile } => logs.push_str(&format!(
                    "{} Decomplete all tasks the {} profile!\n",
                    Local::now().format("[%Y.%-m.%-d %H:%M]"),
                    profile
                )),
                TaskEvent::RemoveCompleted { profile } => logs.push_str(&format!(
                    "{} Remove completed all tasks the {} profile!\n",
                    Local::now().format("[%Y.%-m.%-d %H:%M]"),
                    profile
                )),
                TaskEvent::ReverseAll { profile } => logs.push_str(&format!(
                    "{} Reversed all tasks the {} profile!\n",
                    Local::now().format("[%Y.%-m.%-d %H:%M]"),
                    profile
                )),
                TaskEvent::SetProfile { profile } => logs.push_str(&format!(
                    "{} Set to the {} profile!\n",
                    Local::now().format("[%Y.%-m.%-d %H:%M]"),
                    profile
                )),
                TaskEvent::RemoveProfile { profile } => logs.push_str(&format!(
                    "{} Removed the {} profile!\n",
                    Local::now().format("[%Y.%-m.%-d %H:%M]"),
                    profile
                )),
            }
        }

        let dir = match dirs::cache_dir() {
            Some(c) => c.join("tedo"),
            None => {
                eprintln!("Cannot find cache dir");
                process::exit(1)
            }
        };

        let mut file = dir.join("tedo.log");

        let mut text = String::new();

        match File::open(&mut file) {
            Ok(mut f) => f.read_to_string(&mut text)?,
            Err(_) => 0,
        };

        let mut file = match File::create(&file) {
            Ok(o) => o,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    std::fs::create_dir_all(dir)?;
                    File::create(&file)?
                } else {
                    return Err(e);
                }
            }
        };

        file.write(format!("{}{}", text, logs).as_bytes())?;
        Ok(())
    }

    pub fn write_log(&mut self, write_logs: bool, event: TaskEvent) {
        if write_logs {
            self.logs.push(event);
        }
    }
}
