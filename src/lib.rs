use crate::core::{
    config::{Config, Logs, TaskEvent},
    profiles::Importance,
    save::Save,
};
use clap::{Parser, Subcommand};
use std::process;

pub mod core {
    pub mod config;
    pub mod profiles;
    pub mod save;
}

#[derive(Debug, Parser)]
#[command(about, version, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, help = "Another path to the config file")]
    config: Option<String>,

    #[arg(long, hide = true)]
    markdown_help: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Actions with tasks")]
    Tasks {
        #[command(subcommand)]
        command: Option<TaskCommands>,
    },

    #[command(about = "Actions with profiles")]
    Profiles {
        #[command(subcommand)]
        command: Option<ProfileCommands>,
    },
}

#[derive(Debug, Subcommand)]
enum ProfileCommands {
    #[command(about = "Set or create a profile")]
    Set {
        #[arg(help = "Profile name", value_name = "PROFILE")]
        set_profile: String,
    },

    #[command(about = "Remove a profile")]
    Remove {
        #[arg(help = "Profile name", value_name = "PROFILE")]
        remove_profile: String,
    },

    #[command(about = "List all profiles")]
    List,
}

#[derive(Debug, Subcommand)]
enum TaskCommands {
    #[command(about = "Add a task")]
    Add {
        #[arg(help = "Task name", value_name = "TASK")]
        add: String,

        #[arg(long, short, help = "Change task importance (requires a task number)")]
        importance: Option<usize>,

        #[arg(
            long,
            short,
            help = "Recurrence time, for example: 6m (6 month), 1d (1 day), 4h (4 hours), 100s (100 seconds)"
        )]
        recurrence: Option<String>,

        #[arg(
            long,
            short,
            help = "Expire time, for example: 6m (6 month), 1d (1 day), 4h (4 hours), 100s (100 seconds)"
        )]
        expire: Option<String>,
    },

    #[command(about = "Edit a task")]
    Edit {
        #[arg(help = "Text to change")]
        edit: Option<String>,

        #[arg(short, long, help = "Target task (requres a task number)")]
        target: usize,

        #[arg(long, short, help = "Change task importance (requires a task number)")]
        importance: Option<usize>,

        #[arg(
            long,
            short,
            help = "Recurrence time, for example: 6m (6 month), 1d (1 day), 4h (4 hours), 100s (100 seconds)"
        )]
        recurrence: Option<String>,

        #[arg(
            long,
            short,
            help = "Expire time, for example: 6m (6 month), 1d (1 day), 4h (4 hours), 100s (100 seconds)"
        )]
        expire: Option<String>,
    },

    #[command(about = "Remove a task")]
    Remove {
        #[arg(help = "Task number", value_name = "TASK")]
        remove: usize,
    },

    #[command(about = "Complete a task")]
    Complete {
        #[arg(help = "Task number", value_name = "TASK")]
        complete: usize,
    },

    #[command(about = "List all tasks in current profile")]
    List,

    #[command(about = "Remove all completed tasks in current profile")]
    RemoveCompleted,

    #[command(about = "Remove all tasks in current profile")]
    Clear,

    #[command(about = "Complete all tasks")]
    CompleteAll,

    #[command(about = "Decomplete all tasks")]
    DecompleteAll,

    #[command(about = "Reverse importance from all tasks")]
    ChangeAll,
}

pub fn run(args: Args) {
    let save = Save::load();

    let mut save = match save {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Can not load your data, error: {e}");
            process::exit(1);
        }
    };

    if args.markdown_help {
        clap_markdown::print_help_markdown::<Args>();
        std::process::exit(0);
    }

    save.update_profiles();

    let config = Config::load(args.config);
    let mut logs = Logs::new();

    let command = match args.command {
        Some(c) => c,
        None => {
            save.list(&config);
            return;
        }
    };

    match command {
        Commands::Tasks { command } => task(&mut save, command, config, &mut logs),
        Commands::Profiles { command } => profile(&mut save, command, config, &mut logs),
    }

    match save.save() {
        Ok(_) => (),
        Err(e) => println!("There is an error by saving: {e}"),
    };

    match logs.write_logs() {
        Ok(_) => (),
        Err(e) => eprintln!("There is an error by saving logs: {e}"),
    }
}

