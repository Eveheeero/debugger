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

    /// Thread Available
    #[arg(short = 'T', long, default_value_t = false)]
    thread_available: bool,

    /// Argument From
    #[arg(short = 'f', long, value_name = "from", default_value_t = 0)]
    argument_from: isize,

    /// Argument To
    #[arg(short = 't', long, value_name = "to", default_value_t = 100)]
    argument_to: isize,
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
            target_builder.arg(
                random_generator
                    .gen_range(args.argument_from..=args.argument_to)
                    .to_string(),
            );
        }
    }

    return;
}
