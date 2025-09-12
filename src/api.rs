use rayon::prelude::IntoParallelIterator;
use semver::Version;

use crate::{
    Bundle, Info, Loader, Manager, Plugin, Registry, Requests,
    utils::{
        CallFunctionDependError, LoadPluginError, PluginCallFunctionError, PluginCallRequestError,
        Ptr, RegisterManagerError, RegisterPluginError, UnloadPluginError, UnregisterManagerError,
        UnregisterPluginError,
    },
    variable::Variable,
};

/// API interface provided to plugins during loading.
///
/// The Api struct provides a controlled interface for plugins to interact with the Plux system.
/// It allows plugins to access the loader's functionality while maintaining security boundaries
/// and providing dependency-aware operations.
///
/// # Type Parameters
///
/// * `O` - Output type for plugin functions (must implement Send + Sync + 'static)
/// * `I` - Plugin information type (must implement Info + 'static)
///
/// # Fields
///
/// * `loader` - Reference to the underlying loader
/// * `plugin` - Bundle information for the current plugin
/// * `depends` - List of required dependencies for this plugin
/// * `optional_depends` - List of optional dependencies for this plugin
pub struct Api<O: Send + Sync + 'static, I: Info + 'static> {
    loader: Ptr<'static, Loader<'static, O, I>>,
    plugin: Bundle,
    depends: Vec<Bundle>,
    optional_depends: Vec<Bundle>,
}

impl<O: Send + Sync + 'static, I: Info + 'static> Api<O, I> {
    /// Creates a new API instance for a plugin.
    ///
    /// This is an internal constructor used by the loader when loading plugins.
    ///
    /// # Parameters
    ///
    /// * `loader` - Reference to the loader
    /// * `plugin` - Bundle information for the plugin
    /// * `depends` - Required dependencies
    /// * `optional_depends` - Optional dependencies
    ///
    /// # Returns
    ///
    /// Returns a new Api instance.
    pub(crate) const fn new(
        loader: Ptr<'static, Loader<'static, O, I>>,
        plugin: Bundle,
        depends: Vec<Bundle>,
        optional_depends: Vec<Bundle>,
    ) -> Self {
        Self {
            loader,
            plugin,
            depends,
            optional_depends,
        }
    }

    /// Gets access to the function registry.
    ///
    /// Returns a reference to the registry containing all functions available to plugins.
    ///
    /// # Returns
    ///
    /// Returns `&Registry<O>` containing the function registry.
    pub fn registry(&self) -> &Registry<O> {
        &self.loader.as_ref().registry
    }

    /// Gets information about the current plugin.
    ///
    /// Returns the bundle information for the plugin this API instance belongs to.
    ///
    /// # Returns
    ///
    /// Returns `&Bundle` containing plugin metadata.
    pub const fn plugin(&self) -> &Bundle {
        &self.plugin
    }

    /// Gets the list of required dependencies.
    ///
    /// Returns all dependencies that must be available for this plugin to function.
    ///
    /// # Returns
    ///
    /// Returns `&Vec<Bundle>` containing required dependencies.
    pub const fn depends(&self) -> &Vec<Bundle> {
        &self.depends
    }

    /// Gets the list of optional dependencies.
    ///
    /// Returns all optional dependencies that enhance this plugin's functionality but are not required.
    ///
    /// # Returns
    ///
    /// Returns `&Vec<Bundle>` containing optional dependencies.
    pub const fn optional_depends(&self) -> &Vec<Bundle> {
        &self.optional_depends
    }

    // Loader functions

    /// Registers a plugin manager with the loader.
    ///
    /// This method allows plugins to register new managers during execution.
    ///
    /// # Parameters
    ///
    /// * `manager` - The manager instance to register
    ///
    /// # Returns
    ///
    /// Returns `Result<(), RegisterManagerError>` indicating success or failure.
    ///
    /// # Type Parameters
    ///
    /// * `M` - Type of the manager (must implement Manager trait)
    pub fn register_manager<M>(&self, manager: M) -> Result<(), RegisterManagerError>
    where
        M: Manager<'static, O, I> + 'static,
    {
        self.loader.as_mut().register_manager(manager)
    }

