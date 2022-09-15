use crate::cstring_extras;
use crate::models::project::{ProjectDetails, ProjectMetadata, ProjectPlugin};
use std::collections::HashSet;

const PLUGIN_UID_SEARCH_TERM: &[u8] = b"Plugin UID\0";
const APP_VERSION_SEARCH_TERM: &[u8] = b"PAppVersion\0";

/// Determines the used plugins in a Cubase project along with related version of Cubase which the
/// project was created on by parsing the binary in a *.cpr file.
pub struct Reader {
    /// All plugin GUIDs that should not captured.  Typically this will be the plugins which are
    /// included in Cubase itself.  This is a more accurate way of excluding plugins than usin
    /// their name.
    pub guid_ignores: Vec<String>,

    /// All plugin names that should not captured.  Typically this will be the plugins
    /// which are included in Cubase itself.
    pub name_ignores: Vec<String>,

    /// The binary Cubase project bytes.
    project_bytes: Vec<u8>,
}

impl Reader {
    pub fn new(project_bytes: Vec<u8>) -> Reader {
        Reader {
            guid_ignores: Vec::new(),
            name_ignores: Vec::new(),
            project_bytes,
        }
    }

    /// Obtains all project details including Cubase version and plugins used.
    pub fn get_project_details(&self) -> ProjectDetails {
        let mut metadata = ProjectMetadata {
            application: String::from("Cubase"),
            version: String::from("Unknown"),
            release_date: String::from("Unknown"),
            architecture: String::from("Unknown"),
        };
        let mut plugins = HashSet::new();

        let mut index: usize = 0;

        while index < self.project_bytes.len() {
            if char::from(self.project_bytes[index]) != 'P' {
                index += 1;
                continue;
            }

            if let Some(found_metadata) = self.search_metadata(&mut index) {
                metadata = found_metadata;
            } else if let Some(found_plugin) = self.search_plugin(&mut index) {
                if self.guid_ignores.contains(&found_plugin.guid)
                    || self.name_ignores.contains(&found_plugin.name)
                {
                    continue;
                }

                plugins.insert(found_plugin);
            } else {
                index += 1;
            }
        }

        ProjectDetails { metadata, plugins }
    }

    fn search_metadata(&self, index: &mut usize) -> Option<ProjectMetadata> {
        let mut read_index = *index;

        let version_term =
            self.get_token_bytes_with_len(&mut read_index, APP_VERSION_SEARCH_TERM.len())?;

        if version_term != APP_VERSION_SEARCH_TERM {
            return None;
        }

        read_index += APP_VERSION_SEARCH_TERM.len() + 9;
        let application = self.get_token(&mut read_index)?;
        read_index += 3;
        let version = self.get_token(&mut read_index)?;
        read_index += 3;
        let release_date = self.get_token(&mut read_index)?;
        read_index += 7;
        // Older 32-bit versions of Cubase didn't list the architecture in the project file.
        let architecture = match self.get_token(&mut read_index) {
            Some(architecture) => architecture,
            None => String::from("Not Specified"),
        };

        *index = read_index;

        Some(ProjectMetadata {
            application,
            version,
            release_date,
            architecture,
        })
    }

    fn search_plugin(&self, index: &mut usize) -> Option<ProjectPlugin> {
        let mut read_index = *index;

        let uid_term =
            self.get_token_bytes_with_len(&mut read_index, PLUGIN_UID_SEARCH_TERM.len())?;

        if uid_term != PLUGIN_UID_SEARCH_TERM {
            return None;
        }

        read_index += PLUGIN_UID_SEARCH_TERM.len() + 22;
        let guid = self.get_token(&mut read_index)?;
        read_index += 3;

        let mut key: String;

        key = self.get_token(&mut read_index)?;
        if key != "Plugin Name" {
            return None;
        }
        read_index += 5;

        let mut name = self.get_token(&mut read_index)?;
        read_index += 3;
        key = self.get_token(&mut read_index)?;
        if key == "Original Plugin Name" {
            read_index += 5;
            name = match self.get_token(&mut read_index) {
                Some(original_name) => original_name,
                None => name,
            };
        }

        *index = read_index;

        Some(ProjectPlugin { guid, name })
    }

    fn get_token_bytes_with_len(&self, read_index: &mut usize, len: usize) -> Option<Vec<u8>> {
        let end = *read_index + len;
        if end > self.project_bytes.len() {
            return None;
        }

        let token_bytes = &self.project_bytes[*read_index..end];
        Some(token_bytes.to_vec())
    }

    fn get_token(&self, read_index: &mut usize) -> Option<String> {
        let len = usize::from(self.project_bytes[*read_index]);
        *read_index += 1;

        let token_bytes = self.get_token_bytes_with_len(read_index, len)?;
        let token = cstring_extras::from_vec_until_nul(token_bytes.to_vec())
            .ok()?
            .into_string()
            .ok()?;
        *read_index += len;

        Some(token)
    }
}
