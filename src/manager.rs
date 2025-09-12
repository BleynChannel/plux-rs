use crate::{
    Api, Info, Plugin, RegisterPluginContext, context::LoadPluginContext, utils::ManagerResult,
};

/// Trait for implementing custom plugin managers.
///
/// A plugin manager is responsible for handling plugins of a specific format (e.g., Lua, Rust, WASM).
/// It provides the interface between the Plux engine and the actual plugin execution environment.
///
/// # Type Parameters
///
/// * `'a` - Lifetime parameter for references within the manager
/// * `O` - Output type for plugin functions (must implement Send + Sync)
/// * `I` - Plugin information type (must implement Info trait)
///
/// # Example
///
/// ```rust,no_run
/// use plux_rs::{Manager, LoadPluginContext, RegisterPluginContext, Api, Plugin, StdInfo, utils::ManagerResult, function::FunctionOutput};
///
/// struct MyManager;
///
/// impl<'a> Manager<'a, FunctionOutput, StdInfo> for MyManager {
///     fn format(&self) -> &'static str {
///         "my_format"
///     }
///
///     fn register_plugin(&mut self, context: RegisterPluginContext) -> ManagerResult<StdInfo> {
///         // Implementation for registering a plugin
///         Ok(StdInfo {
///             depends: vec![],
///             optional_depends: vec![],
///         })
///     }
/// }
/// ```
pub trait Manager<'a, O: Send + Sync, I: Info>: Send + Sync {
    /// Returns the file format/extension this manager handles (e.g., "lua", "rs", "wasm").
    ///
    /// This format is used to identify which manager should handle a particular plugin file.
    fn format(&self) -> &'static str;

    /// Called when the manager is registered with the loader.
    ///
    /// This is the place to perform any initialization required by the manager.
    /// Default implementation does nothing and returns Ok(()).
    ///
    /// # Returns
    ///
    /// Returns `ManagerResult<()>` indicating success or failure of manager registration.
    fn register_manager(&mut self) -> ManagerResult<()> {
        Ok(())
    }

    /// Called when the manager is unregistered from the loader.
    ///
    /// This is the place to perform any cleanup required by the manager.
    /// Default implementation does nothing and returns Ok(()).
    ///
    /// # Returns
    ///
    /// Returns `ManagerResult<()>` indicating success or failure of manager unregistration.
    fn unregister_manager(&mut self) -> ManagerResult<()> {
        Ok(())
    }

    /// Registers a plugin with this manager.
    ///
    /// This method is called when a plugin file matching this manager's format is discovered.
    /// The manager should validate the plugin and return information about it.
    ///
    /// # Parameters
    ///
    /// * `context` - Context containing plugin path and bundle information
    ///
    /// # Returns
    ///
    /// Returns `ManagerResult<I>` containing plugin information on success.
    fn register_plugin(&mut self, _context: RegisterPluginContext) -> ManagerResult<I>;

    /// Unregisters a plugin from this manager.
    ///
    /// This method is called when a plugin is being removed from the system.
    /// Default implementation does nothing and returns Ok(()).
    ///
    /// # Parameters
    ///
    /// * `plugin` - Reference to the plugin being unregistered
    ///
    /// # Returns
    ///
    /// Returns `ManagerResult<()>` indicating success or failure of plugin unregistration.
    fn unregister_plugin(&mut self, _plugin: &Plugin<'a, O, I>) -> ManagerResult<()> {
        Ok(())
    }

    /// Loads a plugin into the execution environment.
    ///
    /// This method is called after plugin registration and is responsible for making the plugin
    /// available for execution. This typically involves loading the plugin code, registering
    /// functions, and setting up the execution context.
    ///
    /// Default implementation does nothing and returns Ok(()).
    ///
    /// # Parameters
    ///
    /// * `context` - Context containing plugin and loader information
    /// * `api` - API interface for interacting with the host application
    ///
    /// # Returns
    ///
    /// Returns `ManagerResult<()>` indicating success or failure of plugin loading.
    fn load_plugin(
        &mut self,
        _context: LoadPluginContext<'a, '_, O, I>,
        _api: Api<O, I>,
    ) -> ManagerResult<()> {
        Ok(())
    }

    /// Unloads a plugin from the execution environment.
    ///
    /// This method is called when a plugin is being unloaded. It should clean up any resources
    /// associated with the plugin.
    ///
    /// Default implementation does nothing and returns Ok(()).
    ///
    /// # Parameters
    ///
    /// * `plugin` - Reference to the plugin being unloaded
    ///
    /// # Returns
    ///
    /// Returns `ManagerResult<()>` indicating success or failure of plugin unloading.
    fn unload_plugin(&mut self, _plugin: &Plugin<'a, O, I>) -> ManagerResult<()> {
        Ok(())
    }
}

impl<'a, O: Send + Sync, I: Info> PartialEq for dyn Manager<'a, O, I> {
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
    }
}

impl<'a, O, OO, I, II> PartialEq<Box<dyn Manager<'a, O, I>>> for dyn Manager<'a, OO, II>
where
    O: Send + Sync,
    OO: Send + Sync,
    I: Info,
    II: Info,
{
    fn eq(&self, other: &Box<dyn Manager<'a, O, I>>) -> bool {
        self.format() == other.format()
    }
}

impl<'a, O, OO, I, II> PartialEq<dyn Manager<'a, OO, II>> for Box<dyn Manager<'a, O, I>>
where
    O: Send + Sync,
    OO: Send + Sync,
    I: Info,
    II: Info,
{
    fn eq(&self, other: &dyn Manager<'a, OO, II>) -> bool {
        self.format() == other.format()
    }
}
