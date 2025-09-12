use plux_rs::prelude::*;
use plux_lua_manager::LuaManager;

// A simple function that will be available to plugins
#[plux_rs::function]
fn greet(name: &String, age: &i32) {
    println!("Hello, {}! You are {} years old.", name, age);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new plugin loader
    let mut loader = Loader::new();

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the Lua plugin manager
        ctx.register_manager(LuaManager::new())?;

        // Register functions that will be available to plugins
        ctx.register_function(greet("world".to_string()));

        // Define a request that plugins must implement
        ctx.register_request(Request::new("main".to_string(), vec![], None));
        
        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    // Load a simple plugin
    // Note: You'll need to have a plugin file in the correct format
    // For this example, we'll assume there's a "hello-v1.0.0.lua" plugin
    let bundle = match loader.load_plugin_now("examples/plugins/hello-v1.0.0.lua") {
        Ok(bundle) => bundle,
        Err((Some(e), _)) => return Err(e.into()),
        Err((None, Some(e))) => return Err(e.into()),
        Err((None, None)) => return Err("Unknown error".into()),
    };

    // Access the loaded plugin
    let plugin = loader.get_plugin_by_bundle(&bundle).ok_or("Plugin not found")?;
    println!("Plugin loaded - Path: {:?}, Bundle: {}", 
             plugin.info().path, 
             plugin.info().bundle);

    // Call the 'main' request defined in the plugin
    if let Err(e) = plugin.call_request("main", &[])? {
        eprintln!("Plugin error: {}", e.to_string());
    }

    // Unload the plugin when done (optional)
    loader.unload_plugin_by_bundle(&bundle)?;
    
    // Stop the loader (optional)
    loader.stop()?;
    
    Ok(())
}