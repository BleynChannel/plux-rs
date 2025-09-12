use plux_rs::prelude::*;
use plux_lua_manager::LuaManager;
use std::thread;
use std::time::Duration;

// A function that will be available to plugins
#[plux_rs::function]
fn get_timestamp(_: ()) -> i32 {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32;
    timestamp
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new plugin loader
    let mut loader = Loader::new();

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the Lua plugin manager
        ctx.register_manager(LuaManager::new())?;

        // Register functions that will be available to plugins
        ctx.register_function(get_timestamp());

        // Define a request that plugins must implement
        ctx.register_request(Request::new("status".to_string(), vec![], Some(VariableType::I32)));
        
        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    // Load a plugin that we'll hot-reload
    let bundle = loader.load_plugin_now("examples/plugins/hot_reload-v1.0.0.lua").unwrap();

    println!("Plugin loaded. Starting hot-reload demonstration...");
    println!("Modify the plugin file and see changes without restarting the application.");

    // Simulate a long-running application that periodically calls the plugin
    for i in 0..10 {
        // Check if the plugin file has been modified and reload if needed
        // Note: In a real implementation, you would monitor the file system for changes
        if i == 5 {
            println!("Simulating plugin update...");
            
            // `unregister_plugin_by_bundle` will unload and unregister the plugin
            loader.unregister_plugin_by_bundle(&bundle).unwrap();
            
            // `load_plugin_now` will load the plugin again
            loader.load_plugin_now("examples/plugins/hot_reload-v1.0.0.lua").unwrap();
        }

        // Call the plugin
        if let Some(plugin) = loader.get_plugin_by_bundle(&bundle) {
            match plugin.call_request("status", &[]).unwrap() {
                Ok(Some(result)) => println!("Plugin response: {}", result),
                Ok(None) => println!("Plugin returned no result"),
                Err(e) => eprintln!("Plugin error: {}", e),
            }
        }

        // Wait before next iteration
        thread::sleep(Duration::from_secs(2));
    }
    
    Ok(())
}