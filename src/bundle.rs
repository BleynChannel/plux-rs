use std::{cmp::Ordering, ffi::OsStr, fmt::Display};

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::{Depend, Info, Plugin, utils::BundleFromError};

/// Represents a plugin bundle with its metadata.
///
/// A Bundle contains the essential information needed to identify and manage a plugin,
/// including its unique identifier, version, and format. This information is used throughout
/// the Plux system for plugin discovery, dependency resolution, and lifecycle management.
///
/// # Fields
///
/// * `id` - Unique identifier for the plugin (e.g., "calculator", "logger")
/// * `version` - Semantic version of the plugin (e.g., "1.0.0")
/// * `format` - File format/extension of the plugin (e.g., "lua", "rs", "wasm")
///
/// # Format
///
/// Plugin bundles follow the naming convention: `{id}-v{version}.{format}`
/// For example: `calculator-v1.0.0.lua` or `renderer-v2.1.0.wasm`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct Bundle {
    /// Unique identifier for the plugin
    pub id: String,
    /// Semantic version of the plugin
    pub version: Version,
    /// File format/extension of the plugin
    pub format: String,
}

impl Bundle {
    /// Creates a Bundle from a filename string.
    ///
    /// Parses a plugin filename following the standard Plux naming convention
    /// `{id}-v{version}.{format}` and extracts the bundle information.
    ///
    /// # Parameters
    ///
    /// * `filename` - The filename to parse (e.g., "calculator-v1.0.0.lua")
    ///
    /// # Returns
    ///
    /// Returns `Result<Self, BundleFromError>` containing the parsed Bundle on success,
    /// or an error if the filename doesn't match the expected format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use plux_rs::Bundle;
    ///
    /// let bundle = Bundle::from_filename("my_plugin-v1.2.3.lua")?;
    /// assert_eq!(bundle.id, "my_plugin");
    /// assert_eq!(bundle.version.to_string(), "1.2.3");
    /// assert_eq!(bundle.format, "lua");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The filename cannot be converted to a string
    /// - The filename doesn't contain a format extension
    /// - The filename doesn't contain a version marker "-v"
    /// - The ID, version, or format parts are empty
    /// - The version string is not a valid semantic version
    pub fn from_filename<S>(filename: &S) -> Result<Self, BundleFromError>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        let mut path = filename
            .as_ref()
            .to_str()
            .ok_or(BundleFromError::OsStrToStrFailed)?
            .to_string();

        let format = path
            .drain(path.rfind('.').ok_or(BundleFromError::FormatFailed)? + 1..)
            .collect::<String>();
        let version = path
            .drain(path.rfind("-v").ok_or(BundleFromError::VersionFailed)? + 2..path.len() - 1)
            .collect::<String>();
        let id = path
            .drain(..path.rfind("-v").ok_or(BundleFromError::IDFailed)?)
            .collect::<String>();

        if format.is_empty() {
            return Err(BundleFromError::FormatFailed);
        }
        if version.is_empty() {
            return Err(BundleFromError::VersionFailed);
        }
        if id.is_empty() {
            return Err(BundleFromError::IDFailed);
        }

        Ok(Self {
            id,
            version: Version::parse(version.as_str())?,
            format,
        })
    }
}

impl<ID: AsRef<str>> PartialEq<(ID, &Version)> for Bundle {
    fn eq(&self, (id, version): &(ID, &Version)) -> bool {
        self.id == *id.as_ref() && self.version == **version
    }
}

impl<O: Send + Sync, I: Info> PartialEq<Plugin<'_, O, I>> for Bundle {
    fn eq(&self, other: &Plugin<'_, O, I>) -> bool {
        self.id == other.info.bundle.id && self.version == other.info.bundle.version
    }
}

impl PartialEq<Depend> for Bundle {
    fn eq(&self, Depend { id: name, version }: &Depend) -> bool {
        self.id == *name && version.matches(&self.version)
    }
}

impl<ID: AsRef<str>> PartialOrd<(ID, &Version)> for Bundle {
    fn partial_cmp(&self, (id, version): &(ID, &Version)) -> Option<Ordering> {
        match self.id == *id.as_ref() {
            true => self.version.partial_cmp(*version),
            false => None,
        }
    }
}

impl<O: Send + Sync, I: Info> PartialOrd<Plugin<'_, O, I>> for Bundle {
    fn partial_cmp(&self, other: &Plugin<'_, O, I>) -> Option<Ordering> {
        match self.id == other.info.bundle.id {
            true => self.version.partial_cmp(&other.info.bundle.version),
            false => None,
        }
    }
}

impl Display for Bundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-v{}.{}", self.id, self.version, self.format)
    }
}
