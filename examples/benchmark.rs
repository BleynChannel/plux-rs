use plux_mock::{MockManager, MockPlugin};
use plux_rs::prelude::*;
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

use crate::plugins::benchmark;
use crate::plugins::utils::get_plugin_path;

mod plugins;

// A simple mathematical function for plugins to use
#[plux_rs::function]
fn calculate(_: (), a: &i32, b: &i32) -> i32 {
    thread::sleep(Duration::from_millis(10));
    a + b * 2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create mock plugins in memory
    let mut plugins: HashMap<Bundle, MockPlugin<FunctionOutput>> = HashMap::new();

    // Define plugins
    benchmark::plugin1_v1::insert_plugin(&mut plugins);
    benchmark::plugin2_v1::insert_plugin(&mut plugins);
    benchmark::plugin3_v1::insert_plugin(&mut plugins);

    // Create a new plugin loader
    let mut loader = SimpleLoader::new();

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the Mock plugin manager with our in-memory plugins
        ctx.register_manager(MockManager::from_plugins(plugins))?;

        // Register functions that will be available to plugins
        ctx.register_function(calculate());

        // Define requests that plugins must implement
        ctx.register_request(Request::new(
            "compute".to_string(),
            vec![VariableType::I32, VariableType::I32],
            Some(VariableType::I32),
        ));

        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    let paths = [
        benchmark::plugin1_v1::FILENAME,
        benchmark::plugin2_v1::FILENAME,
        benchmark::plugin3_v1::FILENAME,
    ]
    .into_iter()
    .map(|filename| get_plugin_path(format!("benchmark/{}", filename)))
    .collect::<Vec<_>>();

    // Measure plugin loading time
    let start = Instant::now();
    let bundles = loader
        .load_plugins(paths.iter().map(|path| path.to_str().unwrap()))
        .unwrap();
    let load_time = start.elapsed();

    println!("Loaded {} plugins in {:?}", bundles.len(), load_time);

    // Measure plugin execution time
    let start = Instant::now();
    for _ in 0..100 {
        for bundle in &bundles {
            let plugin = loader.get_plugin_by_bundle(bundle).unwrap();
            let _ = plugin
                .call_request("compute", &[10.into(), 20.into()])
                .unwrap();
        }
    }
    let exec_time = start.elapsed();

    println!("Executed plugin functions 3000 times in {:?}", exec_time);

    // Measure parallel execution time
    let start = Instant::now();
    for _ in 0..100 {
        let _ = loader
            .par_call_request("compute", &[10.into(), 20.into()])
            .unwrap();
    }
    let parallel_exec_time = start.elapsed();

    println!(
        "Executed plugin functions 3000 times in parallel in {:?}",
        parallel_exec_time
    );

    // Unload all plugins
    for bundle in &bundles {
        loader.unload_plugin_by_bundle(bundle).unwrap();
    }

    // Stop the loader
    loader.stop().unwrap();

    Ok(())
}
