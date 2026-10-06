# TEDO
A simple tasks manager written in Rust for people who don't like GUIs and who want to learn Rust.


### Compilling from sources:
1. Download Rust
2. Clone this repo
3. Run ```cargo build --release```
4. Get the binary from target/release


### Usage:
```tedo -h```        --- Print help

```tedo -a <TASK>``` --- Add a task

```tedo -l```        --- List all tasks

```tedo -r <TASK>``` --- Remove a task (requires a task number)

```tedo --remove-completed```        --- Remove all completed tasks

```tedo -c <TASK>``` --- Complete a task (requires a task number)

```tedo -s <PROFILE>``` --- Set profile (requires a profile name)

```tedo --remove-profile <PROFILE>``` --- Remove a profile (requires a profile name)

```tedo -p``` --- list all profiles
