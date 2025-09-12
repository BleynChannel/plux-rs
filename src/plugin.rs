use std::{cmp::Ordering, fmt::Debug, sync::Arc};

use semver::Version;

use crate::{
    Bundle, Depend, Info, Manager, PluginInfo, Registry,
    function::Function,
    utils::{PluginCallFunctionError, PluginCallRequestError, PluginRegisterFunctionError, Ptr},
    variable::Variable,
};

/// Represents a loaded plugin instance.
///
/// A Plugin encapsulates all the information and functionality related to a single plugin,
/// including its metadata, execution state, and available functions.
///
/// # Type Parameters
///
/// * `'a` - Lifetime parameter for references within the plugin
/// * `O` - Output type for plugin functions (must implement Send + Sync)
/// * `I` - Plugin information type (must implement Info trait)
///
/// # Fields
///
/// * `manager` - Reference to the manager responsible for this plugin
/// * `info` - Plugin metadata and configuration
/// * `is_load` - Whether the plugin is currently loaded and ready for execution
/// * `requests` - Functions that this plugin must implement at the request of the host
/// * `registry` - Functions exposed by this plugin to other plugins or the host
pub struct Plugin<'a, O: Send + Sync, I: Info> {
    pub(crate) manager: Ptr<'a, Box<dyn Manager<'a, O, I>>>,
    pub(crate) info: PluginInfo<I>,
    pub(crate) is_load: bool,
    pub(crate) requests: Vec<Box<dyn Function<Output = O>>>,
    pub(crate) registry: Registry<O>,
}

impl<'a, O: Send + Sync, I: Info> Plugin<'a, O, I> {
    /// Creates a new plugin instance.
    ///
    /// This is an internal constructor used by the loader when registering plugins.
    ///
    /// # Parameters
    ///
    /// * `manager` - Reference to the manager responsible for this plugin
    /// * `info` - Plugin metadata and configuration
    ///
    /// # Returns
    ///
    /// Returns a new Plugin instance with default unloaded state.
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

    /// Returns information about this plugin.
    ///
    /// # Returns
    ///
    /// Returns a reference to the plugin's metadata and configuration.
    pub const fn info(&self) -> &PluginInfo<I> {
        &self.info
    }

    /// Checks if the plugin is currently loaded and ready for execution.
    ///
    /// # Returns
    ///
    /// Returns `true` if the plugin is loaded, `false` otherwise.
    pub const fn is_load(&self) -> bool {
        self.is_load
    }

    /// Returns the list of function requests this plugin must implement.
    ///
    /// Function requests are functions that this plugin must implement at the request of the host.
    /// These are functions that the host can call on this plugin when needed.
    ///
    /// # Returns
    ///
    /// Returns a reference to the vector of function requests.
    pub const fn get_requests(&self) -> &Vec<Box<dyn Function<Output = O>>> {
        &self.requests
    }

    /// Calls a function request by name with the given arguments.
    ///
    /// This method searches through the plugin's requests and executes the one matching
    /// the provided name. These are functions that the plugin implements for the host to call.
    ///
    /// # Parameters
    ///
    /// * `name` - Name of the function request to call
    /// * `args` - Arguments to pass to the function
    ///
    /// # Returns
    ///
    /// Returns `Result<O, PluginCallRequestError>` containing the function result on success,
    /// or an error if the request is not found.
    pub fn call_request(&self, name: &str, args: &[Variable]) -> Result<O, PluginCallRequestError> {
        self.requests
            .iter()
            .find_map(|request| match request.name() == name {
                true => Some(request.call(args)),
                false => None,
            })
            .ok_or(PluginCallRequestError::NotFound)
    }

    /// Returns the registry of functions exposed by this plugin.
    ///
    /// The registry contains functions that this plugin makes available to other plugins
    /// or the host application.
    ///
    /// # Returns
    ///
    /// Returns a reference to the function registry.
    pub const fn get_registry(&self) -> &Registry<O> {
        &self.registry
    }

    /// Registers a new function in this plugin's registry.
    ///
    /// This method adds a function to the plugin's registry, making it available for
    /// other plugins or the host to call.
    ///
    /// # Parameters
    ///
    /// * `function` - The function to register
    ///
    /// # Returns
    ///
    /// Returns `Result<(), PluginRegisterFunctionError>` indicating success or failure.
    /// Fails if a function with the same name is already registered.
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

    /// Calls a function from this plugin's registry by name.
    ///
    /// This method searches through the plugin's registry and executes the function
    /// matching the provided name.
    ///
    /// # Parameters
    ///
    /// * `name` - Name of the function to call
    /// * `args` - Arguments to pass to the function
    ///
    /// # Returns
    ///
    /// Returns `Result<O, PluginCallFunctionError>` containing the function result on success,
    /// or an error if the function is not found.
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
