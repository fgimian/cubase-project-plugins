use serde::Deserialize;

fn report_default() -> bool {
    true
}

/// Project specific configuration for the tool.
#[derive(Debug, Deserialize)]
pub struct ProjectsConfig {
    /// Indicates whether 32-bit projects should be reported.
    #[serde(default = "report_default")]
    pub report_32_bit: bool,
    /// Indicates whether 64-bit projects should be reported.
    #[serde(default = "report_default")]
    pub report_64_bit: bool,
}

/// Plugin specific configuration for the tool.
#[derive(Debug, Deserialize)]
pub struct PluginsConfig {
    /// Plugin GUIDs which are to be ignored.
    pub guid_ignores: Vec<String>,
    /// Plugin names which are to be ignored.
    pub name_ignores: Vec<String>,
}

/// The main configuration structure for the tool.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Path patterns which determine which projects are skipped.
    pub path_ignore_patterns: Vec<String>,
    /// Configuration related to projects.
    pub projects: ProjectsConfig,
    /// Configuration related to plugins.
    pub plugins: PluginsConfig,
}
