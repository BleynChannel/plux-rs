use semver::Version;
use std::{
    error::Error as StdError,
    fmt::{Debug, Display},
};
use thiserror::Error;

use crate::{Bundle, Depend};

/// Errors that can occur when parsing a bundle from a filename.
///
/// This error type is returned by `Bundle::from_filename()` when the filename
/// doesn't match the expected format `{id}-v{version}.{format}`.
#[derive(Error, Debug)]
pub enum BundleFromError {
    /// Failed to convert OsStr to UTF-8 string
    #[error("Error converting OsStr to str")]
    OsStrToStrFailed,
    /// Could not extract the plugin ID from the filename
    #[error("Failed to get ID")]
    IDFailed,
    /// Could not extract the version from the filename
    #[error("Failed to get version")]
    VersionFailed,
    /// Could not extract the format from the filename
    #[error("Failed to get format")]
    FormatFailed,
    /// The version string is not a valid semantic version
    #[error("Failed to parse version")]
    ParseVersion(#[from] semver::Error),
}

/// Errors that can occur when creating a ZIP archive from a plugin bundle.
///
/// This error type is returned by the `zip` function when creating plugin archives.
#[cfg(feature = "archive")]
#[derive(Error, Debug)]
pub enum BundleZipError {
    /// The bundle has no name
    #[error("Bundle has no name")]
    NoNameFailed,
    /// The bundle is missing
    #[error("Missing bundle")]
    MissingBundleFailed,
    /// The directory contains a directory with the same name as the bundle
    #[error("The directory contains a directory with the same name as the bundle")]
    ContainSameDirFailed,
    /// Failed to create the bundle archive
    #[error("Failed to create bundle")]
    CreateBundleFailed(std::io::Error),
    /// Failed to open a file in the bundle
    #[error("Failed to open file in bundle")]
    OpenFileInBundleFailed(#[from] std::io::Error),
    /// Failed to create the ZIP archive
    #[error("Failed to zip")]
    ZipFailed(#[from] zip::result::ZipError),
}

/// Errors that can occur when extracting a ZIP archive to a plugin bundle.
///
/// This error type is returned by the `unzip` function when extracting plugin archives.
#[cfg(feature = "archive")]
#[derive(Error, Debug)]
pub enum BundleUnzipError {
    /// The bundle has no name
    #[error("Bundle has no name")]
    NoNameFailed,
    /// The bundle is missing
    #[error("Missing bundle")]
    MissingBundleFailed,
    /// The directory contains a file with the same name as the bundle
    #[error("The directory contains a file with the same name as the bundle")]
    ContainSameFileFailed,
    /// Failed to open the bundle archive
    #[error("Failed to open bundle")]
    OpenBundleFailed(#[from] std::io::Error),
    /// Failed to extract the ZIP archive
    #[error("Failed to unzip")]
    UnzipFailed(#[from] zip::result::ZipError),
    /// Error creating BundleInfo from the extracted files
    #[error("Error creating BundleInfo")]
    BundleFromFailed(#[from] BundleFromError),
}

/// Errors that can occur when stopping the plugin loader.
///
/// This error type is returned by `Loader::stop()` when cleanup operations fail.
#[derive(Error, Debug)]
pub enum StopLoaderError {
    /// Failed to unregister one or more plugins
    #[error("Failed to unregister plugins `{0:?}`")]
    UnregisterPluginFailed(Vec<UnregisterPluginError>),
    /// Failed to unregister one or more managers
    #[error("Failed to unregister managers `{0:?}`")]
    UnregisterManagerFailed(Vec<UnregisterManagerError>),
}

/// Errors that can occur when registering a plugin manager.
///
/// This error type is returned by manager registration operations.
#[derive(Error, Debug)]
pub enum RegisterManagerError {
    /// A manager with the same format is already registered
    #[error("Format `{0}` is already occupied")]
    AlreadyOccupiedFormat(String),
    /// The manager itself returned an error during registration
    #[error("Manager registration error by the manager")]
    RegisterManagerByManager(#[from] Box<dyn StdError + Send + Sync>),
}

/// Errors that can occur when unregistering a plugin manager.
///
/// This error type is returned by manager unregistration operations.
#[derive(Error, Debug)]
pub enum UnregisterManagerError {
    /// The manager was not found
    #[error("Not found manager")]
    NotFound,
    /// Failed to unregister a plugin during manager unregistration
    #[error("Failed to unregister plugin")]
    UnregisterPlugin(#[from] UnregisterPluginError),
    /// The manager itself returned an error during unregistration
    #[error("Manager unregistration error by the manager")]
    UnregisterManagerByManager(#[from] Box<dyn StdError + Send + Sync>),
}

/// Errors that can occur when registering a plugin.
///
/// This error type is returned by plugin registration operations.
#[derive(Error, Debug)]
pub enum RegisterPluginError {
    /// The plugin was not found
    #[error("Not found plugin")]
    NotFound,
    /// Failed to parse bundle information from the filename
    #[error("Failed to bundle from filename")]
    BundleFromFailed(#[from] BundleFromError),
    /// No manager exists for the plugin's format
    #[error("Unknown plugin manager for the format '{0}'")]
    UnknownManagerFormat(String),
    /// The plugin manager returned an error during registration
    #[error("Plugin registration error by the manager")]
    RegisterPluginByManager(#[from] Box<dyn StdError + Send + Sync>),
    /// A plugin with the same ID and version already exists
    #[error("A plugin with ID `{0}` and version `{1}` already exists")]
    AlreadyExistsIDAndVersion(String, Version),
}

/// Errors that can occur when unregistering a plugin.
///
/// This error type is returned by plugin unregistration operations.
#[derive(Error, Debug)]
pub enum UnregisterPluginError {
    /// The plugin was not found
    #[error("Not found plugin")]
    NotFound,
    /// Failed to unload the plugin during unregistration
    #[error("Plugin unload error")]
    UnloadError(#[from] UnloadPluginError),
    /// The plugin's manager has been unregistered
    #[error("The plugin has an unregistered manager")]
    HasUnregisteredManager,
    /// The plugin manager returned an error during unregistration
    #[error("Plugin unregistration error by the manager")]
    UnregisterPluginByManager(#[from] Box<dyn StdError + Send + Sync>),
}

/// Errors that can occur when loading a plugin.
///
/// This error type is returned by plugin loading operations.
#[derive(Error, Debug)]
pub enum LoadPluginError {
    /// The plugin to load was not found
    #[error("Not found plugin")]
    NotFound,
    /// One or more required dependencies could not be found
    #[error("The following dependencies could not be found: {0:?}")]
    NotFoundDependencies(Vec<Depend>),
    /// A dependency failed to load
    #[error("Dependency `{depend}` returned an error: {error:?}")]
    LoadDependency {
        /// The dependency that failed
        depend: Depend,
        /// The error that occurred while loading the dependency
        error: Box<LoadPluginError>,
    },
    /// The plugin manager returned an error during loading
    #[error("Plugin load error by the manager")]
    LoadPluginByManager(#[from] Box<dyn StdError + Send + Sync>),
    /// The plugin doesn't implement required function requests
    #[error("Requests not found: {0:?}")]
    RequestsNotFound(Vec<String>),
}

/// Errors that can occur when unloading a plugin.
///
/// This error type is returned by plugin unloading operations.
#[derive(Error, Debug)]
pub enum UnloadPluginError {
    /// The plugin was not found
    #[error("Not found plugin")]
    NotFound,
    /// The plugin is currently used as a dependency by another plugin
    #[error("The plugin `{plugin}` currently uses the plugin `{depend}` as a dependency")]
    CurrentlyUsesDepend { 
        /// The plugin that depends on the one being unloaded
        plugin: Bundle, 
        /// The dependency that is being unloaded
        depend: Bundle 
    },
    /// The plugin manager returned an error during unloading
    #[error("Plugin unload error by the manager")]
    UnloadPluginByManager(#[from] Box<dyn StdError + Send + Sync>),
}

/// Errors that can occur when registering a function request in a plugin.
///
/// This error type is returned when validating and registering function requests.
#[derive(Error, Debug)]
pub enum RegisterRequestError {
    /// The requested function was not found
    #[error("Function not found")]
    NotFound,
    /// The function arguments are incorrectly specified
    #[error("The arguments are set incorrectly")]
    ArgumentsIncorrectly,
}

/// Errors that can occur when calling a plugin request.
///
/// This error type is returned when attempting to call a function request on a plugin.
#[derive(Error, Debug)]
pub enum PluginCallRequestError {
    /// The requested function was not found in the plugin
    #[error("Request not found")]
    NotFound,
}

/// Errors that can occur when registering a function in a plugin.
///
/// This error type is returned when attempting to register a function in a plugin's registry.
#[derive(Error, Debug)]
pub enum PluginRegisterFunctionError {
    /// A function with the same name already exists in the registry
    #[error("Function {0} already exists")]
    AlreadyExists(String),
}

/// Errors that can occur when calling a function in a plugin.
///
/// This error type is returned when attempting to call a function in a plugin's registry.
#[derive(Error, Debug)]
pub enum PluginCallFunctionError {
    /// The requested function was not found in the plugin's registry
    #[error("Function not found")]
    NotFound,
}

/// Errors that can occur when calling a function on a plugin dependency.
///
/// This error type is returned when attempting to call a function on a plugin's dependency.
#[derive(Error, Debug)]
pub enum CallFunctionDependError {
    /// The required dependency was not found
    #[error("Depend not found")]
    DependNotFound,
    /// Failed to call the function on the dependency
    #[error("Failed to call function")]
    FailedCallFunction(#[from] PluginCallFunctionError),
}

/// Result type for manager operations.
///
/// This type alias is used throughout the plugin system for operations that can fail.
/// It provides a consistent error handling interface for manager implementations.
pub type ManagerResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Error that occurs when parsing a Variable into a specific type.
///
/// This error is returned when attempting to convert a Variable to a specific
/// Rust type, but the conversion fails due to a type mismatch.
#[derive(Debug)]
pub struct ParseVariableError {
    ty: &'static str,
}

impl ParseVariableError {
    /// Creates a new ParseVariableError with the specified type name.
    ///
    /// # Parameters
    ///
    /// * `ty` - The name of the type that the conversion failed for
    ///
    /// # Returns
    ///
    /// Returns a new ParseVariableError instance.
    pub fn new(ty: &'static str) -> Self {
        Self { ty }
    }
}

impl Display for ParseVariableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "data cannot be converted to this type `{}`", self.ty)
    }
}

impl StdError for ParseVariableError {}
