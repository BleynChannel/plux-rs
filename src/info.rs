use std::{fmt::Display, path::PathBuf};

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::{Bundle, Plugin};

pub struct PluginInfo<I: Info> {
    pub path: PathBuf,
    pub bundle: Bundle,
    pub info: I,
}

pub trait Info: Send + Sync {
    fn depends(&self) -> &Vec<Depend>;
    fn optional_depends(&self) -> &Vec<Depend>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Depend {
    pub id: String,
    pub version: VersionReq,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct StdInfo {
    pub depends: Vec<Depend>,
    pub optional_depends: Vec<Depend>,
}

impl Depend {
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
