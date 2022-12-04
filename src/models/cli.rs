use std::path::PathBuf;

use clap::Parser;

/// Obtains information and download links for Native Instruments products
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Set the directory paths to search for Cubase projects.
    #[clap(name = "project-path", short, long, value_parser, value_name = "PATH")]
    pub project_paths: Vec<String>,

    /// Set the config file path
    #[clap(short, long, value_parser, value_name = "PATH")]
    pub config: Option<PathBuf>,
}