    /// Registers multiple plugin managers with the loader.
    ///
    /// This method allows plugins to register multiple managers in sequence.
    ///
    /// # Parameters
    ///
    /// * `managers` - Iterator of manager instances to register
    ///
    /// # Returns
    ///
    /// Returns `Result<(), RegisterManagerError>` indicating success or failure.
    pub fn register_managers<M>(&self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoIterator<Item = Box<dyn Manager<'static, O, I>>>,
    {
        self.loader.as_mut().register_managers(managers)
    }

    /// Registers multiple plugin managers with the loader in parallel.
    ///
    /// This method allows plugins to register multiple managers concurrently.
    ///
    /// # Parameters
    ///
    /// * `managers` - Parallel iterator of manager instances to register
    ///
    /// # Returns
    ///
    /// Returns `Result<(), RegisterManagerError>` indicating success or failure.
    pub fn par_register_managers<M>(&self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoParallelIterator<Item = Box<dyn Manager<'static, O, I>>>,
    {
        self.loader.as_mut().par_register_managers(managers)
    }

    /// Unregisters a plugin manager from the loader.
    ///
    /// This method allows plugins to unregister managers by format.
    ///
    /// # Parameters
    ///
    /// * `format` - The format of the manager to unregister
    ///
    /// # Returns
    ///
    /// Returns `Result<(), UnregisterManagerError>` indicating success or failure.
    pub fn unregister_manager(&self, format: &str) -> Result<(), UnregisterManagerError> {
        self.loader.as_mut().unregister_manager(format)
    }

    /// Gets an immutable reference to a manager by format.
    ///
    /// This method allows plugins to access registered managers.
    ///
    /// # Parameters
    ///
    /// * `format` - The format of the manager to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Option<&Box<dyn Manager<'static, O, I>>>` containing the manager if found.
    pub fn get_manager_ref(&self, format: &str) -> Option<&Box<dyn Manager<'static, O, I>>> {
        self.loader.as_ref().get_manager_ref(format)
    }

    /// Gets an immutable reference to a manager by format (parallel version).
    ///
    /// This method allows plugins to access registered managers using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `format` - The format of the manager to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Option<&Box<dyn Manager<'static, O, I>>>` containing the manager if found.
    pub fn par_get_manager_ref(&self, format: &str) -> Option<&Box<dyn Manager<'static, O, I>>> {
        self.loader.as_ref().par_get_manager_ref(format)
    }

