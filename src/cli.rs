use std::path::PathBuf;

use clap::Parser;

/// Displays plugins used in your Cubase projects along with the Cubase version the project was
/// created with.
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

pub fn default_config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home_dir| home_dir.join(".config").join("cubase-project-plugins.toml"))
}
