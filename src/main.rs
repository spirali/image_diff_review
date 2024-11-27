mod difference;
mod fs;
mod pair;
mod report;

use crate::difference::{compute_differences, Difference, ImageInfoResult};
use crate::pair::pairs_from_paths;
use crate::report::{create_report, ReportConfig};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to "left" images
    left_path: PathBuf,

    /// Path to "right" images
    right_path: PathBuf,

    /// Left title
    #[arg(long, default_value = "Left image")]
    left_title: String,

    /// Right title
    #[arg(long, default_value = "Right image")]
    right_title: String,

    /// Ignore left missing files
    #[arg(long, default_value_t = false)]
    ignore_left_missing: bool,

    /// Ignore right missing files
    #[arg(long, default_value_t = false)]
    ignore_right_missing: bool,

    /// Ignore match
    #[arg(long, default_value_t = false)]
    ignore_match: bool,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
struct ReportArgs {
    /// Output filename, default 'report.html'
    #[arg(long, default_value = "report.html")]
    output: PathBuf,
}

#[derive(Parser, Debug)]
enum Command {
    Report(ReportArgs),
}

fn process_command(args: Args) -> anyhow::Result<()> {
    let pairs = pairs_from_paths(&args.left_path, &args.right_path)?;
    let mut diffs = compute_differences(pairs);

    if args.ignore_match {
        diffs.retain(|pair| !matches!(pair.difference, Difference::None));
    }

    if args.ignore_left_missing {
        diffs.retain(|pair| !matches!(pair.left_info, ImageInfoResult::Missing));
    }

    if args.ignore_right_missing {
        diffs.retain(|pair| !matches!(pair.right_info, ImageInfoResult::Missing));
    }

    match args.command {
        Command::Report(opts) => {
            if diffs.is_empty() {
                println!("Nothing to report");
                return Ok(());
            }
            let count = diffs.len();
            let config = ReportConfig {
                left_title: &args.left_title,
                right_title: &args.right_title,
            };
            create_report(&config, diffs, &opts.output)?;
            println!(
                "Report written into '{}'; found {} images",
                opts.output.display(),
                count,
            )
        }
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
