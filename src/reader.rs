use std::collections::HashSet;

use thiserror::Error;

use crate::project::{Metadata, Plugin, Project};

const PLUGIN_UID_SEARCH_TERM: &[u8] = b"Plugin UID\0";
const APP_VERSION_SEARCH_TERM: &[u8] = b"PAppVersion\0";

#[derive(Error, Debug, Eq, PartialEq)]
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

        let version = version
            .strip_prefix("Version ")
            .map(ToString::to_string)
            .unwrap_or(version);

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
        let mut index = index;

        match self.get_bytes(index, PLUGIN_UID_SEARCH_TERM.len()) {
            Some(PLUGIN_UID_SEARCH_TERM) => (),
            _ => return Ok(None),
        };
        index += PLUGIN_UID_SEARCH_TERM.len() + 22;

        let (guid, len) = self.get_token(index).map_err(|_| Error::NoPluginGUID)?;
        index += len + 3;

        let (key, len) = self.get_token(index).map_err(|_| Error::NoPluginName)?;
        if key != "Plugin Name" {
            return Err(Error::NoPluginName);
        }
        index += len + 5;

        let (mut name, len) = self.get_token(index).map_err(|_| Error::NoPluginName)?;
        index += len + 3;

        // In Cubase 8.x and above, in cases where an instrument track has been renamed using
        // Shift+Enter, the name retrieved above will be the track title and the name of the plugin
        // will follow under the key "Original Plugin Name".
        let (key, len) = self
            .get_token(index)
            .map_err(|_| Error::NoTokenAfterPluginName)?;
        if key == "Original Plugin Name" {
            index += len + 5;

            let (original_name, len) = self
                .get_token(index)
                .map_err(|_| Error::NoOriginalPluginName)?;
            name = original_name;
            index += len;
        }

        Ok(Some((Plugin { guid, name }, index)))
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

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
mod tests {
    use std::{fs, path::PathBuf};

    use rstest::*;

    use super::*;

    struct PluginProperties {
        pub includes_channel_plugins: bool,
        pub dither_plugin_name: String,
    }

    impl Default for PluginProperties {
        fn default() -> Self {
            Self {
                includes_channel_plugins: false,
                dither_plugin_name: "UV22HR".to_string(),
            }
        }
    }

