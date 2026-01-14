use plux_mock::MockManager;
use plux_rs::prelude::*;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use crate::plugins::{parallel_one_v1, parallel_two_v1, utils::get_plugin_path};

mod plugins;

// A function that simulates some work
#[plux_rs::function]
fn process_data(_: (), value: &i32) -> i32 {
    // Simulate some processing time
    thread::sleep(Duration::from_millis(100));
    value * 2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create mock plugins in memory
    let mut plugins = HashMap::new();

    // Define plugins
    parallel_one_v1::insert_plugin(&mut plugins);
    parallel_two_v1::insert_plugin(&mut plugins);

    // Create a new plugin loader
    let mut loader = SimpleLoader::new();

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the Mock plugin manager with our in-memory plugins
        ctx.register_manager(MockManager::from_plugins(plugins))?;

        // Register functions that will be available to plugins
        ctx.register_function(process_data());

        // Define a request that plugins must implement
        ctx.register_request(Request::new(
            "process".to_string(),
            vec![VariableType::I32],
            Some(VariableType::I32),
        ));

        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    // Load multiple plugins at once
    let paths = [parallel_one_v1::FILENAME, parallel_two_v1::FILENAME]
        .into_iter()
        .map(|filename| get_plugin_path(filename))
        .collect::<Vec<_>>();

    let bundles = loader
        .load_plugins(paths.iter().map(|path| path.to_str().unwrap()))
        .unwrap();

    println!("Loaded {} plugins", bundles.len());

    // Call plugins sequentially
    let start = std::time::Instant::now();
    for bundle in &bundles {
        let plugin = loader.get_plugin_by_bundle(bundle).unwrap();
        if let Ok(Some(result)) = plugin.call_request("process", &[10.into()]).unwrap() {
            println!("Sequential result from {}: {:?}", bundle, result);
        }
    }
    let sequential_duration = start.elapsed();

    // Call plugins in parallel
    let start = std::time::Instant::now();
    let results = loader.par_call_request("process", &[10.into()]).unwrap();
    let parallel_duration = start.elapsed();

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(Some(value)) => println!("Parallel result {}: {:?}", i, value),
            Ok(None) => println!("Parallel result {}: No return value", i),
            Err(e) => println!("Parallel result {}: Error - {}", i, e),
        }
    }

    println!("Sequential execution took: {:?}", sequential_duration);
    println!("Parallel execution took: {:?}", parallel_duration);

    Ok(())
}
