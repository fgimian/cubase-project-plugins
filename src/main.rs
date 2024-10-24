mod cli;
mod config;
mod project;
mod reader;

use std::{
    collections::HashMap,
    fs,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    process,
};

use anyhow::{Context, Result};
use clap::Parser as _;
use cli::Cli;
use colored::Colorize as _;
use glob::{MatchOptions, Pattern};

use crate::{config::Config, project::Plugin, reader::Reader};

fn main() {
    if let Err(error) = run() {
        for (index, cause) in error.chain().enumerate() {
            if index == 0 {
                eprintln!("{}: {}", "error".red(), cause);
                continue;
            }

            if index == 1 {
                eprintln!("{}", "caused by:".red());
            }
            println!("    {}: {}", index - 1, cause);
        }
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let config_path = cli.config_path.or_else(|| match get_default_config_path() {
        Some(default_config_path) if default_config_path.is_file() => Some(default_config_path),
        _ => None,
    });

    let config = match config_path {
        Some(config_path) => {
            let config_string = fs::read_to_string(&config_path).with_context(|| {
                format!(
                    "unable to open and read config file '{}'",
                    config_path.display().to_string().blue()
                )
            })?;

            toml::from_str(&config_string).with_context(|| {
                format!(
                    "unable to parse config file '{}'",
                    config_path.display().to_string().blue()
                )
            })?
        }
        None => Config::default(),
    };

    let path_ignore_globs = config
        .path_ignore_patterns
        .iter()
        .map(|p| {
            Pattern::new(p)
                .with_context(|| format!("unable to parse path ignore pattern '{}'", p.blue()))
        })
        .collect::<Result<Vec<_>>>()?;

    let mut plugin_counts = HashMap::new();
    let mut plugin_counts_32 = HashMap::new();
    let mut plugin_counts_64 = HashMap::new();

    let mut project_bytes = Vec::new();

    for project_path in &cli.project_paths {
        let project_path_heading = format!("Path: {project_path}").white().on_red();

        let project_file_path_pattern = Path::new(&project_path).join("**").join("*.cpr");
        let Some(project_file_path_pattern) = project_file_path_pattern.to_str() else {
            println!();
            println!("{project_path_heading}");
            println!();
            println!(
                "{}",
                "Unable to convert the project file pattern to a string".red()
            );
            continue;
        };

        let project_file_paths = match glob::glob_with(
            project_file_path_pattern,
            MatchOptions {
                case_sensitive: false,
                require_literal_separator: false,
                require_literal_leading_dot: false,
            },
        ) {
            Ok(project_file_paths) => project_file_paths,
            Err(error) => {
                println!();
                println!("{project_path_heading}");
                println!();
                println!(
                    "{}",
                    format!("Unable to glob for project files in the project path: {error}").red()
                );
                continue;
            }
        };

        for project_file_path in project_file_paths {
            let project_file_path = match project_file_path {
                Ok(project_file_path) => project_file_path,
                Err(error) => {
                    println!();
                    println!("{project_path_heading}");
                    println!();
                    println!(
                        "{}",
                        format!(
                            "Unable to glob a particular project file in the project path: {error}"
                        )
                        .red()
                    );
                    continue;
                }
            };

            let project_file_path_heading = format!("Path: {}", project_file_path.display())
                .white()
                .on_red();

            let Some(projcet_file_path_str) = project_file_path.to_str() else {
                println!();
                println!("{project_path_heading}");
                println!();
                println!(
                    "{}",
                    "Unable to convert the project file path to a string".red()
                );
                continue;
            };

            if path_ignore_globs
                .iter()
                .any(|glob| glob.matches(projcet_file_path_str))
            {
                continue;
            }

            let mut file = match File::open(&project_file_path) {
                Ok(file) => file,
                Err(error) => {
                    println!();
                    println!("{project_file_path_heading}");
                    println!();
                    println!("{}", format!("Unable to open project file: {error}").red());
                    continue;
                }
            };

            if let Err(error) = file.read_to_end(&mut project_bytes) {
                println!();
                println!("{project_file_path_heading}");
                println!();
                println!("{}", format!("Unable to read project file: {error}").red());
                continue;
            };

            let reader = Reader::new(&project_bytes);
            let project_details = match reader.get_project_details() {
                Ok(project_details) => project_details,
                Err(error) => {
                    project_bytes.clear();
                    println!();
                    println!("{project_file_path_heading}");
                    println!();
                    println!("{}", format!("Unable to parse project file: {error}").red());
                    continue;
                }
            };
            project_bytes.clear();

            let is_64_bit = matches!(
                project_details.metadata.architecture.as_str(),
                "WIN64" | "MAC64 LE"
            );

            if is_64_bit && !config.projects.report_64_bit
                || !is_64_bit && !config.projects.report_32_bit
            {
                continue;
            }

            println!();
            println!("{project_file_path_heading}");
            println!();

            let project_heading = format!(
                "{application} {version} ({architecture})",
                application = project_details.metadata.application,
                version = project_details.metadata.version,
                architecture = project_details.metadata.architecture
            )
            .blue();

            println!("{project_heading}");

            if project_details.plugins.is_empty() {
                continue;
            }

            let mut sorted_plugins = Vec::from_iter(project_details.plugins);
            sorted_plugins.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

            println!();
            for plugin in sorted_plugins
                .iter()
                .filter(|p| !config.plugins.guid_ignores.contains(&p.guid))
                .filter(|p| !config.plugins.name_ignores.contains(&p.name))
            {
                plugin_counts
                    .entry(plugin.clone())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);

                if is_64_bit {
                    plugin_counts_64
                        .entry(plugin.clone())
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                } else {
                    plugin_counts_32
                        .entry(plugin.clone())
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }

                println!("    > {} : {}", plugin.guid, plugin.name);
            }
        }
    }

    print_summary(&plugin_counts_32, "32-bit");
    print_summary(&plugin_counts_64, "64-bit");
    print_summary(&plugin_counts, "All");

    Ok(())
}

fn get_default_config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home_dir| home_dir.join(".config").join("cubase-project-plugins.toml"))
}

fn print_summary(plugin_counts: &HashMap<Plugin, i32>, description: &str) {
    if plugin_counts.is_empty() {
        return;
    }

    let summary_heading = format!("Summary: Plugins Used In {description} Projects")
        .white()
        .on_red();

    println!();
    println!("{summary_heading}");
    println!();

    let mut sorted_plugin_counts = Vec::from_iter(plugin_counts);
    sorted_plugin_counts.sort_by(|a, b| a.0.name.to_lowercase().cmp(&b.0.name.to_lowercase()));

    for (plugin, count) in &sorted_plugin_counts {
        println!("    > {} : {} ({})", plugin.guid, plugin.name, count);
    }
}
