use std::path::PathBuf;

use crate::Bundle;

/// Context provided during plugin registration.
///
/// RegisterPluginContext contains information about a plugin that is being registered
/// with a manager. It provides access to the plugin's filesystem location and metadata.
///
/// # Type Parameters
///
/// * `'a` - Lifetime of the references
///
/// # Fields
///
/// * `path` - Reference to the filesystem path of the plugin file or directory
/// * `bundle` - Reference to the plugin's bundle metadata (id, version, format)
///
/// # Example
///
/// ```rust,no_run
/// use plux_rs::{Manager, RegisterPluginContext, StdInfo, utils::ManagerResult};
/// use std::path::PathBuf;
///
/// struct MyManager;
///
/// impl Manager<'_, (), StdInfo> for MyManager {
///     fn format(&self) -> &'static str { "my" }
///
///     fn register_plugin(&mut self, context: RegisterPluginContext) -> ManagerResult<StdInfo> {
///         println!("Registering plugin: {}", context.bundle.id);
///         println!("Plugin path: {:?}", context.path);
///
///         Ok(StdInfo::new())
///     }
/// }
/// ```
pub struct RegisterPluginContext<'a> {
    /// Filesystem path to the plugin file or directory
    pub path: &'a PathBuf,
    /// Plugin bundle metadata (id, version, format)
    pub bundle: &'a Bundle,
}
