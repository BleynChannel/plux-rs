use std::{fmt::Display, path::PathBuf};

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::{Bundle, Plugin};

/// Complete information about a plugin.
///
/// PluginInfo combines the plugin's filesystem location, bundle metadata, and
/// dependency information into a single structure used throughout the Plux system.
///
/// # Type Parameters
///
/// * `I` - The type implementing the Info trait (contains dependency information)
///
/// # Fields
///
/// * `path` - Filesystem path to the plugin file or directory
/// * `bundle` - Bundle metadata (id, version, format)
/// * `info` - Dependency and configuration information
pub struct PluginInfo<I: Info> {
    /// Filesystem path to the plugin
    pub path: PathBuf,
    /// Bundle metadata for the plugin
    pub bundle: Bundle,
    /// Dependency and configuration information
    pub info: I,
}

/// Trait for plugin information and dependencies.
///
/// The Info trait defines the interface for accessing a plugin's dependency information.
/// Implementations of this trait provide details about what other plugins this plugin
/// depends on for its operation.
///
/// # Required Methods
///
/// * `depends` - Returns the list of required dependencies
/// * `optional_depends` - Returns the list of optional dependencies
///
/// # Example
///
/// ```rust
/// use plux_rs::{Info, Depend};
///
/// struct MyInfo {
///     required: Vec<Depend>,
///     optional: Vec<Depend>,
/// }
///
/// impl Info for MyInfo {
///     fn depends(&self) -> &Vec<Depend> {
///         &self.required
///     }
///
///     fn optional_depends(&self) -> &Vec<Depend> {
///         &self.optional
///     }
/// }
/// ```
pub trait Info: Send + Sync {
    /// Returns the list of required dependencies for this plugin.
    ///
    /// Required dependencies must be available and loaded for the plugin to function.
    ///
    /// # Returns
    ///
    /// Returns a reference to a vector of required dependencies.
    fn depends(&self) -> &Vec<Depend>;

    /// Returns the list of optional dependencies for this plugin.
    ///
    /// Optional dependencies enhance functionality but are not required for basic operation.
    ///
    /// # Returns
    ///
    /// Returns a reference to a vector of optional dependencies.
    fn optional_depends(&self) -> &Vec<Depend>;
}

/// Represents a dependency on another plugin.
///
/// A Depend specifies a plugin that must be available, including both the plugin's
/// identifier and the acceptable version range.
///
/// # Fields
///
/// * `id` - The identifier of the required plugin
/// * `version` - Version requirement specifying acceptable versions
///
/// # Examples
///
/// ```rust
/// use plux_rs::Depend;
/// use semver::VersionReq;
///
/// // Require any version 1.x of the "logger" plugin
/// let dep1 = Depend::new("logger".to_string(), VersionReq::parse("1.*").unwrap());
///
/// // Require exactly version 2.0.0 of the "database" plugin
/// let dep2 = Depend::new("database".to_string(), VersionReq::parse("=2.0.0").unwrap());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Depend {
    /// The identifier of the required plugin
    pub id: String,
    /// Version requirement for the dependency
    pub version: VersionReq,
}

/// Standard implementation of the Info trait.
///
/// StdInfo provides a basic implementation of plugin information with lists of
/// required and optional dependencies. This is the default implementation used
/// by most plugin managers.
///
/// # Fields
///
/// * `depends` - List of plugins required for this plugin to function
/// * `optional_depends` - List of plugins that enhance functionality but are not required
///
/// # Examples
///
/// ```rust
/// use plux_rs::{StdInfo, Depend};
/// use semver::VersionReq;
///
/// let mut info = StdInfo::new();
/// info.depends.push(Depend::new("core".to_string(), VersionReq::parse("1.0").unwrap()));
/// info.optional_depends.push(Depend::new("ui".to_string(), VersionReq::parse("2.*").unwrap()));
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct StdInfo {
    /// Required dependencies for this plugin
    pub depends: Vec<Depend>,
    /// Optional dependencies that enhance functionality
    pub optional_depends: Vec<Depend>,
}

impl Depend {
    /// Creates a new dependency specification.
    ///
    /// # Parameters
    ///
    /// * `name` - The identifier of the required plugin
    /// * `version` - Version requirement for the dependency
    ///
    /// # Returns
    ///
    /// Returns a new Depend instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use plux_rs::Depend;
    /// use semver::VersionReq;
    ///
    /// let dependency = Depend::new("logger".to_string(), VersionReq::parse("1.0").unwrap());
    /// ```
    pub const fn new(name: String, version: VersionReq) -> Self {
        Self { id: name, version }
    }
}

impl Display for Depend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.id, self.version)
    }
}

impl<ID: AsRef<str>> PartialEq<(ID, &Version)> for Depend {
    fn eq(&self, (id, version): &(ID, &Version)) -> bool {
        self.id == id.as_ref() && self.version.matches(*version)
    }
}

impl PartialEq<Bundle> for Depend {
    fn eq(&self, Bundle { id, version, .. }: &Bundle) -> bool {
        self.id == *id && self.version.matches(version)
    }
}

impl<O: Send + Sync, I: Info> PartialEq<Plugin<'_, O, I>> for Depend {
    fn eq(&self, other: &Plugin<'_, O, I>) -> bool {
        self.id == other.info.bundle.id && self.version.matches(&other.info.bundle.version)
    }
}

impl StdInfo {
    /// Creates a new StdInfo instance with no dependencies.
    ///
    /// # Returns
    ///
    /// Returns a new StdInfo with empty dependency lists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use plux_rs::StdInfo;
    ///
    /// let info = StdInfo::new();
    /// assert!(info.depends.is_empty());
    /// assert!(info.optional_depends.is_empty());
    /// ```
    pub const fn new() -> Self {
        Self {
            depends: vec![],
            optional_depends: vec![],
        }
    }
}

impl Info for StdInfo {
    fn depends(&self) -> &Vec<Depend> {
        &self.depends
    }

    fn optional_depends(&self) -> &Vec<Depend> {
        &self.optional_depends
    }
}

impl Display for StdInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Dependencies: {};{}Optional dependencies: {}",
            self.depends
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            f.alternate().then_some('\n').unwrap_or(' '),
            self.optional_depends
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