    /// Gets a mutable reference to a manager by format.
    ///
    /// This method allows plugins to access registered managers for modification.
    ///
    /// # Parameters
    ///
    /// * `format` - The format of the manager to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Option<&mut Box<dyn Manager<'static, O, I>>>` containing the manager if found.
    pub fn get_manager_mut(&self, format: &str) -> Option<&mut Box<dyn Manager<'static, O, I>>> {
        self.loader.as_mut().get_manager_mut(format)
    }

    /// Gets a mutable reference to a manager by format (parallel version).
    ///
    /// This method allows plugins to access registered managers for modification using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `format` - The format of the manager to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Option<&mut Box<dyn Manager<'static, O, I>>>` containing the manager if found.
    pub fn par_get_manager_mut(
        &self,
        format: &str,
    ) -> Option<&mut Box<dyn Manager<'static, O, I>>> {
        self.loader.as_mut().par_get_manager_mut(format)
    }

    /// Registers a plugin with the loader.
    ///
    /// This method allows plugins to register new plugins during execution.
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the plugin file or directory
    ///
    /// # Returns
    ///
    /// Returns `Result<Bundle, RegisterPluginError>` containing the plugin bundle on success.
    pub fn register_plugin(&self, path: &str) -> Result<Bundle, RegisterPluginError> {
        self.loader.as_mut().register_plugin(path)
    }

    /// Registers multiple plugins with the loader.
    ///
    /// This method allows plugins to register multiple plugins in sequence.
    ///
    /// # Parameters
    ///
    /// * `paths` - Iterator of paths to plugin files or directories
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<Bundle>, RegisterPluginError>` containing the plugin bundles on success.
    ///
    /// # Type Parameters
    ///
    /// * `'b` - Lifetime of the path references
    /// * `P` - Type of the iterator containing path references
    pub fn register_plugins<'b, P>(&self, paths: P) -> Result<Vec<Bundle>, RegisterPluginError>
    where
        P: IntoIterator<Item = &'b str>,
    {
        self.loader.as_mut().register_plugins(paths)
    }

    /// Registers multiple plugins with the loader in parallel.
    ///
    /// This method allows plugins to register multiple plugins concurrently.
    ///
    /// # Parameters
    ///
    /// * `paths` - Parallel iterator of paths to plugin files or directories
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<Bundle>, RegisterPluginError>` containing the plugin bundles on success.
    ///
    /// # Type Parameters
    ///
    /// * `'b` - Lifetime of the path references
    /// * `P` - Type of the parallel iterator containing path references
    pub fn par_register_plugins<'b, P>(&self, paths: P) -> Result<Vec<Bundle>, RegisterPluginError>
    where
        P: IntoParallelIterator<Item = &'b str>,
    {
        self.loader.as_mut().par_register_plugins(paths)
    }

    /// Unregisters a plugin from the loader.
    ///
    /// This method allows plugins to unregister plugins by ID and version.
    ///
    /// # Parameters
    ///
    /// * `id` - Plugin identifier
    /// * `version` - Plugin version
    ///
    /// # Returns
    ///
    /// Returns `Result<(), UnregisterPluginError>` indicating success or failure.
    pub fn unregister_plugin(
        &self,
        id: &str,
        version: &Version,
    ) -> Result<(), UnregisterPluginError> {
        self.loader.as_mut().unregister_plugin(id, version)
    }

    /// Unregisters a plugin from the loader by bundle.
    ///
    /// This method allows plugins to unregister plugins by bundle information.
    ///
    /// # Parameters
    ///
    /// * `bundle` - Plugin bundle information
    ///
    /// # Returns
    ///
    /// Returns `Result<(), UnregisterPluginError>` indicating success or failure.
    pub fn unregister_plugin_by_bundle(
        &self,
        bundle: &Bundle,
    ) -> Result<(), UnregisterPluginError> {
        self.loader.as_mut().unregister_plugin_by_bundle(bundle)
    }

    /// Unregisters a plugin from the loader by bundle (parallel version).
    ///
    /// This method allows plugins to unregister plugins by bundle information using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `bundle` - Plugin bundle information
    ///
    /// # Returns
    ///
    /// Returns `Result<(), UnregisterPluginError>` indicating success or failure.
    pub fn par_unregister_plugin_by_bundle(
        &self,
        bundle: &Bundle,
    ) -> Result<(), UnregisterPluginError> {
        self.loader.as_mut().par_unregister_plugin_by_bundle(bundle)
    }

    /// Loads a plugin into the execution environment.
    ///
    /// This method allows plugins to load other plugins by ID and version.
    ///
    /// # Parameters
    ///
    /// * `id` - Plugin identifier
    /// * `version` - Plugin version
    ///
    /// # Returns
    ///
    /// Returns `Result<(), LoadPluginError>` indicating success or failure.
    pub fn load_plugin(&self, id: &str, version: &Version) -> Result<(), LoadPluginError> {
        self.loader.as_mut().load_plugin(id, version)
    }

    /// Loads a plugin into the execution environment (parallel version).
    ///
    /// This method allows plugins to load other plugins by ID and version using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `id` - Plugin identifier
    /// * `version` - Plugin version
    ///
    /// # Returns
    ///
    /// Returns `Result<(), LoadPluginError>` indicating success or failure.
    pub fn par_load_plugin(&self, id: &str, version: &Version) -> Result<(), LoadPluginError> {
        self.loader.as_mut().par_load_plugin(id, version)
    }

    /// Loads a plugin into the execution environment by bundle.
    ///
    /// This method allows plugins to load other plugins by bundle information.
    ///
    /// # Parameters
    ///
    /// * `bundle` - Plugin bundle information
    ///
    /// # Returns
    ///
    /// Returns `Result<(), LoadPluginError>` indicating success or failure.
    pub fn load_plugin_by_bundle(&self, bundle: &Bundle) -> Result<(), LoadPluginError> {
        self.loader.as_mut().load_plugin_by_bundle(bundle)
    }

    /// Loads a plugin into the execution environment by bundle (parallel version).
    ///
    /// This method allows plugins to load other plugins by bundle information using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `bundle` - Plugin bundle information
    ///
    /// # Returns
    ///
    /// Returns `Result<(), LoadPluginError>` indicating success or failure.
    pub fn par_load_plugin_by_bundle(&self, bundle: &Bundle) -> Result<(), LoadPluginError> {
        self.loader.as_mut().par_load_plugin_by_bundle(bundle)
    }

    /// Loads a plugin immediately from the specified path.
    ///
    /// This convenience method allows plugins to register and load a plugin in a single operation.
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the plugin file or directory
    ///
    /// # Returns
    ///
    /// Returns `Result<Bundle, (Option<RegisterPluginError>, Option<LoadPluginError>)>`
    /// containing the plugin bundle on success, or errors from registration or loading.
    pub fn load_plugin_now(
        &self,
        path: &str,
    ) -> Result<Bundle, (Option<RegisterPluginError>, Option<LoadPluginError>)> {
        self.loader.as_mut().load_plugin_now(path)
    }

    /// Loads multiple plugins from the specified paths.
    ///
    /// This method allows plugins to register and load multiple plugins in sequence.
    ///
    /// # Parameters
    ///
    /// * `paths` - Iterator of paths to plugin files or directories
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<Bundle>, (Option<RegisterPluginError>, Option<LoadPluginError>)>`
    /// containing the plugin bundles on success, or errors from registration or loading.
    ///
    /// # Type Parameters
    ///
    /// * `'b` - Lifetime of the path references
    /// * `P` - Type of the iterator containing path references
    pub fn load_plugins<'b, P>(
        &self,
        paths: P,
    ) -> Result<Vec<Bundle>, (Option<RegisterPluginError>, Option<LoadPluginError>)>
    where
        P: IntoIterator<Item = &'b str>,
    {
        self.loader.as_mut().load_plugins(paths)
    }

    /// Loads multiple plugins from the specified paths (parallel version).
    ///
    /// This method allows plugins to register and load multiple plugins concurrently.
    ///
    /// # Parameters
    ///
    /// * `paths` - Parallel iterator of paths to plugin files or directories
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<Bundle>, (Option<RegisterPluginError>, Option<LoadPluginError>)>`
    /// containing the plugin bundles on success, or errors from registration or loading.
    ///
    /// # Type Parameters
    ///
    /// * `'b` - Lifetime of the path references
    /// * `P` - Type of the parallel iterator containing path references
    pub fn par_load_plugins<'b, P>(
        &self,
        paths: P,
    ) -> Result<Vec<Bundle>, (Option<RegisterPluginError>, Option<LoadPluginError>)>
    where
        P: IntoParallelIterator<Item = &'b str>,
    {
        self.loader.as_mut().par_load_plugins(paths)
    }

    /// Loads only the plugins that are used (not dependencies of other plugins).
    ///
    /// This method allows plugins to register and load only the plugins that are not
    /// dependencies of other plugins, and automatically unregisters unused plugins.
    ///
    /// # Parameters
    ///
    /// * `paths` - Iterator of paths to plugin files or directories
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<Bundle>, (Option<RegisterPluginError>, Option<UnregisterPluginError>, Option<LoadPluginError>)>`
    /// containing the plugin bundles on success, or errors from registration, unregistration, or loading.
    ///
    /// # Type Parameters
    ///
    /// * `'b` - Lifetime of the path references
    /// * `P` - Type of the iterator containing path references
    pub fn load_only_used_plugins<'b, P>(
        &self,
        paths: P,
    ) -> Result<
        Vec<Bundle>,
        (
            Option<RegisterPluginError>,
            Option<UnregisterPluginError>,
            Option<LoadPluginError>,
        ),
    >
    where
        P: IntoIterator<Item = &'b str>,
    {
        self.loader.as_mut().load_only_used_plugins(paths)
    }

    /// Loads only the plugins that are used (not dependencies of other plugins) (parallel version).
    ///
    /// This method allows plugins to register and load only the plugins that are not
    /// dependencies of other plugins using parallel processing, and automatically unregisters unused plugins.
    ///
    /// # Parameters
    ///
    /// * `paths` - Parallel iterator of paths to plugin files or directories
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<Bundle>, (Option<RegisterPluginError>, Option<UnregisterPluginError>, Option<LoadPluginError>)>`
    /// containing the plugin bundles on success, or errors from registration, unregistration, or loading.
    ///
    /// # Type Parameters
    ///
    /// * `'b` - Lifetime of the path references
    /// * `P` - Type of the parallel iterator containing path references
    pub fn par_load_only_used_plugins<'b, P>(
        &self,
        paths: P,
    ) -> Result<
        Vec<Bundle>,
        (
            Option<RegisterPluginError>,
            Option<UnregisterPluginError>,
            Option<LoadPluginError>,
        ),
    >
    where
        P: IntoParallelIterator<Item = &'b str>,
    {
        self.loader.as_mut().par_load_only_used_plugins(paths)
    }

    /// Unloads a plugin from the execution environment.
    ///
    /// This method allows plugins to unload other plugins by ID and version.
    ///
    /// # Parameters
    ///
    /// * `id` - Plugin identifier
    /// * `version` - Plugin version
    ///
    /// # Returns
    ///
    /// Returns `Result<(), UnloadPluginError>` indicating success or failure.
    pub fn unload_plugin(&self, id: &str, version: &Version) -> Result<(), UnloadPluginError> {
        self.loader.as_mut().unload_plugin(id, version)
    }

    /// Unloads a plugin from the execution environment (parallel version).
    ///
    /// This method allows plugins to unload other plugins by ID and version using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `id` - Plugin identifier
    /// * `version` - Plugin version
    ///
    /// # Returns
    ///
    /// Returns `Result<(), UnloadPluginError>` indicating success or failure.
    pub fn par_unload_plugin(&self, id: &str, version: &Version) -> Result<(), UnloadPluginError> {
        self.loader.as_mut().par_unload_plugin(id, version)
    }

    /// Unloads a plugin from the execution environment by bundle.
    ///
    /// This method allows plugins to unload other plugins by bundle information.
    ///
    /// # Parameters
    ///
    /// * `bundle` - Plugin bundle information
    ///
    /// # Returns
    ///
    /// Returns `Result<(), UnloadPluginError>` indicating success or failure.
    pub fn unload_plugin_by_bundle(&self, bundle: &Bundle) -> Result<(), UnloadPluginError> {
        self.loader.as_mut().unload_plugin_by_bundle(bundle)
    }

    /// Unloads a plugin from the execution environment by bundle (parallel version).
    ///
    /// This method allows plugins to unload other plugins by bundle information using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `bundle` - Plugin bundle information
    ///
    /// # Returns
    ///
    /// Returns `Result<(), UnloadPluginError>` indicating success or failure.
    pub fn par_unload_plugin_by_bundle(&self, bundle: &Bundle) -> Result<(), UnloadPluginError> {
        self.loader.as_mut().par_unload_plugin_by_bundle(bundle)
    }

    /// Gets an immutable reference to a plugin by ID and version.
    ///
    /// This method allows plugins to access other registered plugins.
    ///
    /// # Parameters
    ///
    /// * `id` - Plugin identifier
    /// * `version` - Plugin version
    ///
    /// # Returns
    ///
    /// Returns `Option<&Plugin<'static, O, I>>` containing the plugin if found.
    pub fn get_plugin(&self, id: &str, version: &Version) -> Option<&Plugin<'static, O, I>> {
        self.loader.as_ref().get_plugin(id, version)
    }

    /// Gets an immutable reference to a plugin by ID and version (parallel version).
    ///
    /// This method allows plugins to access other registered plugins using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `id` - Plugin identifier
    /// * `version` - Plugin version
    ///
    /// # Returns
    ///
    /// Returns `Option<&Plugin<'static, O, I>>` containing the plugin if found.
    pub fn par_get_plugin(&self, id: &str, version: &Version) -> Option<&Plugin<'static, O, I>> {
        self.loader.as_ref().par_get_plugin(id, version)
    }

    /// Gets an immutable reference to a plugin by bundle.
    ///
    /// This method allows plugins to access other registered plugins by bundle information.
    ///
    /// # Parameters
    ///
    /// * `bundle` - Plugin bundle information
    ///
    /// # Returns
    ///
    /// Returns `Option<&Plugin<'static, O, I>>` containing the plugin if found.
    pub fn get_plugin_by_bundle(&self, bundle: &Bundle) -> Option<&Plugin<'static, O, I>> {
        self.loader.as_ref().get_plugin_by_bundle(bundle)
    }

    /// Gets an immutable reference to a plugin by bundle (parallel version).
    ///
    /// This method allows plugins to access other registered plugins by bundle information using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `bundle` - Plugin bundle information
    ///
    /// # Returns
    ///
    /// Returns `Option<&Plugin<'static, O, I>>` containing the plugin if found.
    pub fn par_get_plugin_by_bundle(&self, bundle: &Bundle) -> Option<&Plugin<'static, O, I>> {
        self.loader.as_ref().par_get_plugin_by_bundle(bundle)
    }

    /// Gets a mutable reference to a plugin by ID and version.
    ///
    /// This method allows plugins to access other registered plugins for modification.
    ///
    /// # Parameters
    ///
    /// * `id` - Plugin identifier
    /// * `version` - Plugin version
    ///
    /// # Returns
    ///
    /// Returns `Option<&mut Plugin<'static, O, I>>` containing the plugin if found.
    pub fn get_plugin_mut(
        &self,
        id: &str,
        version: &Version,
    ) -> Option<&mut Plugin<'static, O, I>> {
        self.loader.as_mut().get_plugin_mut(id, version)
    }

    /// Gets a mutable reference to a plugin by ID and version (parallel version).
    ///
    /// This method allows plugins to access other registered plugins for modification using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `id` - Plugin identifier
    /// * `version` - Plugin version
    ///
    /// # Returns
    ///
    /// Returns `Option<&mut Plugin<'static, O, I>>` containing the plugin if found.
    pub fn par_get_plugin_mut(
        &self,
        id: &str,
        version: &Version,
    ) -> Option<&mut Plugin<'static, O, I>> {
        self.loader.as_mut().par_get_plugin_mut(id, version)
    }

    /// Gets a mutable reference to a plugin by bundle.
    ///
    /// This method allows plugins to access other registered plugins for modification by bundle information.
    ///
    /// # Parameters
    ///
    /// * `bundle` - Plugin bundle information
    ///
    /// # Returns
    ///
    /// Returns `Option<&mut Plugin<'static, O, I>>` containing the plugin if found.
    pub fn get_plugin_mut_by_bundle(&self, bundle: &Bundle) -> Option<&mut Plugin<'static, O, I>> {
        self.loader.as_mut().get_plugin_mut_by_bundle(bundle)
    }

    /// Gets a mutable reference to a plugin by bundle (parallel version).
    ///
    /// This method allows plugins to access other registered plugins for modification by bundle information using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `bundle` - Plugin bundle information
    ///
    /// # Returns
    ///
    /// Returns `Option<&mut Plugin<'static, O, I>>` containing the plugin if found.
    pub fn par_get_plugin_mut_by_bundle(
        &self,
        bundle: &Bundle,
    ) -> Option<&mut Plugin<'static, O, I>> {
        self.loader.as_mut().par_get_plugin_mut_by_bundle(bundle)
    }

    /// Gets all plugins with the specified ID.
    ///
    /// This method allows plugins to access all versions of plugins with a specific ID.
    ///
    /// # Parameters
    ///
    /// * `id` - Plugin identifier to search for
    ///
    /// # Returns
    ///
    /// Returns `Vec<&Plugin<'static, O, I>>` containing all matching plugins.
    pub fn get_plugins_by_id(&self, id: &str) -> Vec<&Plugin<'static, O, I>> {
        self.loader.as_ref().get_plugins_by_id(id)
    }

    /// Gets all plugins with the specified ID (parallel version).
    ///
    /// This method allows plugins to access all versions of plugins with a specific ID using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `id` - Plugin identifier to search for
    ///
    /// # Returns
    ///
    /// Returns `Vec<&Plugin<'static, O, I>>` containing all matching plugins.
    pub fn par_get_plugins_by_id(&self, id: &str) -> Vec<&Plugin<'static, O, I>> {
        self.loader.as_ref().par_get_plugins_by_id(id)
    }

    /// Gets mutable references to all plugins with the specified ID.
    ///
    /// This method allows plugins to access all versions of plugins with a specific ID for modification.
    ///
    /// # Parameters
    ///
    /// * `id` - Plugin identifier to search for
    ///
    /// # Returns
    ///
    /// Returns `Vec<&mut Plugin<'static, O, I>>` containing all matching plugins.
    pub fn get_plugins_by_id_mut(&self, id: &str) -> Vec<&mut Plugin<'static, O, I>> {
        self.loader.as_mut().get_plugins_by_id_mut(id)
    }

    /// Gets mutable references to all plugins with the specified ID (parallel version).
    ///
    /// This method allows plugins to access all versions of plugins with a specific ID for modification using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `id` - Plugin identifier to search for
    ///
    /// # Returns
    ///
    /// Returns `Vec<&mut Plugin<'static, O, I>>` containing all matching plugins.
    pub fn par_get_plugins_by_id_mut(&self, id: &str) -> Vec<&mut Plugin<'static, O, I>> {
        self.loader.as_mut().par_get_plugins_by_id_mut(id)
    }

    /// Gets a reference to all loaded plugins.
    ///
    /// This method allows plugins to access the complete list of loaded plugins.
    ///
    /// # Returns
    ///
    /// Returns `&Vec<Plugin<'static, O, I>>` containing all loaded plugins.
    pub fn get_plugins(&self) -> &Vec<Plugin<'static, O, I>> {
        self.loader.as_ref().get_plugins()
    }

    /// Gets a reference to the function registry.
    ///
    /// This method allows plugins to access the registry of functions available to plugins.
    ///
    /// # Returns
    ///
    /// Returns `&Registry<O>` containing the function registry.
    pub fn get_registry(&self) -> &Registry<O> {
        self.loader.as_ref().get_registry()
    }

    /// Gets a reference to the function requests.
    ///
    /// This method allows plugins to access the collection of function requests.
    ///
    /// # Returns
    ///
    /// Returns `&Requests` containing the function requests.
    pub fn get_requests(&self) -> &Requests {
        self.loader.as_ref().get_requests()
    }

    /// Calls a function request across all eligible plugins.
    ///
    /// This method allows plugins to call a function request on all plugins that have the highest
    /// version for their ID (to avoid calling multiple versions of the same plugin).
    ///
    /// # Parameters
    ///
    /// * `name` - Name of the function request to call
    /// * `args` - Arguments to pass to the function
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<O>, PluginCallRequestError>` containing results from all
    /// eligible plugins that have the requested function.
    pub fn call_request(
        &self,
        name: &str,
        args: &[Variable],
    ) -> Result<Vec<O>, PluginCallRequestError> {
        self.loader.as_ref().call_request(name, args)
    }

    /// Calls a function request across all eligible plugins (parallel version).
    ///
    /// This method allows plugins to call a function request on all plugins that have the highest
    /// version for their ID using parallel processing.
    ///
    /// # Parameters
    ///
    /// * `name` - Name of the function request to call
    /// * `args` - Arguments to pass to the function
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<O>, PluginCallRequestError>` containing results from all
    /// eligible plugins that have the requested function.
    pub fn par_call_request(
        &self,
        name: &str,
        args: &[Variable],
    ) -> Result<Vec<O>, PluginCallRequestError> {
        self.loader.as_ref().par_call_request(name, args)
    }

    /// Calls a function on a required dependency.
    ///
    /// This method allows the current plugin to call functions exposed by its required dependencies.
    /// It ensures that the dependency exists and is loaded before attempting the call.
    ///
    /// # Parameters
    ///
    /// * `id` - Dependency plugin ID
    /// * `version` - Dependency plugin version
    /// * `name` - Function name to call
    /// * `args` - Arguments to pass to the function
    ///
    /// # Returns
    ///
    /// Returns `Result<O, CallFunctionDependError>` containing the function result on success,
    /// or an error if the dependency is not found or the function call fails.
    ///
    /// # Note
    ///
    /// This method only works with required dependencies. For optional dependencies,
    /// use `call_function_optional_depend`.
    pub fn call_function_depend(
        &self,
        id: &str,
        version: &Version,
        name: &str,
        args: &[Variable],
    ) -> Result<O, CallFunctionDependError> {
        let depend = self
            .depends
            .iter()
            .find(|&depend| *depend == (id, version))
            .ok_or(CallFunctionDependError::DependNotFound)?;

        let plugin = self
            .loader
            .as_ref()
            .get_plugin_by_bundle(depend)
            .ok_or(CallFunctionDependError::DependNotFound)?;

        Ok(plugin.call_function(name, args)?)
    }

    /// Calls a function on an optional dependency.
    ///
    /// This method allows the current plugin to call functions exposed by its optional dependencies.
    /// Unlike required dependencies, this method returns `None` if the dependency is not available.
    ///
    /// # Parameters
    ///
    /// * `id` - Optional dependency plugin ID
    /// * `version` - Optional dependency plugin version
    /// * `name` - Function name to call
    /// * `args` - Arguments to pass to the function
    ///
    /// # Returns
    ///
    /// Returns `Result<Option<O>, PluginCallFunctionError>` containing:
    /// - `Ok(Some(result))` if the dependency exists and the call succeeds
    /// - `Ok(None)` if the dependency is not available
    /// - `Err(error)` if the function call fails
    ///
    /// # Note
    ///
    /// This method only works with optional dependencies. For required dependencies,
    /// use `call_function_depend`.
    pub fn call_function_optional_depend(
        &self,
        id: &str,
        version: &Version,
        name: &str,
        args: &[Variable],
    ) -> Result<Option<O>, PluginCallFunctionError> {
        let depend = self
            .optional_depends
            .iter()
            .find(|&depend| *depend == (id, version));

        depend
            .and_then(|depend| self.loader.as_ref().get_plugin_by_bundle(depend))
            .map(|plugin| plugin.call_function(name, args))
            .transpose()
    }
}
