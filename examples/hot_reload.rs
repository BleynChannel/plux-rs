use plux_mock::MockManager;
use plux_rs::prelude::*;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use crate::plugins::hot_reload_v1;
use crate::plugins::utils::get_plugin_path;

mod plugins;

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
    // Create mock plugins in memory
    let mut plugins = HashMap::new();

    // Define hot reload plugin
    hot_reload_v1::insert_plugin(&mut plugins);

    // Create a new plugin loader
    let mut loader = SimpleLoader::new();

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the Mock plugin manager with our in-memory plugins
        ctx.register_manager(MockManager::from_plugins(plugins))?;

        // Register functions that will be available to plugins
        ctx.register_function(get_timestamp());

        // Define a request that plugins must implement
        ctx.register_request(Request::new(
            "status".to_string(),
            vec![],
            Some(VariableType::I32),
        ));

        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    // Load a plugin that we'll hot-reload
    let mut bundle = loader
        .load_plugin_now(get_plugin_path(hot_reload_v1::FILENAME).to_str().unwrap())
        .unwrap();

    println!("Plugin loaded. Starting hot-reload demonstration...");
    println!("Modify the plugin file and see changes without restarting the application.");

    // Simulate a long-running application that periodically calls the plugin
    for i in 0..10 {
        // Simulate plugin update after 5 iterations
        if i == 5 {
            println!("Simulating plugin update...");

            // `unregister_plugin_by_bundle` will unload and unregister the plugin
            loader.unregister_plugin_by_bundle(&bundle).unwrap();

            // Re-register the plugin by creating a new one
            bundle = loader
                .load_plugin_now(get_plugin_path(hot_reload_v1::FILENAME).to_str().unwrap())
                .unwrap();

            println!("Plugin reloaded with new version.");

            // Since we can't directly re-register a plugin, we'll just show how it would work conceptually
            // In a real MockManager scenario, you'd need to recreate the manager or use a different approach
            // The new bundle now points to the reloaded plugin
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
