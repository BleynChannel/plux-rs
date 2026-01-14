mod plugins;
mod utils;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use plux_mock::MockManager;
    use plux_rs::prelude::*;
    use semver::Version;

    use crate::{
        plugins::{function_plugin_v1, parallel_plugins, plugin_function},
        utils::{benchmark, get_plugin_path},
    };

    #[function]
    fn add(_: (), a: &i32, b: &i32) -> i32 {
        a + b
    }

    #[function]
    fn sub(_: (), a: &i32, b: &i32) -> i32 {
        a - b
    }

    #[test]
    fn register_function() {
        let mut loader = SimpleLoader::new();
        loader.context(move |mut ctx| {
            ctx.register_function(add());
            ctx.register_manager(MockManager::new()).unwrap();
        });
    }

    #[test]
    fn register_functions() {
        let mut loader = SimpleLoader::new();
        loader.context(move |mut ctx| {
            ctx.register_function(add());
            ctx.register_function(sub());
            ctx.register_manager(MockManager::new()).unwrap();
        });
    }

    #[test]
    fn register_request() {
        let mut plugins = HashMap::new();

        // Create mock plugin with a callback that registers functions and requests
        function_plugin_v1::insert_plugin(&mut plugins);

        let mut loader = SimpleLoader::new();
        loader.context(move |mut ctx| {
            ctx.register_request(Request::new(
                "mul".to_string(),
                vec![VariableType::I32, VariableType::I32],
                Some(VariableType::I32),
            ));
            ctx.register_manager(MockManager::from_plugins(plugins))
                .unwrap();
        });

        loader
            .load_plugin_now(
                get_plugin_path(function_plugin_v1::FILENAME)
                    .to_str()
                    .unwrap(),
            )
            .unwrap();
    }

    #[test]
    fn call_request() {
        let mut plugins = HashMap::new();

        function_plugin_v1::insert_plugin(&mut plugins);

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

        let plugin = loader
            .load_plugin_now(
                get_plugin_path(function_plugin_v1::FILENAME)
                    .to_str()
                    .unwrap(),
            )
            .map(|bundle| loader.get_plugin_by_bundle(&bundle).unwrap())
            .unwrap();

        match plugin
            .call_request("echo", &["Hello world".into()])
            .unwrap()
        {
            Err(e) => panic!("{:?}: {}", e, e.to_string()),
            Ok(Some(result)) => println!("{:?}", result),
            Ok(None) => panic!("Unexpected result"),
        };
    }

    #[test]
    fn common_call() {
        let mut plugins = HashMap::new();

        function_plugin_v1::insert_plugin(&mut plugins);

        let mut loader = SimpleLoader::new();
        loader.context(move |mut ctx| {
            ctx.register_function(add());
            ctx.register_function(sub());
            ctx.register_request(Request::new("main".to_string(), vec![], None));
            ctx.register_manager(MockManager::from_plugins(plugins))
                .unwrap();
        });

        let plugin = loader
            .load_plugin_now(
                get_plugin_path(function_plugin_v1::FILENAME)
                    .to_str()
                    .unwrap(),
            )
            .map(|bundle| loader.get_plugin_by_bundle(&bundle).unwrap())
            .unwrap();

        match plugin.call_request("main", &[]).unwrap() {
            Err(e) => panic!("{:?}: {}", e, e.to_string()),
            Ok(_) => (),
        };
    }

    #[test]
    fn loader_call_request() {
        let mut plugins = HashMap::new();

        function_plugin_v1::insert_plugin(&mut plugins);

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

        loader
            .load_plugin_now(
                get_plugin_path(function_plugin_v1::FILENAME)
                    .to_str()
                    .unwrap(),
            )
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

    #[test]
    fn parallel_call_request() {
        let mut plugins = HashMap::new();

        parallel_plugins::one_plugin::insert_plugin(&mut plugins);
        parallel_plugins::two_plugin::insert_plugin(&mut plugins);

        let mut loader = SimpleLoader::new();
        loader.context(move |mut ctx| {
            ctx.register_request(Request::new(
                "main".to_string(),
                vec![VariableType::I32],
                None,
            ));
            ctx.register_manager(MockManager::from_plugins(plugins))
                .unwrap();
        });

        loader
            .load_plugins([
                get_plugin_path(format!(
                    "{path}/{filename}",
                    path = parallel_plugins::PATH,
                    filename = parallel_plugins::one_plugin::FILENAME,
                ))
                .to_str()
                .unwrap(),
                get_plugin_path(format!(
                    "{path}/{filename}",
                    path = parallel_plugins::PATH,
                    filename = parallel_plugins::two_plugin::FILENAME,
                ))
                .to_str()
                .unwrap(),
            ])
            .unwrap();

        let (duration, result) = benchmark(|| loader.call_request("main", &[10.into()]));
        println!("Single: {duration:?}");

        if let Err(e) = result.unwrap().get(0).unwrap() {
            panic!("{:?}: {}", e, e.to_string());
        }

        let (duration, result) = benchmark(|| loader.par_call_request("main", &[10.into()]));
        println!("Parallel: {duration:?}");

        if let Err(e) = result.unwrap().get(0).unwrap() {
            panic!("{:?}: {}", e, e.to_string());
        }
    }

    #[test]
    fn call_plugin_function() {
        let mut plugins = HashMap::new();

        plugin_function::circle::insert_plugin(&mut plugins);
        plugin_function::square::insert_plugin(&mut plugins);
        plugin_function::paint::insert_plugin(&mut plugins);

        let mut loader = SimpleLoader::new();
        loader.context(move |mut ctx| {
            ctx.register_manager(MockManager::from_plugins(plugins))
                .unwrap();
        });

        let paths = [
            get_plugin_path(format!(
                "{path}/{filename}",
                path = plugin_function::PATH,
                filename = plugin_function::circle::FILENAME,
            )),
            get_plugin_path(format!(
                "{path}/{filename}",
                path = plugin_function::PATH,
                filename = plugin_function::square::FILENAME,
            )),
            get_plugin_path(format!(
                "{path}/{filename}",
                path = plugin_function::PATH,
                filename = plugin_function::paint::FILENAME,
            )),
        ];

        loader
            .load_plugins(paths.iter().map(|path| path.to_str().unwrap()))
            .unwrap();

        let plugin = loader
            .get_plugin("paint", &Version::parse("1.0.0").unwrap())
            .unwrap();

        // Circle
        plugin
            .call_function("paint", &[true.into()])
            .unwrap()
            .unwrap();

        println!();

        // Square
        plugin
            .call_function("paint", &[false.into()])
            .unwrap()
            .unwrap();
    }
}
