# TEDO
A simple tasks manager written in Rust for people how don't like GUIs and how want to learn Rust.


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

```tedo -d```        --- Remove all completed tasks

```tedo -c <TASK>``` --- Complete a task (requires a task number)
