use std::collections::HashMap;
use std::io::{self, Write};

use plux_mock::{MockManager, MockPlugin};
use plux_rs::{function_call, prelude::*};

use crate::plugins::cli::{self, help_v1, user_info_v1};
use crate::plugins::utils::get_plugin_path;

mod plugins;

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
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
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
    let mut loader = SimpleLoader::new();

    // Create mock plugins in memory
    let mut plugins: HashMap<Bundle, MockPlugin<FunctionOutput>> = HashMap::new();

    // Define plugins
    cli::help_v1::insert_plugin(&mut plugins);
    cli::user_info_v1::insert_plugin(&mut plugins);

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the Mock plugin manager with our in-memory plugins
        ctx.register_manager(MockManager::from_plugins(plugins))?;

        // Create a simple user database
        let users = HashMap::from([
            ("admin".to_string(), ("Administrator".to_string(), 30)),
            ("user1".to_string(), ("John Doe".to_string(), 25)),
            ("user2".to_string(), ("Jane Smith".to_string(), 28)),
        ]);

        // Register functions that will be available to plugins
        ctx.register_function(execute_command("".to_string()));
        ctx.register_function(get_user_input("> ".to_string()));
        ctx.register_function(get_user_info(users));

        // Define requests that plugins must implement
        ctx.register_request(Request::new(
            "get_commands".to_string(),
            vec![],
            Some(VariableType::List),
        ));
        ctx.register_request(Request::new(
            "handle_command".to_string(),
            vec![VariableType::String, VariableType::List],
            Some(VariableType::I32),
        ));

        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    // Load CLI plugins
    let paths = [help_v1::FILENAME, user_info_v1::FILENAME]
        .into_iter()
        .map(|filename| get_plugin_path(format!("cli/{}", filename)))
        .collect::<Vec<_>>();

    let bundles = loader
        .load_plugins(paths.iter().map(|path| path.to_str().unwrap()))
        .unwrap();

    // Get function "get_user_input"
    let registry = loader.get_registry();
    let get_user_input = registry
        .iter()
        .find(|f| f.name() == "get_user_input")
        .unwrap();

    let list_commands = loader
        .call_request("get_commands", &[])
        .unwrap()
        .into_iter()
        .try_fold(vec!["exit".to_string()], |mut list_commands, command| {
            let plugin_commands = command?
                .ok_or("Failed to get command")?
                .try_parse::<Vec<String>>()?;
            list_commands.extend(plugin_commands);
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(list_commands)
        })
        .unwrap();

    println!("Loaded {} CLI plugins", bundles.len());
    println!("Available commands: {}", list_commands.join(", "));
    println!("Type 'help' for more information\n");

    loop {
        // Get user input
        let input = function_call!(get_user_input)
            .unwrap()
            .unwrap()
            .parse::<String>();
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

            // Call the plugin's 'handle_command' request
            match plugin
                .call_request("handle_command", &[command.clone().into(), args.into()])
                .unwrap()
            {
                Ok(Some(Variable::I32(exit_code))) => match exit_code {
                    // Error exit code
                    i32::MIN..=-1 => {
                        println!(
                            "Command '{}' executed with exit code: {}",
                            command, exit_code
                        );
                        handled = true;
                        break;
                    }
                    // Not handled exit code
                    1 => continue,
                    // Success exit code
                    _ => {}
                },
                Ok(None | Some(_)) => continue,
                Err(e) => {
                    eprintln!("Plugin '{}' error for command '{}': {}", bundle, command, e);
                    handled = true;
                    break;
                }
            }
        }

        if !handled && command != "exit" {
            println!(
                "Unknown command: '{}'. Type 'help' for available commands.",
                command
            );
        }
    }

    // Stop the loader
    loader.stop().unwrap();

    println!("Goodbye!");
    Ok(())
}
