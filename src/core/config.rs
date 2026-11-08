use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use std::{fs::File, io::Read};

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
