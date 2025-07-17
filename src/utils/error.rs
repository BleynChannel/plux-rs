use semver::Version;
use std::{
    error::Error as StdError,
    fmt::{Debug, Display},
};
use thiserror::Error;

use crate::{Bundle, Depend};

#[derive(Error, Debug)]
pub enum BundleFromError {
    #[error("Error converting OsStr to str")]
    OsStrToStrFailed,
    #[error("Failed to get ID")]
    IDFailed,
    #[error("Failed to get version")]
    VersionFailed,
    #[error("Failed to get format")]
    FormatFailed,
    #[error("Failed to parse version")]
    ParseVersion(#[from] semver::Error),
}

#[cfg(feature = "archive")]
#[derive(Error, Debug)]
pub enum BundleZipError {
    #[error("Bundle has no name")]
    NoNameFailed,
    #[error("Missing bundle")]
    MissingBundleFailed,
    #[error("The directory contains a directory with the same name as the bundle")]
    ContainSameDirFailed,
    #[error("Failed to create bundle")]
    CreateBundleFailed(std::io::Error),
    #[error("Failed to open file in bundle")]
    OpenFileInBundleFailed(#[from] std::io::Error),
    #[error("Failed to zip")]
    ZipFailed(#[from] zip::result::ZipError),
}

#[cfg(feature = "archive")]
#[derive(Error, Debug)]
pub enum BundleUnzipError {
    #[error("Bundle has no name")]
    NoNameFailed,
    #[error("Missing bundle")]
    MissingBundleFailed,
    #[error("The directory contains a file with the same name as the bundle")]
    ContainSameFileFailed,
    #[error("Failed to open bundle")]
    OpenBundleFailed(#[from] std::io::Error),
    #[error("Failed to unzip")]
    UnzipFailed(#[from] zip::result::ZipError),
    #[error("Error creating BundleInfo")]
    BundleFromFailed(#[from] BundleFromError),
}

#[derive(Error, Debug)]
pub enum StopLoaderError {
    #[error("Failed to unregister plugins `{0:?}`")]
    UnregisterPluginFailed(Vec<UnregisterPluginError>),
    #[error("Failed to unregister managers `{0:?}`")]
    UnregisterManagerFailed(Vec<UnregisterManagerError>),
}

#[derive(Error, Debug)]
pub enum RegisterManagerError {
    #[error("Format `{0}` is already occupied")]
    AlreadyOccupiedFormat(String),
    #[error("Manager registration error by the manager")]
    RegisterManagerByManager(#[from] Box<dyn StdError + Send + Sync>),
}

#[derive(Error, Debug)]
pub enum UnregisterManagerError {
    #[error("Not found manager")]
    NotFound,
    #[error("Failed to unregister plugin")]
    UnregisterPlugin(#[from] UnregisterPluginError),
    #[error("Manager unregistration error by the manager")]
    UnregisterManagerByManager(#[from] Box<dyn StdError + Send + Sync>),
}

#[derive(Error, Debug)]
pub enum RegisterPluginError {
    #[error("Not found plugin")]
    NotFound,
    #[error("Failed to bundle from filename")]
    BundleFromFailed(#[from] BundleFromError),
    #[error("Unknown plugin manager for the format '{0}'")]
    UnknownManagerFormat(String),
    #[error("Plugin registration error by the manager")]
    RegisterPluginByManager(#[from] Box<dyn StdError + Send + Sync>),
    #[error("A plugin with ID `{0}` and version `{1}` already exists")]
    AlreadyExistsIDAndVersion(String, Version),
}

#[derive(Error, Debug)]
pub enum UnregisterPluginError {
    #[error("Not found plugin")]
    NotFound,
    #[error("Plugin unload error")]
    UnloadError(#[from] UnloadPluginError),
    #[error("The plugin has an unregistered manager")]
    HasUnregisteredManager,
    #[error("Plugin unregistration error by the manager")]
    UnregisterPluginByManager(#[from] Box<dyn StdError + Send + Sync>),
}

#[derive(Error, Debug)]
pub enum LoadPluginError {
    #[error("Not found plugin")]
    NotFound,
    #[error("The following dependencies could not be found: {0:?}")]
    NotFoundDependencies(Vec<Depend>),
    #[error("Dependency `{depend}` returned an error: {error:?}")]
    LoadDependency {
        depend: Depend,
        error: Box<LoadPluginError>,
    },
    #[error("Plugin load error by the manager")]
    LoadPluginByManager(#[from] Box<dyn StdError + Send + Sync>),
    #[error("Requests not found: {0:?}")]
    RequestsNotFound(Vec<String>),
}

#[derive(Error, Debug)]
pub enum UnloadPluginError {
    #[error("Not found plugin")]
    NotFound,
    #[error("The plugin `{plugin}` currently uses the plugin `{depend}` as a dependency")]
    CurrentlyUsesDepend { plugin: Bundle, depend: Bundle },
    #[error("Plugin unload error by the manager")]
    UnloadPluginByManager(#[from] Box<dyn StdError + Send + Sync>),
}

#[derive(Error, Debug)]
pub enum RegisterRequestError {
    #[error("Function not found")]
    NotFound,
    #[error("The arguments are set incorrectly")]
    ArgumentsIncorrectly,
}

#[derive(Error, Debug)]
pub enum PluginCallRequestError {
    #[error("Request not found")]
    NotFound,
}

#[derive(Error, Debug)]
pub enum PluginRegisterFunctionError {
    #[error("Function {0} already exists")]
    AlreadyExists(String),
}

#[derive(Error, Debug)]
pub enum PluginCallFunctionError {
    #[error("Function not found")]
    NotFound,
}

#[derive(Error, Debug)]
pub enum CallFunctionDependError {
    #[error("Depend not found")]
    DependNotFound,
    #[error("Failed to call function")]
    FailedCallFunction(#[from] PluginCallFunctionError),
}

pub type ManagerResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct ParseVariableError {
    ty: &'static str,
}

impl ParseVariableError {
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
