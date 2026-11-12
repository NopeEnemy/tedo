[![crates.io](https://img.shields.io/crates/v/tedo.svg)](https://crates.io/crates/tedo)
[![crates.io](https://img.shields.io/crates/d/tedo.svg)](https://crates.io/crates/tedo)

# TEDO
A simple terminal-based task manager written in Rust for people who prefer the command line over GUIs.


## Features
- Add, complete and remove tasks
- Multiple profiles
- JSON storage
- TOML config file
- Lightweight CLI
- Task expire and recurrence tasks


## Installation
WARNING: I don't test Windows builds, if you found there a bug, please make an issue.

### Prebuilt binary
1. Go to the latest [release](https://github.com/NopeEnemy/tedo/releases)
2. Download the binary

### From source
1. Download Rust
2. Clone this repo
3. Run ```cargo build --release```
4. The binary will be available in target/release/

### With Cargo:
1. Run ```cargo install tedo```


## Usage
```tedo -h```        --- Print help

```tedo tasks add <TASK>``` --- Add a task

```tedo```        --- List all tasks

```tedo tasks remove <TASK>``` --- Remove a task (requires a task number)

```tedo tasks remove-completed```        --- Remove all completed tasks

```tedo tasks complete <TASK>``` --- Complete a task (requires a task number)

```tedo profiles set <PROFILE>``` --- Set profile (requires a profile name)

```tedo profiles remove <PROFILE>``` --- Remove a profile (requires a profile name)

```tedo profiles``` --- list all profiles

```tedo tasks clear``` --- Clear current profile

```tedo tasks edit -t <TARGET> <TEXT>``` --- Edit a task (requires a task number)

```tedo tasks add -e 1h <TEXT>``` --- Create a task, that expire in 1 hour

```tedo tasks add -r 1d <TEXT>``` --- Create a task, that resets every day


## Profiles
You can collect tasks into *profiles*. This can be very helpful, if you want to divide tasks. For example work and personal tasks. 


## Storage
The app stores all data in ~/.local/share/tedo/save.json file. Please do not change this file, if you don't know what you're doing. This can break it and lead to a loss all data.


## Configuration
You can configure a few setting in ~/.config/tedo/config.toml or AppData\\Roaming\\tedo\\config.tom file. Yet you can see all configure options [here](https://github.com/NopeEnemy/tedo/wiki/Configuration)

## Logs:
If you have write_tasks_logs = true in your config, Tedo will write logs in ~/.cache/tedo/tedo.log or in AppData\\Local\\tedo\\tedo.log

## Feedback
If you found a bug or have a suggestion you can create an [issue](https://github.com/NopeEnemy/tedo/issues)


## Roadmap
- [X] TO-DO tasks
- [X] Profiles
- [X] Configuration file
- [X] Tasks priorities
- [X] Timer for tasks
- [X] Task logs
- [ ] Undo option
- [ ] Filter settings
- [ ] Tags
- [ ] Search
