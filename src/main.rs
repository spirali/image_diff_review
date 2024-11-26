mod difference;
mod fs;
mod pair;
mod report;

use crate::difference::compute_differences;
use crate::pair::pairs_from_paths;
use clap::Parser;
use std::path::PathBuf;
use crate::report::create_report;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to "left" images
    left_path: PathBuf,

    /// Path to "right" images
    right_path: PathBuf,

    /// Enable snapshot testing; ignores missing images in left path
    #[arg(long, default_value_t = false)]
    snapshot_testing: bool,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Report,
}

fn process_command(args: Args) -> anyhow::Result<()> {
    let pairs = pairs_from_paths(&args.left_path, &args.right_path)?;
    let diffs = compute_differences(pairs);
    match args.command {
        Command::Report => { create_report(diffs)? }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(e) = process_command(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}