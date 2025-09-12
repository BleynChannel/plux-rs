use plux_rs::prelude::*;
use plux_lua_manager::LuaManager;
use std::thread;
use std::time::Duration;

// A function that simulates some work
#[plux_rs::function]
fn process_data(_: (), value: &i32) -> i32 {
    // Simulate some processing time
    thread::sleep(Duration::from_millis(100));
    value * 2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new plugin loader
    let mut loader = Loader::new();

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the Lua plugin manager
        ctx.register_manager(LuaManager::new())?;

        // Register functions that will be available to plugins
        ctx.register_function(process_data());

        // Define a request that plugins must implement
        ctx.register_request(Request::new(
            "process".to_string(), 
            vec![VariableType::I32], 
            Some(VariableType::I32)
        ));
        
        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    // Load multiple plugins at once
    let bundles = loader.load_plugins(vec![
        "examples/plugins/parallel_one-v1.0.0.lua",
        "examples/plugins/parallel_two-v1.0.0.lua",
    ]).unwrap();

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