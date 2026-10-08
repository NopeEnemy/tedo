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

```tedo -a <TASK>``` --- Add a task

```tedo -l```        --- List all tasks

```tedo -r <TASK>``` --- Remove a task (requires a task number)

```tedo --remove-completed```        --- Remove all completed tasks

```tedo -c <TASK>``` --- Complete a task (requires a task number)

```tedo -s <PROFILE>``` --- Set profile (requires a profile name)

```tedo --remove-profile <PROFILE>``` --- Remove a profile (requires a profile name)

```tedo -p``` --- list all profiles

```tedo --clear``` --- Clear current profile


## Profiles
You can collect tasks into *profiles*. This can be very helpful, if you want to devide tasks. For example work and personal tasks. 


## Storage
The app store all data in ~/.local/share/tedo/save.json file. Please do not change this file, if you don't know what you doing. This can broke it and lead to a loss all data.


## Roadmap
- [X] TO-DO tasks
- [X] Profiles
- [ ] Configuration file
- [ ] Tasks priorities
- [ ] Points from completing tasks
- [ ] Timer for tasks