fn task(save: &mut Save, commands: Option<TaskCommands>, config: Config, logs: &mut Logs) {
    let commands = match commands {
        Some(c) => c,
        None => {
            save.list(&config);
            return;
        }
    };

    match commands {
        TaskCommands::Add {
            add,
            importance,
            recurrence,
            expire,
        } => {
            save.add(
                &add,
                match importance {
                    Some(i) => i,
                    None => config.default_importance,
                },
                recurrence,
                expire,
            );
            logs.write_log(
                config.write_task_logs,
                TaskEvent::AddTask {
                    profile: save.get_current_profile(),
                    name: add,
                    importance: Importance::new(importance.unwrap_or(config.default_importance)),
                },
            );
        }
        TaskCommands::Remove { remove } => {
            save.remove(remove, config.numerate_from_zero);
            logs.write_log(
                config.write_task_logs,
                TaskEvent::RemoveTask {
                    profile: save.get_current_profile(),
                    name: save.get_task_name_from_id(remove),
                },
            )
        }
        TaskCommands::Complete { complete } => {
            save.complete(complete, config.numerate_from_zero);
            logs.write_log(
                config.write_task_logs,
                TaskEvent::ChangeTask {
                    profile: save.get_current_profile(),
                    name: save.get_task_name_from_id(complete),
                    complete: save.get_task_from_id(complete).complete,
                },
            );
        }
        TaskCommands::Edit {
            edit,
            target,
            importance,
            expire,
            recurrence,
        } => save.edit(
            target,
            edit,
            importance,
            expire,
            recurrence,
            config.numerate_from_zero,
        ),
        TaskCommands::List => save.list(&config),
        TaskCommands::Clear => {
            save.clear();
            logs.write_log(
                config.write_task_logs,
                TaskEvent::Clear {
                    profile: save.get_current_profile(),
                },
            );
        }
        TaskCommands::RemoveCompleted => {
            save.remove_completed();
            logs.write_log(
                config.write_task_logs,
                TaskEvent::RemoveCompleted {
                    profile: save.get_current_profile(),
                },
            );
        }
        TaskCommands::CompleteAll => {
            save.complete_all();
            logs.write_log(
                config.write_task_logs,
                TaskEvent::CompleteAll {
                    profile: save.get_current_profile(),
                },
            );
        }
        TaskCommands::DecompleteAll => {
            save.decomplete_all();
            logs.write_log(
                config.write_task_logs,
                TaskEvent::DecompleteAll {
                    profile: save.get_current_profile(),
                },
            )
        }
        TaskCommands::ChangeAll => {
            save.change_all();
            logs.write_log(
                config.write_task_logs,
                TaskEvent::ReverseAll {
                    profile: save.get_current_profile(),
                },
            )
        }
    }
}

fn profile(save: &mut Save, commands: Option<ProfileCommands>, config: Config, logs: &mut Logs) {
    let commands = match commands {
        Some(c) => c,
        None => {
            save.list_profiles();
            return;
        }
    };

    match commands {
        ProfileCommands::List => save.list_profiles(),
        ProfileCommands::Remove { remove_profile } => {
            logs.write_log(
                config.write_task_logs,
                TaskEvent::RemoveProfile {
                    profile: remove_profile.clone(),
                },
            );
            save.remove_profile(&remove_profile);
        }
        ProfileCommands::Set { set_profile } => {
            save.set_profile(&set_profile);
            logs.write_log(
                config.write_task_logs,
                TaskEvent::SetProfile {
                    profile: set_profile,
                },
            );
        }
    }
}
