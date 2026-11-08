use crate::core::{config::Config, save::Save};
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
            help = "Expire time, for example: 6m (6 month), 1d (1 day), 4h (4 hours), 100s (100 seconds)"
        )]
        reccurence: Option<String>,

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

    save.update_profiles();

    let config = Config::load(args.config);

    let command = match args.command {
        Some(c) => c,
        None => {
            save.list(config.show_importance, config.numerate_from_zero);
            return;
        }
    };

    match command {
        Commands::Tasks { command } => task(&mut save, command, config),
        Commands::Profiles { command } => profile(&mut save, command),
    }

    match save.save() {
        Ok(_) => (),
        Err(e) => println!("There is an error by saving: {e}"),
    };
}

fn task(save: &mut Save, commands: Option<TaskCommands>, config: Config) {
    let commands = match commands {
        Some(c) => c,
        None => {
            save.list(config.show_importance, config.numerate_from_zero);
            return;
        }
    };

    match commands {
        TaskCommands::Add {
            add,
            importance,
            reccurence,
            expire,
        } => save.add(
            &add,
            match importance {
                Some(i) => i,
                None => config.default_importance,
            },
            reccurence,
            expire,
        ),
        TaskCommands::Remove { remove } => save.remove(remove, config.numerate_from_zero),
        TaskCommands::Complete { complete } => save.complete(complete, config.numerate_from_zero),
        TaskCommands::Edit {
            edit,
            target,
            importance,
        } => save.edit(target, edit, importance, config.numerate_from_zero),
        TaskCommands::List => save.list(config.show_importance, config.numerate_from_zero),
        TaskCommands::Clear => save.clear(),
        TaskCommands::RemoveCompleted => save.remove_completed(),
        TaskCommands::CompleteAll => save.complete_all(),
        TaskCommands::DecompleteAll => save.decomplete_all(),
        TaskCommands::ChangeAll => save.change_all(),
    }
}

fn profile(save: &mut Save, commands: Option<ProfileCommands>) {
    let commands = match commands {
        Some(c) => c,
        None => {
            save.list_profiles();
            return;
        }
    };

    match commands {
        ProfileCommands::List => save.list_profiles(),
        ProfileCommands::Remove { remove_profile } => save.remove_profile(&remove_profile),
        ProfileCommands::Set { set_profile } => save.set_profile(&set_profile),
    }
}
