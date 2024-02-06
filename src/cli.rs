use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Directory paths to search for Cubase projects.
    #[arg(value_name = "PROJECT_PATH", required = true)]
    pub project_paths: Vec<String>,

    /// Config file path.
    #[arg(short, long, value_name = "PATH")]
    pub config_path: Option<PathBuf>,
}
