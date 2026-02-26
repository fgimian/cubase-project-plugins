mod cli;
mod config;
mod project;
mod reader;

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read},
    path::Path,
    process,
};

use anyhow::{Context, Error, Result, anyhow, bail};
use clap::{CommandFactory as _, Parser as _};
use colored::Colorize as _;
use glob::{MatchOptions, Pattern};
use wildmatch::{WildMatch, WildMatchPattern};

use crate::{cli::Cli, config::Config, project::Plugin, reader::Reader};

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

    // Display shell completions.
    if let Some(shell) = cli.completions {
        let mut cmd = Cli::command();
        clap_complete::generate(shell, &mut cmd, env!("CARGO_PKG_NAME"), &mut io::stdout());
        return Ok(());
    }

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

    let mut processor = Processor::new(
        config,
        path_ignore_globs,
        cli.patterns,
        cli.only_show_filtered,
    );

    for project_path in &cli.project_paths {
        let project_path = Path::new(project_path);
        if !project_path.is_dir() {
            let project_path_heading = format!("Path: {}", project_path.display()).white().on_red();
            println!();
            println!("{project_path_heading}");
            println!();
            print_error(&anyhow!("the directory path does not exist"));
            continue;
        }

        if let Err(error) = processor.process_cubase_project_path(project_path) {
            let project_path_heading = format!("Path: {}", project_path.display()).white().on_red();
            println!();
            println!("{project_path_heading}");
            println!();
            print_error(&error);
        }
    }

    processor.print_summaries();

    Ok(())
}

struct Processor {
    config: Config,
    path_ignore_globs: Vec<Pattern>,
    filter_patterns: Vec<WildMatchPattern<'*', '?'>>,
    project_bytes: Vec<u8>,
    plugin_counts_32: HashMap<Plugin, i32>,
    plugin_counts_64: HashMap<Plugin, i32>,
    plugin_counts: HashMap<Plugin, i32>,
    cubase_version_counts: HashMap<String, i32>,
    only_show_filtered: bool,
}

impl Processor {
    pub fn new(
        config: Config,
        path_ignore_globs: impl IntoIterator<Item = Pattern>,
        filter_patterns: impl IntoIterator<Item = String>,
        only_show_filtered: bool,
    ) -> Self {
        Self {
            config,
            path_ignore_globs: path_ignore_globs.into_iter().collect(),
            filter_patterns: filter_patterns
                .into_iter()
                .map(|pattern| WildMatch::new_case_insensitive(&pattern))
                .collect::<Vec<_>>(),
            project_bytes: Vec::new(),
            plugin_counts_32: HashMap::new(),
            plugin_counts_64: HashMap::new(),
            plugin_counts: HashMap::new(),
            cubase_version_counts: HashMap::new(),
            only_show_filtered,
        }
    }

    pub fn process_cubase_project_path(&mut self, project_path: &Path) -> Result<()> {
        let project_file_path_pattern = project_path.join("**").join("*.cpr");
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
            let project_file_path = match project_file_path {
                Ok(project_file_path) => project_file_path,
                Err(error) => {
                    let error = Error::from(error)
                        .context("unable to glob a particular project file in the project path");
                    print_error(&error);
                    continue;
                }
            };

            if self
                .path_ignore_globs
                .iter()
                .any(|glob| glob.matches_path(&project_file_path))
            {
                continue;
            }

            if let Err(error) = self.process_cubase_project_file(&project_file_path) {
                print_error(&error);
            }
        }

        Ok(())
    }

    fn matches_filters(&self, plugin: &Plugin) -> bool {
        self.filter_patterns.is_empty()
            || self
                .filter_patterns
                .iter()
                .any(|pattern| pattern.matches(&plugin.name) || pattern.matches(&plugin.guid))
    }

    fn process_cubase_project_file(&mut self, project_file_path: &Path) -> Result<()> {
        let mut file = File::open(project_file_path).context("unable to open project file")?;
        file.read_to_end(&mut self.project_bytes)
            .context("unable to read project file")?;

        let reader = Reader::new(&self.project_bytes);
        let project_details = reader
            .get_project_details()
            .context("unable to parse project file")?;
        self.project_bytes.clear();

        let mut sorted_plugins = Vec::from_iter(project_details.plugins);
        sorted_plugins.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        let filtered_plugins = sorted_plugins
            .iter()
            .filter(|p| !self.config.plugins.guid_ignores.contains(&p.guid))
            .filter(|p| !self.config.plugins.name_ignores.contains(&p.name))
            .collect::<Vec<_>>();

        if !self.filter_patterns.is_empty()
            && !filtered_plugins
                .iter()
                .any(|plugin| self.matches_filters(plugin))
        {
            return Ok(());
        }

        let project_file_path_heading = format!("Path: {}", project_file_path.display())
            .white()
            .on_red();
        println!();
        println!("{project_file_path_heading}");
        println!();

        let is_64_bit = matches!(
            project_details.metadata.architecture.as_str(),
            "WIN64" | "MAC64 LE"
        );

        if is_64_bit && !self.config.projects.report_64_bit
            || !is_64_bit && !self.config.projects.report_32_bit
        {
            return Ok(());
        }

        let cubase_version = format!(
            "{application} {version} ({architecture})",
            application = project_details.metadata.application,
            version = project_details.metadata.version,
            architecture = project_details.metadata.architecture
        );

        let project_heading = cubase_version.blue();
        println!("{project_heading}");

        if filtered_plugins.is_empty() {
            return Ok(());
        }

        self.cubase_version_counts
            .entry(cubase_version)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        println!();
        for plugin in filtered_plugins {
            self.plugin_counts
                .entry(plugin.clone())
                .and_modify(|count| *count += 1)
                .or_insert(1);

            if is_64_bit {
                self.plugin_counts_64
                    .entry(plugin.clone())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            } else {
                self.plugin_counts_32
                    .entry(plugin.clone())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }

            if !self.only_show_filtered || self.matches_filters(plugin) {
                println!("    > {} : {}", plugin.guid, plugin.name);
            }
        }

        Ok(())
    }

    pub fn print_summaries(&self) {
        self.print_plugin_summary(&self.plugin_counts_32, "32-bit");
        self.print_plugin_summary(&self.plugin_counts_64, "64-bit");
        self.print_plugin_summary(&self.plugin_counts, "all");
        self.print_cubase_version_summary();
    }

    fn print_plugin_summary(&self, plugin_counts: &HashMap<Plugin, i32>, description: &str) {
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
            if !self.only_show_filtered || self.matches_filters(plugin) {
                println!("    > {} : {} ({count})", plugin.guid, plugin.name);
            }
        }
    }

    fn print_cubase_version_summary(&self) {
        if self.cubase_version_counts.is_empty() {
            return;
        }

        let summary_heading = "Summary: Cubase Versions Used In Projects".white().on_red();

        println!();
        println!("{summary_heading}");
        println!();

        let mut sorted_cubase_version_counts = Vec::from_iter(&self.cubase_version_counts);
        sorted_cubase_version_counts.sort_by(|a, b| natord::compare_ignore_case(a.0, b.0));

        for (cubase_version, count) in &sorted_cubase_version_counts {
            println!("    > {cubase_version} ({count})");
        }
    }
}
