use rayon::prelude::IntoParallelIterator;
use semver::Version;

use crate::{
    utils::{
        CallFunctionDependError, LoadPluginError, PluginCallFunctionError, PluginCallRequestError,
        Ptr, RegisterManagerError, RegisterPluginError, UnloadPluginError, UnregisterManagerError,
        UnregisterPluginError,
    },
    variable::Variable,
    Bundle, Info, Loader, Manager, Plugin, Registry, Requests,
};

pub struct Api<O: Send + Sync + 'static, I: Info + 'static> {
    loader: Ptr<'static, Loader<'static, O, I>>,
    plugin: Bundle,
    depends: Vec<Bundle>,
    optional_depends: Vec<Bundle>,
}

impl<O: Send + Sync + 'static, I: Info + 'static> Api<O, I> {
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

    pub fn registry(&self) -> &Registry<O> {
        &self.loader.as_ref().registry
    }

    pub const fn plugin(&self) -> &Bundle {
        &self.plugin
    }

    pub const fn depends(&self) -> &Vec<Bundle> {
        &self.depends
    }

    pub const fn optional_depends(&self) -> &Vec<Bundle> {
        &self.optional_depends
    }

    // Функции Loader'а

    pub fn register_manager<M>(&self, manager: M) -> Result<(), RegisterManagerError>
    where
        M: Manager<'static, O, I> + 'static,
    {
        self.loader.as_mut().register_manager(manager)
    }

    pub fn register_managers<M>(&self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoIterator<Item = Box<dyn Manager<'static, O, I>>>,
    {
        self.loader.as_mut().register_managers(managers)
    }

    pub fn par_register_managers<M>(&self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoParallelIterator<Item = Box<dyn Manager<'static, O, I>>>,
    {
        self.loader.as_mut().par_register_managers(managers)
    }

    pub fn unregister_manager(&self, format: &str) -> Result<(), UnregisterManagerError> {
        self.loader.as_mut().unregister_manager(format)
    }

    pub fn get_manager_ref(&self, format: &str) -> Option<&Box<dyn Manager<'static, O, I>>> {
        self.loader.as_ref().get_manager_ref(format)
    }

    pub fn par_get_manager_ref(&self, format: &str) -> Option<&Box<dyn Manager<'static, O, I>>> {
        self.loader.as_ref().par_get_manager_ref(format)
    }

