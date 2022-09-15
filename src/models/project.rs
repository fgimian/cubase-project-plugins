use std::collections::HashSet;

/// Contains information about the Cubase version used to create the project
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ProjectMetadata {
    /// The Cubase application name.
    pub application: String,

    /// The Cubase version used to create the project.
    pub version: String,

    /// The release date of the Cubase version used.
    pub release_date: String,

    /// The system architecture of the Cubase version used.
    pub architecture: String,
}

/// Represents a plugin within a Cubase project.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ProjectPlugin {
    /// The globally unique identifier for the plugin.
    pub guid: String,

    /// The name of the plugin.
    pub name: String,
}

/// Captures the Cubase version and all plugins used for a Cubase project.
#[derive(Debug)]
pub struct ProjectDetails {
    /// Information about the Cubase version used to create the project.
    pub metadata: ProjectMetadata,

    /// All the plugins used in the project.
    pub plugins: HashSet<ProjectPlugin>,
}
