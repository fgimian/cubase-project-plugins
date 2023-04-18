use std::collections::HashSet;

use crate::cstring_extras;
use crate::models::project::{Metadata, Plugin, Project};

const PLUGIN_UID_SEARCH_TERM: &[u8] = b"Plugin UID\0";
const APP_VERSION_SEARCH_TERM: &[u8] = b"PAppVersion\0";

/// Determines the used plugins in a Cubase project along with related version of Cubase which the
/// project was created on by parsing the binary in a *.cpr file.
pub struct Reader {
    /// The binary Cubase project bytes.
    project_bytes: Vec<u8>,
}

impl Reader {
    pub fn new(project_bytes: Vec<u8>) -> Self {
        Self { project_bytes }
    }

    /// Obtains all project details including Cubase version and plugins used.
    pub fn get_project_details(&self) -> Project {
        let mut metadata = Metadata {
            application: String::from("Cubase"),
            version: String::from("Unknown"),
            release_date: String::from("Unknown"),
            architecture: String::from("Unknown"),
        };
        let mut plugins = HashSet::new();

        let mut index = 0;
        while index < self.project_bytes.len() {
            if char::from(self.project_bytes[index]) != 'P' {
                index += 1;
            } else if let Some((found_metadata, updated_index)) = self.search_metadata(index) {
                metadata = found_metadata;
                index = updated_index;
            } else if let Some((found_plugin, updated_index)) = self.search_plugin(index) {
                plugins.insert(found_plugin);
                index = updated_index;
            } else {
                index += 1;
            }
        }

        Project { metadata, plugins }
    }

    fn search_metadata(&self, index: usize) -> Option<(Metadata, usize)> {
        let mut read_index = index;

        let version_term = self.get_bytes(read_index, APP_VERSION_SEARCH_TERM.len())?;
        if version_term != APP_VERSION_SEARCH_TERM {
            return None;
        }
        read_index += APP_VERSION_SEARCH_TERM.len() + 9;

        let (application, len) = self.get_token(read_index)?;
        read_index += len + 3;

        let (version, len) = self.get_token(read_index)?;
        read_index += len + 3;

        let (release_date, len) = self.get_token(read_index)?;
        read_index += len + 7;

        // Older 32-bit versions of Cubase didn't list the architecture in the project file.
        let architecture = match self.get_token(read_index) {
            Some((architecture, len)) => {
                read_index += len;
                architecture
            }
            None => String::from("Not Specified"),
        };

        Some((
            Metadata {
                application,
                version,
                release_date,
                architecture,
            },
            read_index,
        ))
    }

    fn search_plugin(&self, index: usize) -> Option<(Plugin, usize)> {
        let mut read_index = index;

        let uid_term = self.get_bytes(read_index, PLUGIN_UID_SEARCH_TERM.len())?;
        if uid_term != PLUGIN_UID_SEARCH_TERM {
            return None;
        }

        read_index += PLUGIN_UID_SEARCH_TERM.len() + 22;
        let (guid, len) = self.get_token(read_index)?;
        read_index += len + 3;

        let (key, len) = self.get_token(read_index)?;
        if key != "Plugin Name" {
            return None;
        }
        read_index += len + 5;

        let (mut name, len) = self.get_token(read_index)?;
        read_index += len + 3;

        let (key, len) = self.get_token(read_index)?;
        if key == "Original Plugin Name" {
            read_index += len + 5;

            name = match self.get_token(read_index) {
                Some((original_name, len)) => {
                    read_index += len;
                    original_name
                }
                None => name,
            };
        }

        Some((Plugin { guid, name }, read_index))
    }

    fn get_bytes(&self, index: usize, len: usize) -> Option<&[u8]> {
        let end = index + len;
        if end > self.project_bytes.len() {
            return None;
        }

        let buffer = &self.project_bytes[index..end];
        Some(buffer)
    }

    fn get_token(&self, index: usize) -> Option<(String, usize)> {
        let len_bytes = self.get_bytes(index, 1)?;
        let len = usize::from(len_bytes[0]);

        let token_bytes = self.get_bytes(index + 1, len)?;
        let token = cstring_extras::from_vec_until_nul(token_bytes)
            .ok()?
            .into_string()
            .ok()?;

        Some((token, len + 1))
    }
}
