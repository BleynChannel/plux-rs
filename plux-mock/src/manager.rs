use std::collections::HashMap;

use plux_rs::{
    Api, Bundle, Manager, Plugin, RegisterPluginContext, StdInfo,
    context::LoadPluginContext,
    utils::{ManagerResult, RegisterPluginError},
};

use crate::MockPlugin;

pub struct MockManager<'a, O: Send + Sync + 'static> {
    pub(crate) plugins: HashMap<Bundle, MockPlugin<'a, O>>,
}

impl<'a, O: Send + Sync> Manager<'a, O, StdInfo> for MockManager<'a, O> {
    fn format(&self) -> &'static str {
        "mock"
    }

    fn register_manager(&mut self) -> ManagerResult<()> {
        println!("MockManager::register_manager");
        Ok(())
    }

    fn unregister_manager(&mut self) -> ManagerResult<()> {
        println!("MockManager::unregister_manager");
        Ok(())
    }

    fn register_plugin(&mut self, context: RegisterPluginContext) -> ManagerResult<StdInfo> {
        println!("MockManager::register_plugin - {}", context.bundle);

        let plugin = self
            .plugins
            .get_mut(&context.bundle)
            .ok_or_else(|| RegisterPluginError::NotFound)?;
        let info = plugin.info.clone();

        Ok(info)
    }

    fn unregister_plugin(&mut self, plugin: &Plugin<'a, O, StdInfo>) -> ManagerResult<()> {
        println!("MockManager::unregister_plugin - {}", plugin.info().bundle);
        Ok(())
    }

    fn load_plugin(
        &mut self,
        context: LoadPluginContext<'a, '_, O, StdInfo>,
        api: Api<O, StdInfo>,
    ) -> ManagerResult<()> {
        println!(
            "MockManager::load_plugin - {}",
            context.plugin().info().bundle
        );

        let plugin = self
            .plugins
            .get_mut(&context.plugin().info().bundle)
            .unwrap();

        (plugin.on_load_plugin)(context, api)?;

        Ok(())
    }

    fn unload_plugin(&mut self, plugin: &Plugin<'a, O, StdInfo>) -> ManagerResult<()> {
        println!("MockManager::unload_plugin - {}", plugin.info().bundle);
        Ok(())
    }
}

impl<'a, O: Send + Sync> MockManager<'a, O> {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn from_plugins(plugins: HashMap<Bundle, MockPlugin<'a, O>>) -> Self {
        Self { plugins }
    }
}
