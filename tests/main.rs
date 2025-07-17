mod utils;

#[cfg(test)]
mod tests {
    use august_plugin_system::{
        utils::{UnloadPluginError, UnregisterManagerError, UnregisterPluginError},
        Loader,
    };

    use crate::utils::{get_plugin_path, loader_init, LuaPluginManager, VoidPluginManager};

    #[test]
    fn get_plugin_manager() {
        let mut loader = loader_init(VoidPluginManager::new());

        let is_manager = loader.get_manager_ref("vpl").is_some();
        assert!(is_manager);

        loader.stop().unwrap();
    }

    #[test]
    fn register_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

        let bundle = loader
            .register_plugin(
                get_plugin_path("void_plugin", "1.0.0", "vpl")
                    .to_str()
                    .unwrap(),
            )
            .unwrap();

        let plugin = loader.get_plugin_by_bundle(&bundle).unwrap();
        println!(
            "Path = {:?}, Bundle = {}",
            plugin.info().path,
            plugin.info().bundle
        );

        loader.unregister_plugin_by_bundle(&bundle).unwrap();
        loader.stop().unwrap();
    }

    #[test]
    fn load_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

        let bundle = loader
            .register_plugin(
                get_plugin_path("void_plugin", "1.0.0", "vpl")
                    .to_str()
                    .unwrap(),
            )
            .unwrap();

        loader.load_plugin_by_bundle(&bundle).unwrap();
        loader.unload_plugin_by_bundle(&bundle).unwrap();

        loader.stop().unwrap();
    }

    #[test]
    fn load_now_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

        let bundle = loader
            .load_plugin_now(
                get_plugin_path("void_plugin", "1.0.0", "vpl")
                    .to_str()
                    .unwrap(),
            )
            .unwrap();

        loader.unload_plugin_by_bundle(&bundle).unwrap();
        loader.stop().unwrap();
    }

    #[test]
    fn unload_managers() {
        let mut loader = Loader::new();
        loader.context(|mut ctx| {
            ctx.register_manager(VoidPluginManager::new()).unwrap();
            ctx.register_manager(LuaPluginManager::new()).unwrap();
        });

        let paths = vec![
            get_plugin_path("dependency/dep_1", "1.0.0", "vpl"),
            get_plugin_path("plugin_for_manager", "1.0.0", "vpl"),
            get_plugin_path("dependency/dep_2", "1.0.0", "vpl"),
            get_plugin_path("function_plugin", "1.0.0", "fpl"),
            get_plugin_path("dependency/dep_3", "1.0.0", "vpl"),
            get_plugin_path("dependency/dep_4", "1.0.0", "vpl"),
        ];

        loader
            .load_plugins(paths.iter().map(|x| x.to_str().unwrap()))
            .unwrap();

        match loader.unregister_manager("fpl") {
            Err(UnregisterManagerError::UnregisterPlugin(UnregisterPluginError::UnloadError(
                UnloadPluginError::CurrentlyUsesDepend { .. },
            ))) => assert!(true),
            _ => assert!(false),
        };

        loader.stop().unwrap();
    }

    #[test]
    fn heavy_load() {
        let mut loader = Loader::new();
        loader.context(|mut ctx| {
            ctx.register_manager(VoidPluginManager::new()).unwrap();
            ctx.register_manager(LuaPluginManager::new()).unwrap();
        });

        let paths = vec![
            get_plugin_path("dependency/dep_1", "1.0.0", "vpl"),
            get_plugin_path("plugin_for_manager", "1.0.0", "vpl"),
            get_plugin_path("dependency/dep_2", "1.0.0", "vpl"),
            get_plugin_path("function_plugin", "1.0.0", "fpl"),
            get_plugin_path("dependency/dep_3", "1.0.0", "vpl"),
            get_plugin_path("dependency/dep_4", "1.0.0", "vpl"),
        ];

        loader
            .load_plugins(paths.iter().map(|x| x.to_str().unwrap()))
            .unwrap();

        loader.stop().unwrap();
    }
}
