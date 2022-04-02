use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub display: Option<String>,

    #[clap(short, long)]
    pub output: Option<PathBuf>,

    #[clap(long)]
    pub framerate: Option<u8>,
}
