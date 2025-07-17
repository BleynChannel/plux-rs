mod utils;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use august_plugin_system::{function::Request, variable::VariableType, Loader};

    use crate::utils::{get_plugin_path, loader_init, LuaPluginManager, VoidPluginManager};

    const FORMAT: &str = "vpl";
    const PATH: &str = "versions";

    const TOOLS: [(&str, &str); 2] = [("paint", "1.0.0"), ("photoshop", "1.0.0")];

    fn get_versions_path() -> Vec<PathBuf> {
        let id = format!("{PATH}/brush");

        vec![
            get_plugin_path(id.as_str(), "1.0.0", FORMAT),
            get_plugin_path(id.as_str(), "2.0.0", FORMAT),
            get_plugin_path(id.as_str(), "3.0.0", FORMAT),
        ]
    }

    fn get_tools_path() -> Vec<PathBuf> {
        TOOLS
            .into_iter()
            .map(|(id, version)| get_plugin_path(format!("{PATH}/{id}").as_str(), version, FORMAT))
            .collect()
    }

    #[test]
    fn load_another_version() {
        let mut loader = loader_init(VoidPluginManager::new());

        let paths = get_versions_path();

        let plugins = loader
            .load_plugins(
                paths
                    .iter()
                    .map(|path| path.to_str().unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap();

        for bundle in plugins {
            println!("Plugin = {}", bundle);
        }
    }

    #[test]
    fn load_version_as_dependency() {
        let mut loader = loader_init(VoidPluginManager::new());

        let paths: Vec<_> = get_versions_path()
            .into_iter()
            .chain(get_tools_path().into_iter())
            .collect();

        let plugins = loader
            .load_plugins(
                paths
                    .iter()
                    .map(|path| path.to_str().unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap();

        for bundle in plugins {
            println!("Plugin = {}", bundle);
        }
    }

    #[test]
    fn load_only_used_plugins() {
        let mut loader = loader_init(VoidPluginManager::new());

        let paths: Vec<_> = get_versions_path()
            .into_iter()
            .chain(get_tools_path().into_iter())
            .collect();

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
        let mut loader = Loader::new();
        loader.context(move |mut ctx| {
            ctx.register_request(Request::new(
                "echo".to_string(),
                vec![VariableType::String],
                Some(VariableType::String),
            ));
            ctx.register_manager(LuaPluginManager::new()).unwrap();
        });

        const VERSIONS: [&str; 2] = ["1.0.0", "2.0.0"];
        let paths: Vec<_> = VERSIONS
            .iter()
            .map(|&version| get_plugin_path("function_plugin", version, "fpl"))
            .collect();

        loader
            .load_plugins(paths.iter().map(|path| path.to_str().unwrap()))
            .unwrap();

        match loader
            .call_request("echo", &["Hello world".into()])
            .unwrap()
            .get(0)
            .unwrap()
        {
            Err(e) => match e.downcast_ref::<mlua::Error>() {
                Some(e) => panic!("[LUA ERROR]: {e:?}"),
                None => panic!("{:?}: {}", e, e.to_string()),
            },
            Ok(Some(result)) => println!("{:?}", result),
            Ok(None) => panic!("Unexpected result"),
        };
    }
}
