mod utils;

#[cfg(test)]
mod dependency {
    use plux_mock::{MockManager, MockPlugin};
    use plux_rs::{Bundle, Depend, StdInfo};
    use semver::{Version, VersionReq};

    use crate::utils::{get_plugin_path, loader_init};

    fn get_dependencys() -> Vec<(String, StdInfo)> {
        vec![
            (
                format!("dependency/dep_1-v1.0.0.mock"),
                StdInfo {
                    ..Default::default()
                },
            ),
            (
                format!("dependency/dep_2-v1.0.0.mock"),
                StdInfo {
                    depends: vec![Depend::new(
                        "dep_1".to_string(),
                        VersionReq::parse("1.0").unwrap(),
                    )],
                    optional_depends: vec![Depend::new(
                        "dep_3".to_string(),
                        VersionReq::parse("2.0").unwrap(),
                    )],
                    ..Default::default()
                },
            ),
            (
                format!("dependency/dep_3-v1.0.0.mock"),
                StdInfo {
                    optional_depends: vec![Depend::new(
                        "dep_2".to_string(),
                        VersionReq::parse("1.0").unwrap(),
                    )],
                    ..Default::default()
                },
            ),
            (
                format!("dependency/dep_4-v1.0.0.mock"),
                StdInfo {
                    depends: vec![Depend::new(
                        "dep_1".to_string(),
                        VersionReq::parse("1.0").unwrap(),
                    )],
                    optional_depends: vec![
                        Depend::new("dep_3".to_string(), VersionReq::parse("1.0").unwrap()),
                        Depend::new("dep_5".to_string(), VersionReq::parse("1.0").unwrap()),
                    ],
                },
            ),
        ]
    }

    #[test]
    fn register_dependency_plugin() {
        let (dependency, paths): (_, Vec<_>) = get_dependencys()
            .into_iter()
            .map(|x| {
                let bundle = Bundle::from_filename(x.0.split('/').last().unwrap()).unwrap();
                let path = get_plugin_path(x.0);
                let plugin = MockPlugin::new(x.1, |_, _| Ok(()));
                ((bundle, plugin), path)
            })
            .unzip();

        let mut loader = loader_init(MockManager::from_plugins(dependency));

        for path in paths {
            loader.register_plugin(path.to_str().unwrap()).unwrap();
        }
    }

    #[test]
    fn load_dependency_plugin() {
        let (dependency, paths): (_, Vec<_>) = get_dependencys()
            .into_iter()
            .map(|x| {
                let bundle = Bundle::from_filename(x.0.split('/').last().unwrap()).unwrap();
                let path = get_plugin_path(x.0);
                let plugin = MockPlugin::new(x.1, |_, _| Ok(()));
                ((bundle, plugin), path)
            })
            .unzip();

        let mut loader = loader_init(MockManager::from_plugins(dependency));

        for path in paths {
            loader.register_plugin(path.to_str().unwrap()).unwrap();
        }

        loader
            .load_plugin("dep_3", &Version::parse("1.0.0").unwrap())
            .unwrap();
    }

    #[test]
    fn load_plugins() {
        let (dependency, paths): (_, Vec<_>) = get_dependencys()
            .into_iter()
            .map(|x| {
                let bundle = Bundle::from_filename(x.0.split('/').last().unwrap()).unwrap();
                let path = get_plugin_path(x.0);
                let plugin = MockPlugin::new(x.1, |_, _| Ok(()));
                ((bundle, plugin), path)
            })
            .unzip();

        let mut loader = loader_init(MockManager::from_plugins(dependency));

        let plugins = loader
            .load_plugins(paths.iter().map(|x| x.to_str().unwrap()))
            .unwrap();

        for bundle in plugins {
            let plugin = loader.get_plugin_by_bundle(&bundle).unwrap();
            println!(
                "Path = {:?}, Bundle = {}",
                plugin.info().path,
                plugin.info().bundle
            );
        }
    }
}
