use plux_mock::MockManager;
use plux_rs::prelude::*;
use std::collections::HashMap;

use crate::plugins::{dependency, utils::get_plugin_path};

mod plugins;

// A function that will be available to plugins
#[plux_rs::function]
fn log_message(_: (), message: &i32) -> i32 {
    println!("Host log: {}", message);
    *message
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create mock plugins in memory with dependencies
    let mut plugins = HashMap::new();

    // Define plugins
    dependency::plugin_a_v1::insert_plugin(&mut plugins);
    dependency::plugin_b_v1::insert_plugin(&mut plugins);
    dependency::plugin_c_v1::insert_plugin(&mut plugins);
    dependency::plugin_d_v1::insert_plugin(&mut plugins);

    // Create a new plugin loader
    let mut loader = SimpleLoader::new();

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the Mock plugin manager with our in-memory plugins
        ctx.register_manager(MockManager::from_plugins(plugins))?;

        // Register functions that will be available to plugins
        ctx.register_function(log_message());

        // Define a request that plugins must implement
        ctx.register_request(Request::new("main".to_string(), vec![], None));

        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    // Load multiple plugins at once
    let paths = [
        dependency::plugin_a_v1::FILENAME,
        dependency::plugin_b_v1::FILENAME,
        dependency::plugin_c_v1::FILENAME,
        dependency::plugin_d_v1::FILENAME,
    ]
    .into_iter()
    .map(|filename| get_plugin_path(format!("dependency/{}", filename)))
    .collect::<Vec<_>>();

    let bundles = loader
        .load_plugins(paths.iter().map(|path| path.to_str().unwrap()))
        .unwrap();

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
