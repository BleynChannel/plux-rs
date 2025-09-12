use plux_rs::prelude::*;
use plux_lua_manager::LuaManager;
use std::time::{Instant, Duration};
use std::thread;

// A simple mathematical function for plugins to use
#[plux_rs::function]
fn calculate(_: (), a: &i32, b: &i32) -> i32 {
    thread::sleep(Duration::from_millis(10));
    a + b * 2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new plugin loader
    let mut loader = Loader::new();

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the Lua plugin manager
        ctx.register_manager(LuaManager::new())?;

        // Register functions that will be available to plugins
        ctx.register_function(calculate());

        // Define requests that plugins must implement
        ctx.register_request(Request::new(
            "compute".to_string(), 
            vec![VariableType::I32, VariableType::I32], 
            Some(VariableType::I32)
        ));
        
        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    // Measure plugin loading time
    let start = Instant::now();
    let bundles = loader.load_plugins(vec![
        "examples/plugins/benchmark/plugin1-v1.0.0.lua",
        "examples/plugins/benchmark/plugin2-v1.0.0.lua",
        "examples/plugins/benchmark/plugin3-v1.0.0.lua",
    ]).unwrap();
    let load_time = start.elapsed();

    println!("Loaded {} plugins in {:?}", bundles.len(), load_time);

    // Measure plugin execution time
    let start = Instant::now();
    for _ in 0..100 {
        for bundle in &bundles {
            let plugin = loader.get_plugin_by_bundle(bundle).unwrap();
            let _ = plugin.call_request("compute", &[10.into(), 20.into()]).unwrap();
        }
    }
    let exec_time = start.elapsed();

    println!("Executed plugin functions 3000 times in {:?}", exec_time);

    // Measure parallel execution time
    let start = Instant::now();
    for _ in 0..100 {
        let _ = loader.par_call_request("compute", &[10.into(), 20.into()]).unwrap();
    }
    let parallel_exec_time = start.elapsed();

    println!("Executed plugin functions 3000 times in parallel in {:?}", parallel_exec_time);

    // Unload all plugins
    for bundle in &bundles {
        loader.unload_plugin_by_bundle(bundle).unwrap();
    }
    
    // Stop the loader
    loader.stop().unwrap();
    
    Ok(())
}