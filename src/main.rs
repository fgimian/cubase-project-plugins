#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::expect_used,
    // clippy::unwrap_used
)]
#![allow(clippy::too_many_lines)]

mod cli;
mod config;
mod project;
mod reader;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::{fs, path::Path};

use clap::Parser;
use config::Config;
use glob::{glob_with, MatchOptions, Pattern};
use project::Plugin;
use reader::Reader;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use cli::Cli;

fn main() {
    let cli = Cli::parse();

    let config = cli.config.map_or_else(Config::default, |config_path| {
        let config_string = fs::read_to_string(config_path).unwrap();
        toml::from_str(&config_string).unwrap()
    });

    let path_ignore_globs = config
        .path_ignore_patterns
        .iter()
        .map(|p| Pattern::new(p).unwrap())
        .collect::<Vec<Pattern>>();

    let mut project_file_paths = Vec::new();

    for project_path in cli.project_paths {
        if let Ok(paths) = glob_with(
            Path::new(&project_path)
                .join("**")
                .join("*.cpr")
                .to_str()
                .unwrap(),
            MatchOptions {
                case_sensitive: false,
                require_literal_separator: false,
                require_literal_leading_dot: false,
            },
        ) {
            let filtered_paths = paths.filter_map(Result::ok).filter(|path| {
                !path_ignore_globs.iter().any(|glob| {
                    path.clone()
                        .into_os_string()
                        .into_string()
                        .map_or(true, |path| glob.matches(&path))
                })
            });

            project_file_paths.extend(filtered_paths);
        }
    }

    let mut heading_spec = ColorSpec::new();
    heading_spec.set_bg(Some(Color::Red));
    heading_spec.set_fg(Some(Color::White));

    let mut project_spec = ColorSpec::new();
    project_spec.set_fg(Some(Color::Blue));

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let mut plugin_counts = HashMap::new();
    let mut plugin_counts_32 = HashMap::new();
    let mut plugin_counts_64 = HashMap::new();

    let mut data = Vec::new();

    for project_file_path in project_file_paths {
        let mut file = File::open(&project_file_path).unwrap();
        file.read_to_end(&mut data).unwrap();

        let reader = Reader::new(&data);
        let project_details = reader.get_project_details().unwrap();
        data.clear();

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
        stdout.set_color(&heading_spec).unwrap();
        write!(
            &mut stdout,
            "Path: {}",
            project_file_path.into_os_string().into_string().unwrap(),
        )
        .unwrap();
        stdout.reset().unwrap();
        println!();
        println!();

        stdout.set_color(&project_spec).unwrap();
        write!(
            &mut stdout,
            "{application} {version} ({architecture})",
            application = project_details.metadata.application,
            version = project_details.metadata.version,
            architecture = project_details.metadata.architecture
        )
        .unwrap();
        stdout.reset().unwrap();
        println!();

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

    print_summary(&plugin_counts_32, "32-bit", &mut stdout, &heading_spec).unwrap();
    print_summary(&plugin_counts_64, "64-bit", &mut stdout, &heading_spec).unwrap();
    print_summary(&plugin_counts, "All", &mut stdout, &heading_spec).unwrap();
}

fn print_summary(
    plugin_counts: &HashMap<Plugin, i32>,
    description: &str,
    stdout: &mut StandardStream,
    heading_spec: &ColorSpec,
) -> io::Result<()> {
    if plugin_counts.is_empty() {
        return Ok(());
    }

    println!();
    stdout.set_color(heading_spec)?;
    write!(stdout, "Summary: Plugins Used In {description} Projects")?;
    stdout.reset()?;
    println!();
    println!();

    let mut sorted_plugin_counts = Vec::from_iter(plugin_counts);
    sorted_plugin_counts.sort_by(|a, b| a.0.name.to_lowercase().cmp(&b.0.name.to_lowercase()));

    for (plugin, count) in &sorted_plugin_counts {
        println!("    > {} : {} ({})", plugin.guid, plugin.name, count);
    }

    Ok(())
}
