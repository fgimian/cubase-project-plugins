use std::collections::HashSet;

/// Represents a plugin within a Cubase project.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Plugin {
    /// The globally unique identifier for the plugin.
    pub guid: String,

    /// The name of the plugin.
    pub name: String,
}

/// Captures the Cubase version and all plugins used for a Cubase project.
#[derive(Debug)]
pub struct ProjectDetails {
    /// The Cubase application name.
    pub cubase_application: String,

    /// The Cubase version used to create the project.
    pub cubase_version: String,

    /// The release date of the Cubase version used.
    pub cubase_release_date: String,

    /// The system architecture of the Cubase version used.
    pub architecture: String,

    /// All the plugins used in the project.
    pub plugins: HashSet<Plugin>,
}
