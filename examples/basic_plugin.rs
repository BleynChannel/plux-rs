use plux_mock::MockManager;
use plux_rs::prelude::*;
use std::collections::HashMap;

use crate::plugins::{hello_v1, utils::get_plugin_path};

mod plugins;

// A simple function that will be available to plugins
#[plux_rs::function]
fn greet(name: &String, age: &i32) {
    println!("Hello, {}! You are {} years old.", name, age);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create mock plugins in memory
    let mut plugins = HashMap::new();
    
    // Insert the hello_v1 plugin into our in-memory collection
    hello_v1::insert_plugin(&mut plugins);

    // Create a new plugin loader
    let mut loader = SimpleLoader::new();

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the Mock plugin manager with our in-memory plugins
        ctx.register_manager(MockManager::from_plugins(plugins))?;

        // Register functions that will be available to plugins
        ctx.register_function(greet("world".to_string()));

        // Define a request that plugins must implement
        ctx.register_request(Request::new("main".to_string(), vec![], None));
        
        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    // Load the plugin
    let plugin = loader
            .load_plugin_now(
                get_plugin_path(hello_v1::FILENAME)
                    .to_str()
                    .unwrap(),
            )
            .map(|bundle| loader.get_plugin_by_bundle(&bundle).unwrap())
            .unwrap();
    println!("Plugin loaded - Path: {:?}, Bundle: {}", 
             plugin.info().path, 
             plugin.info().bundle);

    // Call the 'main' request defined in the plugin
    if let Err(e) = plugin.call_request("main", &[])? {
        eprintln!("Plugin error: {}", e.to_string());
    }

    // Unload the plugin when done (optional)
    loader.unload_plugin_by_bundle(&hello_v1::BUNDLE)?;
    
    // Stop the loader (optional)
    loader.stop()?;
    
    Ok(())
}