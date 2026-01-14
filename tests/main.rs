mod utils;

#[cfg(test)]
mod tests {
    use plux_mock::MockManager;

    use crate::utils::{get_plugin_path, loader_init, plugins_init};

    #[test]
    fn get_plugin_manager() {
        let mut loader = loader_init(MockManager::new());

        let is_manager = loader.get_manager_ref("mock").is_some();
        assert!(is_manager);

        loader.stop().unwrap();
    }

    #[test]
    fn register_plugin() {
        let filename = "void_plugin-v1.0.0.mock";

        let plugins = plugins_init(vec![filename]);
        let mut loader = loader_init(MockManager::from_plugins(plugins));

        let bundle = loader
            .register_plugin(get_plugin_path(filename).to_str().unwrap())
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
        let filename = "void_plugin-v1.0.0.mock";

        let plugins = plugins_init(vec![filename]);
        let mut loader = loader_init(MockManager::from_plugins(plugins));

        let bundle = loader
            .register_plugin(get_plugin_path(filename).to_str().unwrap())
            .unwrap();

        loader.load_plugin_by_bundle(&bundle).unwrap();
        loader.unload_plugin_by_bundle(&bundle).unwrap();

        loader.stop().unwrap();
    }

    #[test]
    fn load_now_plugin() {
        let filename = "void_plugin-v1.0.0.mock";

        let plugins = plugins_init(vec![filename]);
        let mut loader = loader_init(MockManager::from_plugins(plugins));

        let bundle = loader
            .load_plugin_now(get_plugin_path(filename).to_str().unwrap())
            .unwrap();

        loader.unload_plugin_by_bundle(&bundle).unwrap();
        loader.stop().unwrap();
    }

    #[test]
    fn unload_managers() {
        let filenames = vec![
            "dependency/dep_1-v1.0.0.mock",
            "plugin_for_manager-v1.0.0.mock",
            "dependency/dep_2-v1.0.0.mock",
            "function_plugin-v1.0.0.mock",
            "dependency/dep_3-v1.0.0.mock",
            "dependency/dep_4-v1.0.0.mock",
        ];

        let plugins = plugins_init(
            filenames
                .iter()
                .map(|x| x.split('/').last().unwrap())
                .collect(),
        );
        let mut loader = loader_init(MockManager::from_plugins(plugins));

        let paths = filenames
            .into_iter()
            .map(|filename| get_plugin_path(filename))
            .collect::<Vec<_>>();

        loader
            .load_plugins(paths.iter().map(|x| x.to_str().unwrap()))
            .unwrap();

        loader.unregister_manager("mock").unwrap();

        loader.stop().unwrap();
    }

    #[test]
    fn heavy_load() {
        let filenames = vec![
            "dependency/dep_1-v1.0.0.mock",
            "plugin_for_manager-v1.0.0.mock",
            "dependency/dep_2-v1.0.0.mock",
            "function_plugin-v1.0.0.mock",
            "dependency/dep_3-v1.0.0.mock",
            "dependency/dep_4-v1.0.0.mock",
        ];

        let plugins = plugins_init(
            filenames
                .iter()
                .map(|x| x.split('/').last().unwrap())
                .collect(),
        );
        let mut loader = loader_init(MockManager::from_plugins(plugins));

        let paths = filenames
            .into_iter()
            .map(|filename| get_plugin_path(filename))
            .collect::<Vec<_>>();

        loader
            .load_plugins(paths.iter().map(|x| x.to_str().unwrap()))
            .unwrap();

        loader.stop().unwrap();
    }
}
