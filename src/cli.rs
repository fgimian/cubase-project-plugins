use std::path::PathBuf;

use clap::{Parser, ValueHint};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Directory paths to search for Cubase projects.
    #[arg(
        value_name = "PROJECT_PATH",
        value_hint = ValueHint::DirPath,
        required_unless_present = "completions",
        conflicts_with = "completions"
    )]
    pub project_paths: Vec<String>,

    /// Config file path.
    #[arg(short, long, value_name = "PATH", value_hint = ValueHint::FilePath)]
    pub config_path: Option<PathBuf>,

    /// Filter projects based on a plugin name or GUID using a wildcard pattern.
    #[arg(name = "filter", short, long, value_name = "PATTERN")]
    pub patterns: Vec<String>,

    /// Only show filtered plugins for each project.
    #[arg(short, long)]
    pub only_show_filtered: bool,

    /// Generate shell completions.
    #[arg(long, value_name = "SHELL")]
    pub completions: Option<Shell>,
}

#[must_use]
pub fn default_config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home_dir| home_dir.join(".config").join("cubase-project-plugins.toml"))
}
