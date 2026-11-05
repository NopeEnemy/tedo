use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::process;

use std::{
    fmt::Display,
    fs::{self, File, create_dir_all},
    io::{Error, ErrorKind, Read},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Importance {
    VeryHigh,
    High,
    Middle,
    Low,
    Zero,
}

impl Importance {
    pub fn new(importance: usize) -> Self {
        match importance {
            0 => Importance::Zero,
            1 => Importance::Low,
            2 => Importance::Middle,
            3 => Importance::High,
            _ => Importance::VeryHigh,
        }
    }
}

impl Display for Importance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text;

        match self {
            Importance::VeryHigh => text = "A",
            Importance::High => text = "B",
            Importance::Middle => text = "C",
            Importance::Low => text = "D",
            Importance::Zero => text = "E",
        }

        write!(f, "{text}")
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
struct ToDo {
    field: String,
    complete: bool,
    importance: Importance,
}

impl ToDo {
    fn new(s: &str, importance: usize) -> ToDo {
        Self {
            field: s.to_string(),
            complete: false,
            importance: Importance::new(importance),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Profile {
    name: String,
    content: Vec<ToDo>,
}

impl Profile {
    fn list(&self, show_importance: bool, numerate_from_zero: bool) {
        let mut str = format!("Profile '{}':\n\n", self.name);
        let mut num = if numerate_from_zero { 0 } else { 1 };

        for i in &self.content {
            let buf = if i.complete {
                "[ X ]".to_string()
            } else {
                "[   ]".to_string()
            };

            let buf = if show_importance {
                format!("{num}. {} {buf} {}\n", i.importance, i.field)
            } else {
                format!("{num}. {buf} {}\n", i.field)
            };

            str.push_str(&buf);

            num += 1;
        }

        println!("{str}");
    }

    pub fn add(&mut self, text: &str, importance: usize) {
        self.content.push(ToDo::new(text, importance));
    }

    pub fn edit(
        &mut self,
        target: usize,
        text: Option<String>,
        importance: Option<usize>,
        numerate_from_zero: bool,
    ) {
        let target = if numerate_from_zero {
            target
        } else {
            target - 1
        };

        match self.content.get_mut(target) {
            Some(t) => {
                if let Some(nt) = text {
                    t.field = nt;
                }

                if let Some(i) = importance {
                    t.importance = Importance::new(i);
                }
            }
            None => eprintln!("Can not find a task with this number"),
        }
    }

    pub fn importance(&mut self, target: usize, importance: usize, numerate_from_zero: bool) {
        let target = if numerate_from_zero {
            target
        } else {
            target - 1
        };

        let target = match self.content.get_mut(target) {
            Some(t) => t,
            None => return,
        };

        let importance = Importance::new(importance);

        target.importance = importance;
    }

    pub fn remove(&mut self, target: usize, numerate_from_zero: bool) {
        let target = if numerate_from_zero {
            target
        } else {
            target - 1
        };

        match self.content.get(target) {
            Some(_) => (),
            None => eprintln!("Can not find a task with this number"),
        }

        self.content.remove(target);
    }

    pub fn remove_completed(&mut self) {
        self.content.retain(|x| !x.complete);
    }

    pub fn complete(&mut self, target: usize, numerate_from_zero: bool) {
        let target = if numerate_from_zero {
            target
        } else {
            target - 1
        };

        let target = match self.content.get_mut(target) {
            Some(t) => t,
            None => {
                eprintln!("Can not find a task with this number");
                return;
            }
        };

        target.complete = !target.complete;
    }

    pub fn clear(&mut self) {
        self.content = vec![];
    }

    pub fn complete_all(&mut self) {
        self.content.iter_mut().for_each(|t| t.complete = true);
    }

    pub fn decomplete_all(&mut self) {
        self.content.iter_mut().for_each(|t| t.complete = false);
    }

    pub fn change_all(&mut self) {
        self.content
            .iter_mut()
            .for_each(|t| t.complete = !t.complete);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Save {
    current_profile: usize,
    profiles: Vec<Profile>,
}

impl Save {
    // Actions with current profile:

    pub fn list(&self, show_importance: bool, numerate_from_zero: bool) {
        self.profiles[self.current_profile].list(show_importance, numerate_from_zero);
    }

    pub fn add(&mut self, text: &str, importance: usize) {
        self.profiles[self.current_profile].add(text, importance);
    }

    pub fn edit(
        &mut self,
        target: usize,
        text: Option<String>,
        importance: Option<usize>,
        numerate_from_zero: bool,
    ) {
        self.profiles[self.current_profile].edit(target, text, importance, numerate_from_zero);
    }

    pub fn importance(&mut self, target: usize, importance: usize, numerate_from_zero: bool) {
        self.profiles[self.current_profile].importance(target, importance, numerate_from_zero);
    }

    pub fn remove(&mut self, i: usize, numerate_from_zero: bool) {
        self.profiles[self.current_profile].remove(i, numerate_from_zero);
    }

    pub fn remove_completed(&mut self) {
        self.profiles[self.current_profile].remove_completed();
    }

    pub fn complete(&mut self, i: usize, numerate_from_zero: bool) {
        self.profiles[self.current_profile].complete(i, numerate_from_zero);
    }

    pub fn clear(&mut self) {
        self.profiles[self.current_profile].clear();
    }

    pub fn complete_all(&mut self) {
        self.profiles[self.current_profile].complete_all();
    }

    pub fn decomplete_all(&mut self) {
        self.profiles[self.current_profile].decomplete_all();
    }

    pub fn change_all(&mut self) {
        self.profiles[self.current_profile].change_all();
    }

    // Load & Save:

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
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    create_dir_all(dir)?;
                    File::create(&path)?;
                    File::open(&path)?
                } else {
                    eprint!("Cannot parse data, maybe save.json is broken: {e}");
                    process::exit(1);
                }
            }
        };

        let mut save = String::new();
        file.read_to_string(&mut save)?;

        let save: Save = match from_str(&save) {
            Ok(p) => p,
            Err(e) => {
                eprint!("Cannot parse data, maybe save.json is broken: {e}");
                process::exit(1);
            }
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

    // Actions with all profiles:

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
        if self.profiles.len() < 2 {
            eprintln!("Can not remove latest profile");
        }

        if let Some(i) = self.find_profile_num(profile_name) {
            if self.current_profile <= i {
                self.current_profile -= 1;
            }
            self.profiles.remove(i);
        } else {
            eprintln!("Can not find a profile with this number")
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

    pub fn list_profiles(&self) {
        println!("There are {} profiles:", self.profiles.len());
        for i in &self.profiles {
            println!("{}: {} tasks", i.name, i.content.len());
        }
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

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    #[serde(default = "config_true")]
    pub show_importance: bool,
    #[serde(default)]
    pub numerate_from_zero: bool,
    #[serde(default = "config_importance")]
    pub default_importance: usize,
}

impl Config {
    pub fn load(path: Option<String>) -> Self {
        let path = match path {
            Some(p) => p,
            None => format!(
                "{}/.config/tedo/config.toml",
                home_dir()
                    .expect("Can not find home dir")
                    .to_str()
                    .expect("Can not parse home dir")
            ),
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
