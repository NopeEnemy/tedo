use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

use std::{
    fmt::Display,
    fs::{self, File, create_dir_all},
    io::{Error, Read},
};

#[derive(Debug, Serialize, Clone, Deserialize)]
struct ToDo {
    field: String,
    complete: bool,
}

impl ToDo {
    fn new(s: &str) -> ToDo {
        Self {
            field: s.to_string(),
            complete: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Profile {
    name: String,
    content: Vec<ToDo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Save {
    current_profile: usize,
    profiles: Vec<Profile>,
}

impl Display for Save {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.profiles[self.current_profile].fmt(f)
    }
}

impl Save {
    pub fn add(&mut self, text: &str) {
        self.profiles[self.current_profile]
            .content
            .push(ToDo::new(text));
    }

    pub fn load() -> Result<Save, Error> {
        let dir = format!(
            "{}/.local/share/tedo",
            home_dir()
                .expect("Can not find home directory:")
                .as_path()
                .to_str()
                .expect("Can not parse path to home directory")
        );

        let path = format!("{dir}/save.json");

        let file = File::open(&path);

        let mut file = match file {
            Ok(f) => f,
            Err(_) => {
                create_dir_all(dir)?;
                File::create(&path)?;
                File::open(&path)?
            }
        };

        let mut save = String::new();
        file.read_to_string(&mut save)?;

        let save: Save = match from_str(&save) {
            Ok(p) => p,
            Err(_) => Save {
                current_profile: 0,
                profiles: vec![Profile::new("Tasks")],
            },
        };

        Ok(save)
    }

    pub fn save(&self) -> Result<(), Error> {
        let str = to_string(self)?;

        let file = format!(
            "{}/.local/share/tedo/save.json",
            home_dir()
                .expect("Can not find home directory:")
                .as_path()
                .to_str()
                .expect("Can not parse path to home directory")
        );

        fs::write(file, str)?;

        Ok(())
    }

    fn find_profile_num(&self, profile_name: &str) -> Option<usize> {
        let mut i = 0;

        for p in &self.profiles {
            if p.name == profile_name {
                return Some(i);
            }
            i += 1;
        }

        None
    }

    pub fn remove_profile(&mut self, profile_name: &str) {
        let mut i = 0;

        if self.profiles.len() < 2 {
            return;
        }

        for p in &self.profiles {
            if p.name == profile_name {
                break;
            }
            i += 1;
        }

        self.profiles.remove(i);

        if self.profiles.len() >= self.current_profile {
            self.current_profile -= 1;
        }
    }

    pub fn set_profile(&mut self, profile_name: &str) {
        if let Some(n) = self.find_profile_num(profile_name) {
            self.current_profile = n;
        } else {
            let profile = Profile::new(profile_name);
            self.profiles.push(profile);

            if let Some(n) = self.find_profile_num(profile_name) {
                self.current_profile = n;
            }
        }
    }

    pub fn remove(&mut self, i: usize) {
        match self.profiles[self.current_profile].content.get(i) {
            Some(_) => (),
            None => return,
        }

        self.profiles[self.current_profile].content.remove(i);
    }

    pub fn remove_completed(&mut self) {
        self.profiles[self.current_profile]
            .content
            .retain(|x| !x.complete);
    }

    pub fn complete(&mut self, i: usize) {
        let target = match self.profiles[self.current_profile].content.get_mut(i) {
            Some(t) => t,
            None => return,
        };

        target.complete = !target.complete;
    }

    pub fn list_profiles(&self) {
        println!("There are {} profiles:", self.profiles.len());
        for i in &self.profiles {
            println!("{}: {} tasks", i.name, i.content.len());
        }
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = format!("Profile '{}':\n\n", self.name);
        let mut num = 0;

        for i in &self.content {
            let buf = if i.complete {
                "[ X ]".to_string()
            } else {
                "[   ]".to_string()
            };

            let buf = format!("{num}. {buf} {}\n", i.field);

            str.push_str(&buf);

            num += 1;
        }

        write!(f, "{str}")
    }
}

impl Profile {
    fn new(profile_name: &str) -> Self {
        Self {
            name: profile_name.to_string(),
            content: vec![],
        }
    }
}
