use std::{cmp::Ordering, fmt::Debug, sync::Arc};

use semver::Version;

use crate::{
    function::Function,
    utils::{PluginCallFunctionError, PluginCallRequestError, PluginRegisterFunctionError, Ptr},
    variable::Variable,
    Bundle, Depend, Info, Manager, PluginInfo, Registry,
};

pub struct Plugin<'a, O: Send + Sync, I: Info> {
    pub(crate) manager: Ptr<'a, Box<dyn Manager<'a, O, I>>>,
    pub(crate) info: PluginInfo<I>,
    pub(crate) is_load: bool,
    pub(crate) requests: Vec<Box<dyn Function<Output = O>>>,
    pub(crate) registry: Registry<O>,
}

impl<'a, O: Send + Sync, I: Info> Plugin<'a, O, I> {
    pub(crate) const fn new(
        manager: Ptr<'a, Box<dyn Manager<'a, O, I>>>,
        info: PluginInfo<I>,
    ) -> Self {
        Self {
            manager,
            info,
            is_load: false,
            requests: vec![],
            registry: vec![],
        }
    }

    pub const fn info(&self) -> &PluginInfo<I> {
        &self.info
    }

    pub const fn is_load(&self) -> bool {
        self.is_load
    }

    pub const fn get_requests(&self) -> &Vec<Box<dyn Function<Output = O>>> {
        &self.requests
    }

    pub fn call_request(&self, name: &str, args: &[Variable]) -> Result<O, PluginCallRequestError> {
        self.requests
            .iter()
            .find_map(|request| match request.name() == name {
                true => Some(request.call(args)),
                false => None,
            })
            .ok_or(PluginCallRequestError::NotFound)
    }

    pub const fn get_registry(&self) -> &Registry<O> {
        &self.registry
    }

    pub fn register_function<F>(&mut self, function: F) -> Result<(), PluginRegisterFunctionError>
    where
        F: Function<Output = O> + 'static,
    {
        let find_function = self
            .registry
            .iter()
            .find(|&f| f.as_ref() == &function as &dyn Function<Output = O>);

        match find_function {
            Some(_) => Err(PluginRegisterFunctionError::AlreadyExists(function.name())),
            None => Ok(self.registry.push(Arc::new(function))),
        }
    }

    pub fn call_function(
        &self,
        name: &str,
        args: &[Variable],
    ) -> Result<O, PluginCallFunctionError> {
        self.registry
            .iter()
            .find_map(|function| match function.name() == name {
                true => Some(function.call(args)),
                false => None,
            })
            .ok_or(PluginCallFunctionError::NotFound)
    }
}

impl<O: Send + Sync, I: Info> PartialEq for Plugin<'_, O, I> {
    fn eq(&self, other: &Self) -> bool {
        self.info.bundle.id == other.info.bundle.id
            && self.info.bundle.version == other.info.bundle.version
    }
}

impl<O: Send + Sync, I: Info, ID: AsRef<str>> PartialEq<(ID, &Version)> for Plugin<'_, O, I> {
    fn eq(&self, (id, version): &(ID, &Version)) -> bool {
        self.info.bundle.id == *id.as_ref() && self.info.bundle.version == **version
    }
}

impl<O: Send + Sync, I: Info> PartialEq<Bundle> for Plugin<'_, O, I> {
    fn eq(&self, Bundle { id, version, .. }: &Bundle) -> bool {
        self.info.bundle.id == *id && self.info.bundle.version == *version
    }
}

impl<O: Send + Sync, I: Info> PartialEq<Depend> for Plugin<'_, O, I> {
    fn eq(&self, Depend { id: name, version }: &Depend) -> bool {
        self.info.bundle.id == *name && version.matches(&self.info.bundle.version)
    }
}

impl<O: Send + Sync, I: Info> Eq for Plugin<'_, O, I> {}

impl<O: Send + Sync, I: Info> PartialOrd for Plugin<'_, O, I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.info.bundle.id == other.info.bundle.id {
            true => self
                .info
                .bundle
                .version
                .partial_cmp(&other.info.bundle.version),
            false => None,
        }
    }
}

impl<O: Send + Sync, I: Info, ID: AsRef<str>> PartialOrd<(ID, &Version)> for Plugin<'_, O, I> {
    fn partial_cmp(&self, (id, version): &(ID, &Version)) -> Option<Ordering> {
        match self.info.bundle.id == *id.as_ref() {
            true => self.info.bundle.version.partial_cmp(*version),
            false => None,
        }
    }
}

impl<O: Send + Sync, I: Info> PartialOrd<Bundle> for Plugin<'_, O, I> {
    fn partial_cmp(&self, Bundle { id, version, .. }: &Bundle) -> Option<Ordering> {
        match self.info.bundle.id == *id {
            true => self.info.bundle.version.partial_cmp(version),
            false => None,
        }
    }
}

impl<O: Send + Sync, I: Info> Debug for Plugin<'_, O, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plugin")
            .field("id", &self.info.bundle.id)
            .field("version", &self.info.bundle.version)
            .field("format", &self.info.bundle.format)
            .field("path", &self.info.path)
            .field("is_load", &self.is_load)
            .field("depends", self.info.info.depends())
            .field("optional_depends", self.info.info.optional_depends())
            .finish()
    }
}
