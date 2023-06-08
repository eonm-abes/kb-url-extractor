use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Number of parallel workers
    #[arg(short, long, default_value_t = 5)]
    pub worker: usize,
    /// Output file
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    /// Config file
    #[arg(short, long)]
    pub config: PathBuf,
}
