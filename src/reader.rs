use std::collections::HashSet;

use thiserror::Error;

use crate::project::{Metadata, Plugin, Project};

const PLUGIN_UID_SEARCH_TERM: &[u8] = b"Plugin UID\0";
const APP_VERSION_SEARCH_TERM: &[u8] = b"PAppVersion\0";

#[derive(Error, Debug)]
pub enum Error {
    #[error("the length byte goes beyond the end of the project")]
    LengthBeyondEOF,
    #[error("the token size goes beyond the end of the project")]
    TokenBeyondEOF,
    #[error("the project has no metadata and appears to be corrupt")]
    CorruptProject,
    #[error("unable to obtain the application name")]
    NoApplication,
    #[error("unable to obtain the application version")]
    NoVersion,
    #[error("unable to obtain the application release date")]
    NoReleaseDate,
    #[error("unable to obtain a plugin GUID")]
    NoPluginGUID,
    #[error("unable to obtain a plugin name")]
    NoPluginName,
    #[error("unable to obtain the token after a plugin name")]
    NoTokenAfterPluginName,
    #[error("unable to obtain an original plugin name")]
    NoOriginalPluginName,
}

/// Determines the used plugins in a Cubase project along with related version of Cubase which the
/// project was created on by parsing the binary in a *.cpr file.
pub struct Reader<'a> {
    /// Binary Cubase project bytes.
    project_bytes: &'a [u8],
}

impl<'a> Reader<'a> {
    pub const fn new(project_bytes: &'a [u8]) -> Self {
        Self { project_bytes }
    }

    /// Obtains all project details including Cubase version and plugins used and returns an
    /// instance of Project containing project details.
    pub fn get_project_details(&self) -> Result<Project, Error> {
        let mut metadata: Option<Metadata> = None;
        let mut plugins = HashSet::new();

        let mut index = 0;
        while index < self.project_bytes.len() {
            // Check if the current byte matches the letter P which is the first letter of all our
            // search terms.
            if char::from(self.project_bytes[index]) != 'P' {
                index += 1;
                continue;
            }

            // Check whether the next set of bytes are related to the Cubase version.
            if metadata.is_none() {
                if let Some((found_metadata, updated_index)) = self.search_metadata(index)? {
                    metadata = Some(found_metadata);
                    index = updated_index;
                    continue;
                }
            }

            // Check whether the next set of bytes relate to a plugin.
            if let Some((found_plugin, updated_index)) = self.search_plugin(index)? {
                plugins.insert(found_plugin);
                index = updated_index;
                continue;
            }

            index += 1;
        }

        metadata.map_or_else(
            || Err(Error::CorruptProject),
            |metadata| Ok(Project { metadata, plugins }),
        )
    }

    fn search_metadata(&self, index: usize) -> Result<Option<(Metadata, usize)>, Error> {
        let mut index = index;

        match self.get_bytes(index, APP_VERSION_SEARCH_TERM.len()) {
            Some(APP_VERSION_SEARCH_TERM) => (),
            _ => return Ok(None),
        };
        index += APP_VERSION_SEARCH_TERM.len() + 9;

        let (application, len) = self.get_token(index).map_err(|_| Error::NoApplication)?;
        index += len + 3;

        let (version, len) = self.get_token(index).map_err(|_| Error::NoVersion)?;
        index += len + 3;

        let (release_date, len) = self.get_token(index).map_err(|_| Error::NoReleaseDate)?;
        index += len + 7;

        // Older 32-bit versions of Cubase didn't list the architecture in the project file.
        let architecture = match self.get_token(index) {
            Ok((architecture, len)) => {
                index += len;
                architecture
            }
            Err(_) => String::from("Unspecified"),
        };

        Ok(Some((
            Metadata {
                application,
                version,
                release_date,
                architecture,
            },
            index,
        )))
    }

    fn search_plugin(&self, index: usize) -> Result<Option<(Plugin, usize)>, Error> {
        let mut read_index = index;

        match self.get_bytes(read_index, PLUGIN_UID_SEARCH_TERM.len()) {
            Some(PLUGIN_UID_SEARCH_TERM) => (),
            _ => return Ok(None),
        };
        read_index += PLUGIN_UID_SEARCH_TERM.len() + 22;

        let (guid, len) = self
            .get_token(read_index)
            .map_err(|_| Error::NoPluginGUID)?;
        read_index += len + 3;

        let (key, len) = self
            .get_token(read_index)
            .map_err(|_| Error::NoPluginName)?;
        if key != "Plugin Name" {
            return Err(Error::NoPluginName);
        }
        read_index += len + 5;

        let (mut name, len) = self
            .get_token(read_index)
            .map_err(|_| Error::NoPluginName)?;
        read_index += len + 3;

        // In Cubase 8.x and above, in cases where an instrument track has been renamed using
        // Shift+Enter, the name retrieved above will be the track title and the name of the plugin
        // will follow under the key "Original Plugin Name".
        let (key, len) = self
            .get_token(read_index)
            .map_err(|_| Error::NoTokenAfterPluginName)?;
        if key == "Original Plugin Name" {
            read_index += len + 5;

            let (original_name, len) = self
                .get_token(read_index)
                .map_err(|_| Error::NoOriginalPluginName)?;
            name = original_name;
            read_index += len;
        }

        Ok(Some((Plugin { guid, name }, read_index)))
    }

    fn get_bytes(&self, index: usize, len: usize) -> Option<&[u8]> {
        let end = index + len;
        if end > self.project_bytes.len() {
            return None;
        }

        Some(&self.project_bytes[index..end])
    }

    fn get_token(&self, index: usize) -> Result<(String, usize), Error> {
        let len_bytes = self.get_bytes(index, 1).ok_or(Error::LengthBeyondEOF)?;
        let len = usize::from(len_bytes[0]);

        let token_bytes = self
            .get_bytes(index + 1, len)
            .ok_or(Error::TokenBeyondEOF)?;

        // Older versions of before Cubase 5 didn't always provide nul terminators in token strings.
        let nul_index = token_bytes.iter().position(|&byte| byte == 0);
        let token = nul_index.map_or_else(
            || String::from_utf8_lossy(token_bytes),
            |nul_index| String::from_utf8_lossy(&token_bytes[..nul_index]),
        );

        Ok((token.to_string(), len + 1))
    }
}
