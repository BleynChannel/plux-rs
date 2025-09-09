use plux_rs::function::FunctionOutput;
use plux_rs::{
    Api, LoadPluginContext, Manager, Plugin, RegisterPluginContext, StdInfo, utils::ManagerResult,
};

// Create our custom plugin manager
pub struct CustomManager {
    // Here you can store some plugin data
}

// Implement the Manager trait for our manager
// Important! The 'Manager' trait requires two type parameters:
// 1. The return type for plugin functions
// 2. The plugin info trait
//
// In our case, we're using 'FunctionOutput' and 'StdInfo' provided by default in plux
impl<'a> Manager<'a, FunctionOutput, StdInfo> for CustomManager {
    // Register our plugin format
    // Plugins with the "cst" format will be loaded by this manager
    fn format(&self) -> &'static str {
        "cst"
    }

    // Implement manager registration
    // This function is called when the manager is loaded
    fn register_manager(&mut self) -> ManagerResult<()> {
        println!("CustomManager::register_manager");
        Ok(())
    }

    // Implement manager unregistration
    // This function is called when the manager is unloaded
    fn unregister_manager(&mut self) -> ManagerResult<()> {
        println!("CustomManager::unregister_manager");
        Ok(())
    }

    // Implement plugin registration
    // This function is called when a plugin is registered in the plugin system
    // Here you can load/initialize the plugin
    // Must return the plugin info
    fn register_plugin(&mut self, context: RegisterPluginContext) -> ManagerResult<StdInfo> {
        let info = StdInfo {
            depends: vec![],
            optional_depends: vec![],
        };

        println!("CustomManager::register_plugin - {}", context.bundle);
        Ok(info)
    }

    // Implement plugin unregistration
    // This function is called when a plugin is unregistered
    fn unregister_plugin(
        &mut self,
        plugin: &Plugin<'a, FunctionOutput, StdInfo>,
    ) -> ManagerResult<()> {
        println!(
            "CustomManager::unregister_plugin - {}",
            plugin.info().bundle
        );
        Ok(())
    }

    // Implement plugin loading
    // This function is called after successful plugin registration
    // Here you can register plugin functions, APIs, specify dependency functions, etc.
    fn load_plugin(
        &mut self,
        context: LoadPluginContext<'a, '_, FunctionOutput, StdInfo>,
        _: Api<FunctionOutput, StdInfo>,
    ) -> ManagerResult<()> {
        println!(
            "CustomManager::load_plugin - {}",
            context.plugin().info().bundle
        );
        Ok(())
    }

    // Implement plugin unregistration
    // This function is called when a plugin is unregistered
    fn unload_plugin(&mut self, plugin: &Plugin<'a, FunctionOutput, StdInfo>) -> ManagerResult<()> {
        println!("CustomManager::unload_plugin - {}", plugin.info().bundle);
        Ok(())
    }
}

impl CustomManager {
    pub fn new() -> Self {
        Self {}
    }
}

// -- Using CustomManager in practice -- //

use plux_rs::Loader;

fn main() {
    let mut loader = Loader::<FunctionOutput, StdInfo>::new();
    loader.context(|mut ctx| {
        ctx.register_manager(CustomManager::new()).unwrap(); // Register the manager
    });

    // Here you can load your plugin
    // loader.load_plugin_now("my_plugin-v1.0.0.cst").unwrap();
}
