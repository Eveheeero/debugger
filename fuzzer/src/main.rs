use std::{path::PathBuf, process::Command};

use clap::Parser;
use rand::Rng;

/// Custom parser
#[derive(Parser)]
struct Args {
    /// File Path
    #[arg(short, long)]
    path: String,

    /// Arguments Count
    #[arg(short = 'c', long, value_name = "count", default_value_t = 1)]
    argument_count: usize,

    /// Loop Count
    #[arg(short = 'l', long, value_name = "count", default_value_t = 100)]
    loop_count: usize,
}

fn main() {
    let args = Args::parse();

    let target_path = PathBuf::from(args.path);
    let argument_count = args.argument_count;
    let loop_count = args.loop_count;

    let mut target_builder = Command::new(target_path);

    let mut random_generator = rand::thread_rng();

    // Run program for specified times
    for _ in 0..loop_count {
        for _ in 0..argument_count {
            target_builder.arg(random_generator.gen::<u64>().to_string());
        }
    }

    return;
}
