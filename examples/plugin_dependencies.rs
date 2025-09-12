use plux_rs::prelude::*;
use plux_lua_manager::LuaManager;

// A function that will be available to plugins
#[plux_rs::function]
fn log_message(_: (), message: &i32) -> i32 {
    println!("Host log: {}", message);
    *message
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new plugin loader
    let mut loader = Loader::new();

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the Lua plugin manager
        ctx.register_manager(LuaManager::new())?;

        // Register functions that will be available to plugins
        ctx.register_function(log_message());

        // Define a request that plugins must implement
        ctx.register_request(Request::new("main".to_string(), vec![], None));
        
        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    // Load plugins with dependencies
    // The loader will automatically resolve dependencies
    let bundles = loader.load_plugins(vec![
        "examples/plugins/dependency/plugin_a-v1.0.0.lua",
        "examples/plugins/dependency/plugin_b-v1.0.0.lua",
        "examples/plugins/dependency/plugin_c-v1.0.0.lua",
        "examples/plugins/dependency/plugin_d-v1.0.0.lua",
    ]).unwrap();

    println!("Loaded {} plugins with dependencies", bundles.len());

    // Access a specific plugin that has dependencies
    if let Some(plugin) = loader.get_plugin("plugin_c", &semver::Version::parse("1.0.0").unwrap()) {
        println!("Plugin with dependencies loaded: {}", plugin.info().bundle);
        
        // Call the 'main' request defined in the plugin
        if let Err(e) = plugin.call_request("main", &[]).unwrap() {
            eprintln!("Plugin error: {}", e);
        }
    }

    // Show all loaded plugins
    for bundle in &bundles {
        let plugin = loader.get_plugin_by_bundle(bundle).unwrap();
        println!("Plugin: {} (Info: {})", bundle, plugin.info().info);
    }

    Ok(())
}