use std::collections::HashMap;
use std::io::{self, Write};

use plux_lua_manager::LuaManager;
use plux_rs::prelude::*;

// A function that simulates a CLI command executor
#[plux_rs::function]
fn execute_command(command: &String, args: &Variable) -> i32 {
    let args = args.clone().parse::<Vec<Variable>>();
    
    println!("Executing command: {}", command);
    for arg in args {
        match arg {
            Variable::String(s) => println!("  Arg: {}", s),
            Variable::I32(i) => println!("  Arg: {}", i),
            _ => println!("  Arg: {:?}", arg),
        }
    }
    
    // Return exit code (0 for success)
    0
}

// A function that gets user input
#[plux_rs::function]
fn get_user_input(prompt: &String) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// A function that provides access to a simple database
#[plux_rs::function]
fn get_user_info(users: &HashMap<String, (String, i32)>, username: &String) -> Variable {
    match users.get(username) {
        Some((name, age)) => Variable::List(vec![name.clone().into(), (*age).into()]),
        None => Variable::List(vec!["Unknown".into(), 0.into()]),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new plugin loader
    let mut loader = Loader::new();

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the Lua plugin manager
        ctx.register_manager(LuaManager::new())?;

        // Create a simple user database
        let users = HashMap::from([
            ("admin".to_string(), ("Administrator".to_string(), 30)),
            ("user1".to_string(), ("John Doe".to_string(), 25)),
            ("user2".to_string(), ("Jane Smith".to_string(), 28)),
        ]);

        // Register functions that will be available to plugins
        ctx.register_function(execute_command("".to_string()));
        ctx.register_function(get_user_input("Enter command: ".to_string()));
        ctx.register_function(get_user_info(users));

        // Define requests that plugins must implement
        ctx.register_request(Request::new(
            "handle_command".to_string(),
            vec![VariableType::String, VariableType::List],
            Some(VariableType::I32),
        ));

        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    // Load CLI plugins
    let bundles = loader
        .load_plugins(vec![
            "examples/plugins/cli/help-v1.0.0.lua",
            "examples/plugins/cli/user_info-v1.0.0.lua",
        ])
        .unwrap();

    println!("Loaded {} CLI plugins", bundles.len());
    println!("Available commands: help, user-info, exit");
    println!("Type 'help' for more information\n");

    loop {
        // Get user input (using standard Rust functions, not Plux functions)
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            continue;
        }
        
        let command = parts[0].to_string();
        
        // Handle built-in commands
        if command == "exit" {
            break;
        }
        
        // Handle plugin commands
        let mut handled = false;
        for bundle in &bundles {
            let plugin = loader.get_plugin_by_bundle(bundle).unwrap();
            
            // Convert arguments to Variables
            let args: Vec<Variable> = parts[1..].iter().map(|&s| s.to_string().into()).collect();
            
            match plugin
                .call_request("handle_command", &[command.clone().into(), args.into()])
                .unwrap()
            {
                Ok(Some(Variable::I32(exit_code))) => {
                    println!("Command '{}' executed with exit code: {}", command, exit_code);
                    handled = true;
                    break;
                }
                Ok(None | Some(_)) => continue,
                Err(e) => {
                    eprintln!("Plugin '{}' error for command '{}': {}", bundle, command, e);
                    handled = true;
                    break;
                }
            }
        }
        
        if !handled && command != "exit" {
            println!("Unknown command: '{}'. Type 'help' for available commands.", command);
        }
    }

    // Unload all plugins
    for bundle in &bundles {
        loader.unload_plugin_by_bundle(bundle).unwrap();
    }

    // Stop the loader
    loader.stop().unwrap();

    println!("Goodbye!");
    Ok(())
}