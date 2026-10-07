use clap::Parser;
use logic::Save;
pub mod logic;

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
