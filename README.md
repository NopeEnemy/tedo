# TEDO
A simple terminal-based task manager written in Rust for people who prefer the command line over GUIs.


## Features
- Add, coplete and remove tasks
- Multiple profiles
- JSON storage
- Lighteight CLI


## Installation

### Prebuilt binary
1. Go to the latest [release](https://github.com/NopeEnemy/tedo/releases)
2. Download bin file

### From source
1. Download Rust
2. Clone this repo
3. Run ```cargo build --release```
4. Get the binary from target/release


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


## Profiles
You can collect tasks into *profiles*. This can be very helpful, if you want to devide tasks. For example work and personal tasks. 


## Storage
The app store all data in ~/.local/share/tedo/save.json file. Please do not change this file, if you don't know what you doing. This can broke it and lead to a loss all data.


## Configuration
You can configure few setting in ~/.config/tedo/config.toml file. Yet you can configure only 2 parameters (I implement more in future): 
show_importance = [true/false] # Show importance, when listing all tasks. Default: false
numerate_from_zero = [true/false] # Numerate from zero. Default: false


## Roadmap
- [X] TO-DO tasks
- [X] Profiles
- [X] Configuration file
- [X] Tasks priorities
- [ ] Timer for tasks
