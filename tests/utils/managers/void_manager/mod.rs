use config::*;
use plux::{
    Api, Manager, Plugin, RegisterPluginContext, StdInfo, context::LoadPluginContext,
    utils::ManagerResult,
};

mod config;

pub struct VoidPluginManager {
    configs: Vec<Config>,
}

impl<'a, O: Send + Sync> Manager<'a, O, StdInfo> for VoidPluginManager {
    fn format(&self) -> &'static str {
        "vpl"
    }

    fn register_manager(&mut self) -> ManagerResult<()> {
        println!("VoidPluginManager::register_manager");
        Ok(())
    }

    fn unregister_manager(&mut self) -> ManagerResult<()> {
        println!("VoidPluginManager::unregister_manager");
        Ok(())
    }

    fn register_plugin(&mut self, context: RegisterPluginContext) -> ManagerResult<StdInfo> {
        let (config, info) = load_config(context.path)?;
        self.configs.push(config);

        println!("VoidPluginManager::register_plugin - {}", context.bundle);
        Ok(info)
    }

    fn unregister_plugin(&mut self, plugin: &Plugin<'a, O, StdInfo>) -> ManagerResult<()> {
        println!(
            "VoidPluginManager::unregister_plugin - {}",
            plugin.info().bundle
        );
        Ok(())
    }

    fn load_plugin(
        &mut self,
        context: LoadPluginContext<'a, '_, O, StdInfo>,
        _: Api<O, StdInfo>,
    ) -> ManagerResult<()> {
        println!(
            "VoidPluginManager::load_plugin - {}",
            context.plugin().info().bundle
        );
        Ok(())
    }

    fn unload_plugin(&mut self, plugin: &Plugin<'a, O, StdInfo>) -> ManagerResult<()> {
        println!(
            "VoidPluginManager::unload_plugin - {}",
            plugin.info().bundle
        );
        Ok(())
    }
}

impl VoidPluginManager {
    pub fn new() -> Self {
        Self { configs: vec![] }
    }
}
