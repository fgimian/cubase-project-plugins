mod cli;
mod config;
mod project;
mod reader;

use std::{collections::HashMap, fs, fs::File, io::Read, path::Path, process};

use anyhow::{bail, Context, Error, Result};
use clap::Parser as _;
use cli::Cli;
use colored::Colorize as _;
use glob::{MatchOptions, Pattern};

use crate::{config::Config, project::Plugin, reader::Reader};

fn print_error(error: &Error) {
    for (index, cause) in error.chain().enumerate() {
        if index == 0 {
            eprintln!("{}: {cause}", "error".red());
            continue;
        }

        if index == 1 {
            eprintln!("{}", "caused by:".red());
        }
        println!("    {}: {cause}", index - 1);
    }
}

fn main() {
    if let Err(error) = run() {
        print_error(&error);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    // Parse CLI arguments.
    let cli = Cli::parse();

    // Load the user config.
    let config_path = cli
        .config_path
        .or_else(|| match cli::default_config_path() {
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

    // Process Cubase project files.
    let path_ignore_globs = config
        .path_ignore_patterns
        .iter()
        .map(|pattern| {
            Pattern::new(pattern).with_context(|| {
                format!("unable to parse path ignore pattern '{}'", pattern.blue())
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let mut project_bytes = Vec::new();
    let mut plugin_counts = HashMap::new();
    let mut plugin_counts_32 = HashMap::new();
    let mut plugin_counts_64 = HashMap::new();

    for project_path in &cli.project_paths {
        if let Err(error) = process_cubase_project_path(
            project_path,
            &config,
            &path_ignore_globs,
            &mut project_bytes,
            &mut plugin_counts_32,
            &mut plugin_counts_64,
            &mut plugin_counts,
        ) {
            let project_path_heading = format!("Path: {project_path}").white().on_red();
            println!();
            println!("{project_path_heading}");
            println!();
            print_error(&error);
        }
    }

    print_summary(&plugin_counts_32, "32-bit");
    print_summary(&plugin_counts_64, "64-bit");
    print_summary(&plugin_counts, "All");

    Ok(())
}

fn process_cubase_project_path(
    project_path: &str,
    config: &Config,
    path_ignore_globs: &[Pattern],
    project_bytes: &mut Vec<u8>,
    plugin_counts_32: &mut HashMap<Plugin, i32>,
    plugin_counts_64: &mut HashMap<Plugin, i32>,
    plugin_counts: &mut HashMap<Plugin, i32>,
) -> Result<()> {
    let project_file_path_pattern = Path::new(&project_path).join("**").join("*.cpr");
    let Some(project_file_path_pattern) = project_file_path_pattern.to_str() else {
        bail!("unable to convert the project file pattern to a string");
    };

    let project_file_paths = glob::glob_with(
        project_file_path_pattern,
        MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        },
    )
    .context("unable to glob for project files in the project path")?;

    for project_file_path in project_file_paths {
        let project_file_path = project_file_path
            .context("unable to glob a particular project file in the project path")?;

        if path_ignore_globs
            .iter()
            .any(|glob| glob.matches_path(&project_file_path))
        {
            continue;
        }

        let project_file_path_heading = format!("Path: {}", project_file_path.display())
            .white()
            .on_red();
        println!();
        println!("{project_file_path_heading}");
        println!();

        if let Err(error) = process_cubase_project_file(
            &project_file_path,
            config,
            project_bytes,
            plugin_counts_32,
            plugin_counts_64,
            plugin_counts,
        ) {
            print_error(&error);
        }
    }

    Ok(())
}

fn process_cubase_project_file(
    project_file_path: &Path,
    config: &Config,
    project_bytes: &mut Vec<u8>,
    plugin_counts_32: &mut HashMap<Plugin, i32>,
    plugin_counts_64: &mut HashMap<Plugin, i32>,
    plugin_counts: &mut HashMap<Plugin, i32>,
) -> Result<()> {
    let mut file = File::open(project_file_path).context("unable to open project file")?;
    file.read_to_end(project_bytes)
        .context("unable to read project file")?;

    let reader = Reader::new(project_bytes);
    let project_details = reader
        .get_project_details()
        .context("unable to parse project file")?;
    project_bytes.clear();

    let is_64_bit = matches!(
        project_details.metadata.architecture.as_str(),
        "WIN64" | "MAC64 LE"
    );

    if is_64_bit && !config.projects.report_64_bit || !is_64_bit && !config.projects.report_32_bit {
        return Ok(());
    }

    let project_heading = format!(
        "{application} {version} ({architecture})",
        application = project_details.metadata.application,
        version = project_details.metadata.version,
        architecture = project_details.metadata.architecture
    )
    .blue();
    println!("{project_heading}");

    if project_details.plugins.is_empty() {
        return Ok(());
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

    Ok(())
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
