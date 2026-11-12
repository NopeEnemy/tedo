use crate::core::config::Config;
use crate::core::profiles::{Profile, ToDo};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::fmt::Debug;
use std::process;

use std::{
    fs::{self, File, create_dir_all},
    io::{Error, ErrorKind, Read},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Save {
    current_profile: usize,
    profiles: Vec<Profile>,
}

impl Save {
    // Actions with current profile:

    pub fn get_current_profile(&self) -> String {
        self.profiles[self.current_profile].name.clone()
    }

    pub fn get_task_name_from_id(&self, id: usize) -> String {
        match self.profiles[self.current_profile].content.get(id) {
            Some(s) => s.field.clone(),
            None => "Undefined".to_string(),
        }
    }

    pub fn get_task_from_id(&self, id: usize) -> ToDo {
        match self.profiles[self.current_profile].content.get(id) {
            Some(s) => s.clone(),
            None => ToDo::new("Undefined", 0, None),
        }
    }

    pub fn list(&mut self, config: &Config) {
        self.profiles[self.current_profile].list(config);
    }

    pub fn add(
        &mut self,
        text: &str,
        importance: usize,
        recurrence: Option<String>,
        expire: Option<String>,
    ) {
        self.profiles[self.current_profile].add(text, importance, recurrence, expire);
    }

    pub fn edit(
        &mut self,
        target: usize,
        text: Option<String>,
        importance: Option<usize>,
        expire: Option<String>,
        recurrence: Option<String>,
        numerate_from_zero: bool,
    ) {
        self.profiles[self.current_profile].edit(
            target,
            text,
            importance,
            expire,
            recurrence,
            numerate_from_zero,
        );
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
    pub fn new() -> Self {
        Self {
            current_profile: 0,
            profiles: vec![Profile::new("Tasks")],
        }
    }

    pub fn load() -> Result<Save, Error> {
        let dir = dirs::data_local_dir()
            .expect("Can not get data dir")
            .join("tedo");
        let path = dir.join("save.json");

        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound || e.kind() == ErrorKind::UnexpectedEof {
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
                if e.is_eof() {
                    Save::new()
                } else {
                    eprint!("Cannot parse data, maybe save.json is broken: {e}");
                    process::exit(1);
                }
            }
        };

        Ok(save)
    }

    pub fn save(&self) -> Result<(), Error> {
        let str = to_string(self)?;
        let file = dirs::data_local_dir()
            .expect("Can not get data dir")
            .join("tedo")
            .join("save.json");

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

    pub fn update_profiles(&mut self) {
        for i in self.profiles.iter_mut() {
            i.update();
        }
    }
}
