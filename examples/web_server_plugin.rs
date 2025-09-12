use std::collections::HashMap;

use plux_lua_manager::LuaManager;
use plux_rs::prelude::*;

// A function that simulates a web server response
#[plux_rs::function]
fn render_banner(template: &String, data: &Variable) -> String {
    let data = data.clone().parse::<Vec<Variable>>();

    let user_id = data[0].clone().parse::<i32>();
    let name = data[1].clone().parse::<String>();
    let age = data[2].clone().parse::<i32>();

    template
        .replace("{{name}}", &name)
        .replace("{{age}}", &age.to_string())
        .replace("{{user_id}}", &user_id.to_string())
}

// A function that simulates database access
#[plux_rs::function]
fn get_user_data(users: &HashMap<i32, (String, i32)>, user_id: &i32) -> Variable {
    let (name, age) = users.get(user_id).unwrap();
    Variable::List(vec![name.clone().into(), age.clone().into()])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new plugin loader
    let mut loader = Loader::new();

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the Lua plugin manager
        ctx.register_manager(LuaManager::new())?;

        // Here we can create our own banner templates
        let banner = r#"
        <div>
            <h1>{{name}}</h1>
            <p>{{age}}</p>
            <p>{{user_id}}</p>
        </div>
        "#;

        let users = HashMap::from([
            (1, ("John Doe".to_string(), 30)),
            (2, ("Jane Smith".to_string(), 25)),
            (3, ("Bob Johnson".to_string(), 35)),
        ]);

        // Register functions that will be available to plugins
        ctx.register_function(render_banner(banner.to_string()));
        ctx.register_function(get_user_data(users));

        // Define requests that plugins must implement
        ctx.register_request(Request::new(
            "handle_request".to_string(),
            vec![VariableType::I32],
            Some(VariableType::String),
        ));

        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    // Load web server plugins
    let bundles = loader
        .load_plugins(vec![
            "examples/plugins/web/user_profile-v1.0.0.lua",
            "examples/plugins/web/dashboard-v1.0.0.lua",
            "examples/plugins/web/admin_panel-v1.0.0.lua",
        ])
        .unwrap();

    println!("Loaded {} web server plugins", bundles.len());

    // Simulate handling web requests
    let requests = vec![1, 2, 3];

    for request in &requests {
        // In a real web server, you would route requests to appropriate plugins
        // Here we're just demonstrating the concept
        for bundle in &bundles {
            let plugin = loader.get_plugin_by_bundle(bundle).unwrap();
            match plugin
                .call_request("handle_request", &[(*request).into()])
                .unwrap()
            {
                Ok(Some(Variable::String(response))) => {
                    println!("Request '{}' handled by plugin '{}'", request, bundle);
                    println!("Response: {}", response);
                    // break; // In a real server, you might stop after the first match
                }
                Ok(None | Some(_)) => continue, // Plugin didn't handle this request
                Err(e) => eprintln!("Plugin '{}' error for request '{}': {}", bundle, request, e),
            }
        }
    }

    // Unload all plugins
    for bundle in &bundles {
        loader.unload_plugin_by_bundle(bundle).unwrap();
    }

    // Stop the loader
    loader.stop().unwrap();

    Ok(())
}