    pub fn get_manager_mut(&self, format: &str) -> Option<&mut Box<dyn Manager<'static, O, I>>> {
        self.loader.as_mut().get_manager_mut(format)
    }

    pub fn par_get_manager_mut(
        &self,
        format: &str,
    ) -> Option<&mut Box<dyn Manager<'static, O, I>>> {
        self.loader.as_mut().par_get_manager_mut(format)
    }

    pub fn register_plugin(&self, path: &str) -> Result<Bundle, RegisterPluginError> {
        self.loader.as_mut().register_plugin(path)
    }

    pub fn register_plugins<'b, P>(&self, paths: P) -> Result<Vec<Bundle>, RegisterPluginError>
    where
        P: IntoIterator<Item = &'b str>,
    {
        self.loader.as_mut().register_plugins(paths)
    }

    pub fn par_register_plugins<'b, P>(&self, paths: P) -> Result<Vec<Bundle>, RegisterPluginError>
    where
        P: IntoParallelIterator<Item = &'b str>,
    {
        self.loader.as_mut().par_register_plugins(paths)
    }

    pub fn unregister_plugin(
        &self,
        id: &str,
        version: &Version,
    ) -> Result<(), UnregisterPluginError> {
        self.loader.as_mut().unregister_plugin(id, version)
    }

    pub fn unregister_plugin_by_bundle(
        &self,
        bundle: &Bundle,
    ) -> Result<(), UnregisterPluginError> {
        self.loader.as_mut().unregister_plugin_by_bundle(bundle)
    }

    pub fn par_unregister_plugin_by_bundle(
        &self,
        bundle: &Bundle,
    ) -> Result<(), UnregisterPluginError> {
        self.loader.as_mut().par_unregister_plugin_by_bundle(bundle)
    }

    pub fn load_plugin(&self, id: &str, version: &Version) -> Result<(), LoadPluginError> {
        self.loader.as_mut().load_plugin(id, version)
    }

    pub fn par_load_plugin(&self, id: &str, version: &Version) -> Result<(), LoadPluginError> {
        self.loader.as_mut().par_load_plugin(id, version)
    }

    pub fn load_plugin_by_bundle(&self, bundle: &Bundle) -> Result<(), LoadPluginError> {
        self.loader.as_mut().load_plugin_by_bundle(bundle)
    }

    pub fn par_load_plugin_by_bundle(&self, bundle: &Bundle) -> Result<(), LoadPluginError> {
        self.loader.as_mut().par_load_plugin_by_bundle(bundle)
    }

    pub fn load_plugin_now(
        &self,
        path: &str,
    ) -> Result<Bundle, (Option<RegisterPluginError>, Option<LoadPluginError>)> {
        self.loader.as_mut().load_plugin_now(path)
    }

    pub fn load_plugins<'b, P>(
        &self,
        paths: P,
    ) -> Result<Vec<Bundle>, (Option<RegisterPluginError>, Option<LoadPluginError>)>
    where
        P: IntoIterator<Item = &'b str>,
    {
        self.loader.as_mut().load_plugins(paths)
    }

    pub fn par_load_plugins<'b, P>(
        &self,
        paths: P,
    ) -> Result<Vec<Bundle>, (Option<RegisterPluginError>, Option<LoadPluginError>)>
    where
        P: IntoParallelIterator<Item = &'b str>,
    {
        self.loader.as_mut().par_load_plugins(paths)
    }

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

    pub fn unload_plugin(&self, id: &str, version: &Version) -> Result<(), UnloadPluginError> {
        self.loader.as_mut().unload_plugin(id, version)
    }

    pub fn par_unload_plugin(&self, id: &str, version: &Version) -> Result<(), UnloadPluginError> {
        self.loader.as_mut().par_unload_plugin(id, version)
    }

    pub fn unload_plugin_by_bundle(&self, bundle: &Bundle) -> Result<(), UnloadPluginError> {
        self.loader.as_mut().unload_plugin_by_bundle(bundle)
    }

    pub fn par_unload_plugin_by_bundle(&self, bundle: &Bundle) -> Result<(), UnloadPluginError> {
        self.loader.as_mut().par_unload_plugin_by_bundle(bundle)
    }

    pub fn get_plugin(&self, id: &str, version: &Version) -> Option<&Plugin<'static, O, I>> {
        self.loader.as_ref().get_plugin(id, version)
    }

    pub fn par_get_plugin(&self, id: &str, version: &Version) -> Option<&Plugin<'static, O, I>> {
        self.loader.as_ref().par_get_plugin(id, version)
    }

    pub fn get_plugin_by_bundle(&self, bundle: &Bundle) -> Option<&Plugin<'static, O, I>> {
        self.loader.as_ref().get_plugin_by_bundle(bundle)
    }

    pub fn par_get_plugin_by_bundle(&self, bundle: &Bundle) -> Option<&Plugin<'static, O, I>> {
        self.loader.as_ref().par_get_plugin_by_bundle(bundle)
    }

    pub fn get_plugin_mut(
        &self,
        id: &str,
        version: &Version,
    ) -> Option<&mut Plugin<'static, O, I>> {
        self.loader.as_mut().get_plugin_mut(id, version)
    }

    pub fn par_get_plugin_mut(
        &self,
        id: &str,
        version: &Version,
    ) -> Option<&mut Plugin<'static, O, I>> {
        self.loader.as_mut().par_get_plugin_mut(id, version)
    }

    pub fn get_plugin_mut_by_bundle(&self, bundle: &Bundle) -> Option<&mut Plugin<'static, O, I>> {
        self.loader.as_mut().get_plugin_mut_by_bundle(bundle)
    }

    pub fn par_get_plugin_mut_by_bundle(
        &self,
        bundle: &Bundle,
    ) -> Option<&mut Plugin<'static, O, I>> {
        self.loader.as_mut().par_get_plugin_mut_by_bundle(bundle)
    }

    pub fn get_plugins_by_id(&self, id: &str) -> Vec<&Plugin<'static, O, I>> {
        self.loader.as_ref().get_plugins_by_id(id)
    }

    pub fn par_get_plugins_by_id(&self, id: &str) -> Vec<&Plugin<'static, O, I>> {
        self.loader.as_ref().par_get_plugins_by_id(id)
    }

    pub fn get_plugins_by_id_mut(&self, id: &str) -> Vec<&mut Plugin<'static, O, I>> {
        self.loader.as_mut().get_plugins_by_id_mut(id)
    }

    pub fn par_get_plugins_by_id_mut(&self, id: &str) -> Vec<&mut Plugin<'static, O, I>> {
        self.loader.as_mut().par_get_plugins_by_id_mut(id)
    }

    pub fn get_plugins(&self) -> &Vec<Plugin<'static, O, I>> {
        self.loader.as_ref().get_plugins()
    }

    pub fn get_registry(&self) -> &Registry<O> {
        self.loader.as_ref().get_registry()
    }

    pub fn get_requests(&self) -> &Requests {
        self.loader.as_ref().get_requests()
    }

    pub fn call_request(
        &self,
        name: &str,
        args: &[Variable],
    ) -> Result<Vec<O>, PluginCallRequestError> {
        self.loader.as_ref().call_request(name, args)
    }

    pub fn par_call_request(
        &self,
        name: &str,
        args: &[Variable],
    ) -> Result<Vec<O>, PluginCallRequestError> {
        self.loader.as_ref().par_call_request(name, args)
    }

    // Дополнительные функции

    //TODO: Добавить параллельную версию
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

    //TODO: Добавить параллельную версию
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
