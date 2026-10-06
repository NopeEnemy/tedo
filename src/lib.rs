use std::{
    fmt::Display,
    fs::{self, File, create_dir_all},
    io::{Error, Read},
};

use clap::Parser;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug, Parser)]
#[command(about, version, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "Add a task", value_name = "TASK")]
    add: Option<String>,

    #[arg(short, long, help = "List all tasks")]
    list: bool,

    #[arg(
        short,
        long,
        help = "Remove a task (requires a task number)",
        value_name = "TASK"
    )]
    remove: Option<usize>,

    #[arg(long, help = "Remove all completed tasks")]
    remove_completed: bool,

    #[arg(
        short,
        long,
        help = "Complete a task (requires a task number)",
        value_name = "TASK"
    )]
    complete: Option<usize>,

    #[arg(
        short,
        long,
        help = "Set or create a profile (required a profile name)",
        value_name = "PROFILE"
    )]
    set_profile: Option<String>,

    #[arg(
        long,
        help = "Remove a profile profile (required a profile name)",
        value_name = "PROFILE"
    )]
    remove_profile: Option<String>,

    #[arg(long, short, help = "List all available profiles")]
    profiles: bool,
}

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
struct Save {
    current_profile: usize,
    profiles: Vec<Profile>,
}

impl Display for Save {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.profiles[self.current_profile].fmt(f)
    }
}

impl Save {
    fn add(&mut self, text: &str) {
        self.profiles[self.current_profile]
            .content
            .push(ToDo::new(text));
    }

    fn load() -> Result<Save, Error> {
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

    fn save(&self) -> Result<(), Error> {
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

    fn remove_profile(&mut self, profile_name: &str) {
        let mut i = 0;

        if self.profiles.len() > 2 {
            return;
        }

        for p in &self.profiles {
            if p.name == profile_name {
                break;
            }
            i += 1;
        }

        self.profiles.remove(i);

        if self.profiles.len() - 1 > self.current_profile {
            self.current_profile -= 1;
        }
    }

    fn set_profile(&mut self, profile_name: &str) {
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

    fn remove(&mut self, i: usize) {
        match self.profiles[self.current_profile].content.get(i) {
            Some(_) => (),
            None => return,
        }

        self.profiles[self.current_profile].content.remove(i);
    }

    fn remove_completed(&mut self) {
        self.profiles[self.current_profile]
            .content
            .retain(|x| !x.complete);
    }

    fn complete(&mut self, i: usize) {
        let target = match self.profiles[self.current_profile].content.get_mut(i) {
            Some(t) => t,
            None => return,
        };

        target.complete = !target.complete;
    }

    fn list_profiles(&self) {
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

pub fn run(args: Args) {
    let save = Save::load();

    let mut command = false;

    let mut save = match save {
        Ok(s) => s,
        Err(e) => panic!("Error: {e}"),
    };

    match args.add {
        Some(s) => {
            command = true;
            save.add(&s);
        }
        None => (),
    }

    if args.list {
        println!("{save}");
        command = true;
    }

    match args.remove {
        Some(i) => {
            command = true;
            save.remove(i)
        }
        None => (),
    }

    if args.remove_completed {
        save.remove_completed();
        command = true;
    }

    match args.complete {
        Some(i) => {
            command = true;
            save.complete(i)
        }
        None => (),
    }

    if args.profiles {
        command = true;
        save.list_profiles();
    }

    match args.set_profile {
        Some(p) => {
            save.set_profile(&p);
            command = true
        }
        None => (),
    }

    match args.remove_profile {
        Some(p) => {
            save.remove_profile(&p);
            command = true;
        }
        None => (),
    }

    if !command {
        println!("{save}");
    }

    match save.save() {
        Ok(_) => (),
        Err(e) => println!("There is an error by saving: {e}"),
    };
}
