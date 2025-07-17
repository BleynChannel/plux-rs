use std::{cmp::Ordering, ffi::OsStr, fmt::Display};

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::{utils::BundleFromError, Depend, Info, Plugin};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct Bundle {
    pub id: String,
    pub version: Version,
    pub format: String,
}

impl Bundle {
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
