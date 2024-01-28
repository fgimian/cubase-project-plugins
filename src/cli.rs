use std::{path::PathBuf, sync::OnceLock};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Directory paths to search for Cubase projects.
    #[arg(value_name = "PROJECT_PATH", required = true)]
    pub project_paths: Vec<String>,

    /// Config file path.
    #[arg(
        short,
        long,
        value_name = "PATH",
        default_value = default_config_path(),
    )]
    pub config: Option<PathBuf>,
}

fn default_config_path() -> Option<&'static str> {
    dirs::home_dir()
        .map(|home_dir| {
            home_dir
                .join(".config")
                .join("cubase-project-plugins.toml")
                .into_os_string()
        })
        .and_then(|path| path.into_string().ok())
        .map(|path| {
            static DEFAULT_CONFIG_PATH: OnceLock<String> = OnceLock::new();
            let path = DEFAULT_CONFIG_PATH.get_or_init(|| path);
            let path: &'static str = path;
            path
        })
}
