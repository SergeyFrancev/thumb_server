use std::path::PathBuf;

use clap::{arg, command, Parser};

/// Program launch argument
#[derive(Debug, Default, Parser)]
#[command(
    author,
    about,
    name = "thumb_server",
    disable_version_flag = true,
    version
)]
pub struct Args {
    /// Print the version
    #[arg(long)]
    pub version: bool,
    #[arg(short, long)]
    pub conf: PathBuf,
    // #[arg(short, long, default_value = "")]
    // pub sizes: Vec<String>,
    // The log file or folder to analyze
    // pub file: PathBuf,
}