use crate::cstring_extras;
use std::collections::HashSet;

use crate::models::project::{Plugin, ProjectDetails};

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
    /// Initializes a new instance of the <see cref="ProjectReader"/> class.
    /// <param name="project_bytes">The binary bytes from a *.cpr Cubase project file.</param>
    /// <param name="guidIgnores">All plugin GUIDs which should be ignored.</param>
    /// <param name="nameIgnores">All plugin names which should be ignored.</param>
    pub fn new(project_bytes: Vec<u8>) -> Reader {
        Reader {
            guid_ignores: Vec::new(),
            name_ignores: Vec::new(),
            project_bytes,
        }
    }

    /// Obtains all project details including Cubase version and plugins used.
    // /// <returns>An instance of <see cref="ProjectDetails"/>containing project details.</returns>
    pub fn get_project_details(&self) -> ProjectDetails {
        let mut plugins = HashSet::new();
        let mut cubase_application = String::from("Cubase");
        let mut cubase_version = String::from("Unknown");
        let mut cubase_release_date = String::from("Unknown");
        let mut architecture = String::from("Unknown");

        for (index, &byte) in self.project_bytes.iter().enumerate() {
            if char::from(byte) != 'P' {
                continue;
            }

            let mut current_index: usize;

            // Check that the next set of bytes related to the Cubase version.
            current_index = index;
            let version_term =
                self.get_token_bytes_with_len(&mut current_index, APP_VERSION_SEARCH_TERM.len());
            if version_term == APP_VERSION_SEARCH_TERM {
                current_index += APP_VERSION_SEARCH_TERM.len() + 9;
                cubase_application = self.get_token(&mut current_index);
                current_index += 3;
                cubase_version = self.get_token(&mut current_index);
                current_index += 3;
                cubase_release_date = self.get_token(&mut current_index);
                current_index += 7;
                architecture = self.get_token(&mut current_index);
                // Older 32-bit versions of Cubase didn't list the architecture in the project file.
                // architecture = "Not Specified";
                continue;
            }

            // Check that the next set of bytes relate to a plugin.
            current_index = index;
            let uid_term =
                self.get_token_bytes_with_len(&mut current_index, PLUGIN_UID_SEARCH_TERM.len());
            if uid_term == PLUGIN_UID_SEARCH_TERM {
                current_index += PLUGIN_UID_SEARCH_TERM.len() + 22;
                let guid = self.get_token(&mut current_index);
                current_index += 3;

                let mut key: String;

                key = self.get_token(&mut current_index);
                if key != "Plugin Name" {
                    continue;
                }
                current_index += 5;

                let mut name = self.get_token(&mut current_index);
                current_index += 3;
                key = self.get_token(&mut current_index);
                if key == "Original Plugin Name" {
                    current_index += 5;
                    name = self.get_token(&mut current_index);
                }

                // Skip GUIDs that are to be ignored.
                if self.guid_ignores.contains(&guid) {
                    continue;
                }

                // Skip names that are to be ignored.
                if self.name_ignores.contains(&name) {
                    continue;
                }

                let plugin = Plugin { guid, name };
                plugins.insert(plugin);
            }
        }

        ProjectDetails {
            cubase_application,
            cubase_version,
            cubase_release_date,
            architecture,
            plugins,
        }
    }

    fn get_token_bytes_with_len(&self, current_index: &mut usize, len: usize) -> Vec<u8> {
        let end = *current_index + len;
        let buffer = &self.project_bytes[*current_index..end];
        buffer.to_vec()
    }

    fn get_token(&self, current_index: &mut usize) -> String {
        let len = usize::from(self.project_bytes[*current_index]);
        *current_index += 1;

        let token_bytes = self.get_token_bytes_with_len(current_index, len);
        *current_index += len;

        cstring_extras::from_vec_until_nul(token_bytes.to_vec())
            .unwrap()
            .into_string()
            .unwrap()
    }
}