    #[rstest]
    #[case::cubase_45_32_bit(
        "Example Project (Cubase 4.5 32-bit).cpr",
        Metadata {
            application: "Cubase".to_string(),
            version: "4.5.2".to_string(),
            release_date: "Sep  2 2008".to_string(),
            architecture: "WIN32".to_string(),
        },
        PluginProperties::default(),
    )]
    #[case::cubase_45_64_bit(
        "Example Project (Cubase 4.5 64-bit).cpr",
        Metadata {
            application: "Cubase".to_string(),
            version: "4.5.2".to_string(),
            release_date: "Sep  2 2008".to_string(),
            architecture: "WIN64".to_string(),
        },
        PluginProperties::default(),
    )]
    #[case::cubase_5_32_bit(
        "Example Project (Cubase 5 32-bit).cpr",
        Metadata {
            application: "Cubase".to_string(),
            version: "5.5.3".to_string(),
            release_date: "Jan 13 2011".to_string(),
            architecture: "WIN32".to_string(),
        },
        PluginProperties::default(),
    )]
    #[case::cubase_5_64_bit(
        "Example Project (Cubase 5 64-bit).cpr",
        Metadata {
            application: "Cubase".to_string(),
            version: "5.5.3".to_string(),
            release_date: "Jan 13 2011".to_string(),
            architecture: "WIN64".to_string(),
        },
        PluginProperties::default(),
    )]
    #[case::cubase_65_32_bit(
        "Example Project (Cubase 6.5 32-bit).cpr",
        Metadata {
            application: "Cubase".to_string(),
            version: "6.5.5".to_string(),
            release_date: "Jun 24 2013".to_string(),
            architecture: "WIN32".to_string(),
        },
        PluginProperties::default(),
    )]
    #[case::cubase_65_64_bit(
        "Example Project (Cubase 6.5 64-bit).cpr",
        Metadata {
            application: "Cubase".to_string(),
            version: "6.5.5".to_string(),
            release_date: "Jun 24 2013".to_string(),
            architecture: "WIN64".to_string(),
        },
        PluginProperties::default(),
    )]
    #[case::cubase_7_32_bit(
        "Example Project (Cubase 7 32-bit).cpr",
        Metadata {
            application: "Cubase".to_string(),
            version: "7.0.7".to_string(),
            release_date: "Jan 21 2014".to_string(),
            architecture: "WIN32".to_string(),
        },
        PluginProperties {
            includes_channel_plugins: true,
            ..Default::default()
        },
    )]
    #[case::cubase_7_64_bit(
        "Example Project (Cubase 7 64-bit).cpr",
        Metadata {
            application: "Cubase".to_string(),
            version: "7.0.7".to_string(),
            release_date: "Jan 21 2014".to_string(),
            architecture: "WIN64".to_string(),
        },
        PluginProperties {
            includes_channel_plugins: true,
            ..Default::default()
        },
    )]
    #[case::cubase_85_32_bit(
        "Example Project (Cubase 8.5 32-bit).cpr",
        Metadata {
            application: "Cubase".to_string(),
            version: "8.5.30".to_string(),
            release_date: "Feb 22 2017".to_string(),
            architecture: "WIN32".to_string(),
        },
        PluginProperties {
            includes_channel_plugins: true,
            ..Default::default()
        },
    )]
    #[case::cubase_85_64_bit(
        "Example Project (Cubase 8.5 64-bit).cpr",
        Metadata {
            application: "Cubase".to_string(),
            version: "8.5.30".to_string(),
            release_date: "Feb 22 2017".to_string(),
            architecture: "WIN64".to_string(),
        },
        PluginProperties {
            includes_channel_plugins: true,
            ..Default::default()
        },
    )]
    #[case::cubase_95(
        "Example Project (Cubase 9.5).cpr",
        Metadata {
            application: "Cubase".to_string(),
            version: "9.5.50".to_string(),
            release_date: "Feb  2 2019".to_string(),
            architecture: "WIN64".to_string(),
        },
        PluginProperties {
            includes_channel_plugins: true,
            ..Default::default()
        },
    )]
    #[case::cubase_11(
        "Example Project (Cubase 11).cpr",
        Metadata {
            application: "Cubase".to_string(),
            version: "11.0.41".to_string(),
            release_date: "Sep 27 2021".to_string(),
            architecture: "WIN64".to_string(),
        },
        PluginProperties {
            includes_channel_plugins: true,
            ..Default::default()
        },
    )]
    #[case::cubase_13(
        "Example Project (Cubase 13).cpr",
        Metadata {
            application: "Cubase".to_string(),
            version: "13.0.10".to_string(),
            release_date: "Oct 10 2023".to_string(),
            architecture: "WIN64".to_string(),
        },
        PluginProperties {
            includes_channel_plugins: true,
            dither_plugin_name: "Lin Dither".to_string()
        },
    )]
    fn get_project_details(
        #[case] filename: &str,
        #[case] expected_metadata: Metadata,
        #[case] plugin_properties: PluginProperties,
    ) {
        let project_path = PathBuf::from("testdata").join(filename);
        let project_bytes = fs::read(project_path).unwrap();

        let reader = Reader::new(&project_bytes);
        let project_details = reader.get_project_details().unwrap();

        let mut actual_plugins_sorted = Vec::from_iter(project_details.plugins);
        actual_plugins_sorted.sort_by(|a, b| a.guid.to_lowercase().cmp(&b.guid.to_lowercase()));

        let mut expected_plugins = vec![Plugin {
            guid: "1C3A662167D347A99F7D797EA4911CDB".to_string(),
            name: "Elephant".to_string(),
        }];

        if plugin_properties.includes_channel_plugins {
            expected_plugins.push(Plugin {
                guid: "297BA567D83144E1AE921DEF07B41156".to_string(),
                name: "EQ".to_string(),
            });
        }

        expected_plugins.extend([
            Plugin {
                guid: "44E1149EDB3E4387BDD827FEA3A39EE7".to_string(),
                name: "Standard Panner".to_string(),
            },
            Plugin {
                guid: "565354414152626172747361636F7573".to_string(),
                name: "ArtsAcousticReverb".to_string(),
            },
            Plugin {
                guid: "565354416D62726F6D6E697370686572".to_string(),
                name: "Omnisphere".to_string(),
            },
            Plugin {
                guid: "56535444475443747261636B636F6D70".to_string(),
                name: "TrackComp".to_string(),
            },
            Plugin {
                guid: "56535455564852757632326872000000".to_string(),
                name: plugin_properties.dither_plugin_name,
            },
            Plugin {
                guid: "56535473796C3173796C656E74683100".to_string(),
                name: "Sylenth1".to_string(),
            },
            Plugin {
                guid: "77BBA7CA90F14C9BB298BA9010D6DD78".to_string(),
                name: "StereoEnhancer".to_string(),
            },
            Plugin {
                guid: "946051208E29496E804F64A825C8A047".to_string(),
                name: "StudioEQ".to_string(),
            },
            Plugin {
                guid: "D39D5B69D6AF42FA1234567868495645".to_string(),
                name: "Hive".to_string(),
            },
        ]);

        if plugin_properties.includes_channel_plugins {
            expected_plugins.push(Plugin {
                guid: "D56B9C6CA4F946018EED73EB83A74B58".to_string(),
                name: "Input Filter".to_string(),
            });
        }

        assert_eq!(project_details.metadata, expected_metadata);
        assert_eq!(actual_plugins_sorted, expected_plugins);
    }

    #[rstest]
    fn get_project_details_sx3() {
        let project_path = PathBuf::from("testdata").join("Example Project (Cubase SX3).cpr");
        let project_bytes = fs::read(project_path).unwrap();

        let reader = Reader::new(&project_bytes);
        let project_details = reader.get_project_details().unwrap();

        assert_eq!(
            project_details.metadata,
            Metadata {
                application: "Cubase SX".to_string(),
                version: "3.1.1".to_string(),
                release_date: "Oct 13 2005".to_string(),
                architecture: "Unspecified".to_string()
            }
        );
        assert!(project_details.plugins.is_empty());
    }

    #[rstest]
    #[case::application("Truncated Project (Application).cpr", Error::NoApplication)]
    #[case::version("Truncated Project (Version).cpr", Error::NoVersion)]
    #[case::release_date("Truncated Project (Release Date).cpr", Error::NoReleaseDate)]
    #[case::plugin_guid("Truncated Project (Plugin GUID).cpr", Error::NoPluginGUID)]
    #[case::plugin_name_tag("Truncated Project (Plugin Name Tag).cpr", Error::NoPluginName)]
    #[case::plugin_name_value("Truncated Project (Plugin Name Value).cpr", Error::NoPluginName)]
    #[case::tag_after_plugin_name(
        "Truncated Project (Tag After Plugin Name).cpr",
        Error::NoTokenAfterPluginName
    )]
    #[case::original_plugin_name(
        "Truncated Project (Original Plugin Name).cpr",
        Error::NoOriginalPluginName
    )]
    fn get_project_details_truncated(#[case] filename: &str, #[case] expected_error: Error) {
        let project_path = PathBuf::from("testdata").join(filename);
        let project_bytes = fs::read(project_path).unwrap();

        let reader = Reader::new(&project_bytes);
        let project_details = reader.get_project_details();

        assert_eq!(project_details, Err(expected_error));
    }

    #[rstest]
    fn get_project_details_invalid_project() {
        let project_bytes = Vec::new();

        let reader = Reader::new(&project_bytes);
        let project_details = reader.get_project_details();

        assert_eq!(project_details, Err(Error::CorruptProject));
    }
}
