use serde::Deserialize;

/// Project specific configuration for the tool.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Projects {
    /// Whether 32-bit projects should be reported.
    pub report_32_bit: bool,
    /// Whether 64-bit projects should be reported.
    pub report_64_bit: bool,
}

impl Default for Projects {
    fn default() -> Self {
        Self {
            report_32_bit: true,
            report_64_bit: true,
        }
    }
}

/// Plugin specific configuration for the tool.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Plugins {
    /// Plugin GUIDs which should be ignored.
    pub guid_ignores: Vec<String>,
    /// Plugin names which should be ignored.
    pub name_ignores: Vec<String>,
}

/// The main configuration structure for the tool.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Project path patterns to skip.
    pub path_ignore_patterns: Vec<String>,
    /// Configuration related to projects.
    pub projects: Projects,
    /// Configuration related to plugins.
    pub plugins: Plugins,
}
