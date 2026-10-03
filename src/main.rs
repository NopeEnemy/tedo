use clap::Parser;
use tedo::{Args, run};

fn main() {
    let args = Args::parse();

    run(args);
}
