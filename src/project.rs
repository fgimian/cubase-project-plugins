use std::collections::HashSet;

/// Contains information about the Cubase version used to create the project.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Metadata {
    /// Application name (this is always "Cubase").
    pub application: String,
    /// Version of Cubase used to create the project.
    pub version: String,
    /// Release date of the Cubase version used.
    pub release_date: String,
    /// System architecture used to create the project.
    pub architecture: String,
}

/// Represents a plugin within a Cubase project.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Plugin {
    /// Globally unique identifier for the plugin.
    pub guid: String,
    /// Name of the plugin.
    pub name: String,
}

/// Captures the Cubase version and all plugins used for a Cubase project.
#[derive(Debug)]
pub struct Project {
    /// Metadata describing the Cubase version used to create the project.
    pub metadata: Metadata,
    /// Plugins used in the project.
    pub plugins: HashSet<Plugin>,
}
