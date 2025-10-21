#[cfg(test)]
/// Tests for the MockManager functionality
/// This module tests the MockManager implementation with MockPlugin
mod manager {
    use std::collections::HashMap;

    use plux_mock::{MockManager, MockPlugin};
    use plux_rs::prelude::Request;

    pub mod plugin {
        use plux_rs::{Api, Bundle, StdInfo, function::FunctionOutput, function_call};
        use std::{
            path::PathBuf,
            sync::{LazyLock, Mutex},
        };

        // Plugin metadata
        pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("plugin-v1.0.0.mock"));
        pub static BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(PATH.as_os_str()).unwrap());
        pub static INFO: LazyLock<StdInfo> = LazyLock::new(|| StdInfo::new());

        // Plugin variables
        static USER: Mutex<String> = Mutex::new(String::new());

        /// A sample plugin function that prints a greeting
        #[plux_rs::function]
        fn say_hello(_: ()) -> () {
            println!("Hello, {}!", USER.lock().unwrap());
        }

        /// Plugin entrypoint function that gets the user name and stores it
        #[plux_rs::function]
        fn entrypoint(
            api: &Api<FunctionOutput, StdInfo>,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let get_user = api
                .get_registry()
                .iter()
                .find(|f| f.name() == "get_user")
                .ok_or("get_user not found")?;
            let result = function_call!(get_user, 1)?.ok_or("Failed to get user")?;
            *USER.lock().unwrap() = result.try_parse::<String>()?;
            Ok(())
        }
    }

    /// Sample user data for testing
    const USERS: [&'static str; 3] = ["Poul", "John", "Jane"];

    /// Function to get a user by ID
    #[plux_rs::function]
    fn get_user(_: (), id: &i32) -> String {
        USERS[*id as usize].to_string()
    }
    
    #[test]
    fn test() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut plugins = HashMap::new();

        // Create mock plugin with a callback that registers functions and requests
        plugins.insert(
            plugin::BUNDLE.clone(),
            MockPlugin::new(plugin::INFO.clone(), |mut context, api| {
                let plugin = api
                    .get_plugin_mut_by_bundle(&plugin::BUNDLE)
                    .ok_or("Plugin not found")?;
                plugin.register_function(plugin::say_hello())?;

                context.register_request(plugin::entrypoint(api))?;

                Ok(())
            }),
        );

        // Create a MockManager with the mock plugins
        let manager = MockManager::from_plugins(plugins);

        // Set up a loader with the mock manager and register functions/requests
        let mut loader = plux_rs::Loader::new();
        loader.context(move |mut ctx| {
            ctx.register_function(get_user());
            ctx.register_request(Request::new("entrypoint".to_string(), vec![], None));

            ctx.register_manager(manager)?;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        })?;

        // Load the mock plugin
        loader.load_plugin_now(
            format!(
                "tests/plugins/{}",
                plugin::PATH.as_os_str().to_str().unwrap()
            )
            .as_str(),
        )?;

        // Call the entrypoint request to setup the plugin state
        loader.call_request("entrypoint", &[])?;

        // Call the say_hello function to verify the plugin is working
        let plugin = loader
            .get_plugin_by_bundle(&plugin::BUNDLE)
            .ok_or("Plugin not found")?;
        let _ = plugin.call_function("say_hello", &[])?;

        Ok(())
    }
}
