use std::{
    fmt::Display,
    fs::{self, File},
    io::{Error, Read},
};

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug, Parser)]
#[command(about, version, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "Add a task")]
    add: Option<String>,

    #[arg(short, long, help = "List all tasks")]
    list: bool,

    #[arg(short, long, help = "Remove a task (requires a task number)")]
    remove: Option<usize>,

    #[arg(short, long, help = "Remove all completed tasks")]
    delete_completed: bool,

    #[arg(short, long, help = "Complete a task (requires a task number)")]
    complete: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ToDo {
    field: String,
    complete: bool,
}

impl ToDo {
    fn new(s: String) -> ToDo {
        Self {
            field: s,
            complete: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Profiles {
    content: Vec<ToDo>,
}

impl Display for Profiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
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

impl Profiles {
    fn load() -> Result<Profiles, Error> {
        let file = File::open("save.json");

        let mut file = match file {
            Ok(f) => f,
            Err(_) => {
                File::create("save.json")?;
                File::open("save.json")?
            }
        };

        let mut save = String::new();
        file.read_to_string(&mut save)?;

        let save: Profiles = match from_str(&save) {
            Ok(p) => p,
            Err(_) => Profiles { content: vec![] },
        };

        Ok(save)
    }

    fn save(&self) -> Result<(), Error> {
        let str = to_string(self)?;
        fs::write("save.json", str)?;

        Ok(())
    }

    fn remove(&mut self, i: usize) {
        if self.content.capacity() > i {
            self.content.remove(i);
        }
    }

    fn remove_completed(&mut self) {
        let mut n = 0;
        let mut to_remove = Vec::new();

        for i in &self.content {
            if i.complete {
                to_remove.push(n);
            }
            n += 1;
        }

        for i in to_remove {
            self.remove(i);
        }
    }

    fn complete(&mut self, i: usize) {
        let target = match self.content.get_mut(i) {
            Some(t) => t,
            None => return,
        };

        target.complete = true;
    }
}

pub fn run(args: Args) {
    let save = Profiles::load();

    let mut command = false;

    let mut save = match save {
        Ok(s) => s,
        Err(e) => panic!("Error: {e}"),
    };

    match args.add {
        Some(s) => {
            command = true;
            save.content.push(ToDo::new(s));
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

    if args.delete_completed {
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

    if !command {
        println!("{save}");
    }

    match save.save() {
        Ok(_) => (),
        Err(e) => println!("There is an error by saving: {e}"),
    };
}
