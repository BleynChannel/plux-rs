mod plugins;
mod utils;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use plux_mock::MockManager;
    use plux_rs::prelude::*;

    use crate::{
        plugins::{function_plugin_v1, function_plugin_v2},
        utils::{get_plugin_path, loader_init, plugins_init},
    };

    const PATH: &str = "versions";

    const TOOLS: [(&str, &str); 2] = [("paint", "1.0.0"), ("photoshop", "1.0.0")];

    fn get_versions_filename() -> Vec<String> {
        vec![
            format!("{PATH}/brush-v1.0.0.mock"),
            format!("{PATH}/brush-v2.0.0.mock"),
            format!("{PATH}/brush-v3.0.0.mock"),
        ]
    }

    fn get_tools_filename() -> Vec<String> {
        TOOLS
            .into_iter()
            .map(|(id, version)| format!("{PATH}/{id}-v{version}.mock"))
            .collect()
    }

    #[test]
    fn load_another_version() {
        let filenames = get_versions_filename();

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

        let plugins = loader
            .load_plugins(paths.iter().map(|x| x.to_str().unwrap()))
            .unwrap();

        for bundle in plugins {
            println!("Plugin = {}", bundle);
        }
    }

    #[test]
    fn load_version_as_dependency() {
        let filenames: Vec<_> = get_versions_filename()
            .into_iter()
            .chain(get_tools_filename().into_iter())
            .collect();

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

        let plugins = loader
            .load_plugins(paths.iter().map(|x| x.to_str().unwrap()))
            .unwrap();

        for bundle in plugins {
            println!("Plugin = {}", bundle);
        }
    }

    #[test]
    fn load_only_used_plugins() {
        let filenames: Vec<_> = get_versions_filename()
            .into_iter()
            .chain(get_tools_filename().into_iter())
            .collect();

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

        let bundles = loader
            .load_only_used_plugins(paths.iter().map(|x| x.to_str().unwrap()))
            .unwrap();

        bundles
            .iter()
            .for_each(|bundle| println!("Plugin: {}", bundle));

        loader.stop().unwrap();
    }

    #[test]
    fn call_request() {
        let mut plugins = HashMap::new();

        function_plugin_v1::insert_plugin(&mut plugins);
        function_plugin_v2::insert_plugin(&mut plugins);

        let mut loader = SimpleLoader::new();
        loader.context(move |mut ctx| {
            ctx.register_request(Request::new(
                "echo".to_string(),
                vec![VariableType::String],
                Some(VariableType::String),
            ));
            ctx.register_manager(MockManager::from_plugins(plugins))
                .unwrap();
        });

        let paths = vec![function_plugin_v1::FILENAME, function_plugin_v2::FILENAME]
            .into_iter()
            .map(|filename| get_plugin_path(filename))
            .collect::<Vec<_>>();

        loader
            .load_plugins(paths.iter().map(|path| path.to_str().unwrap()))
            .unwrap();

        match loader
            .call_request("echo", &["Hello world".into()])
            .unwrap()
            .get(0)
            .unwrap()
        {
            Err(e) => panic!("{:?}: {}", e, e.to_string()),
            Ok(Some(result)) => println!("{:?}", result),
            Ok(None) => panic!("Unexpected result"),
        };
    }
}
