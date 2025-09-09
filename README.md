# Plux (formerly August Plugin System)

[![Crates.io](https://img.shields.io/crates/v/plux-rs)](https://crates.io/crates/plux-rs)
[![Documentation](https://docs.rs/plux-rs/badge.svg)](https://docs.rs/plux-rs)
[![License](https://img.shields.io/crates/l/plux-rs)](LICENSE)
[![Rust](https://github.com/BleynChannel/plux-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/BleynChannel/plux-rs/actions)

Plux is a comprehensive plugin system for Rust applications, offering a robust and flexible architecture for extending application functionality through plugins. Designed with modularity and performance in mind, Plux enables seamless integration of third-party code while maintaining security and stability.

## Key Features

- **Language Agnostic**: Write plugins in any programming language
- **Hot Reloading**: Update plugins without restarting the host application
- **Dynamic Loading**: Load and unload plugins at runtime
- **Type Safety**: Rust's type system ensures safe plugin interactions
- **Cross-Platform**: Works on all major platforms (Windows, macOS, Linux)
- **Performance Optimized**: Efficient loading and caching of plugins
- **Isolated Execution**: Secure sandboxing for plugin execution
- **Parallel Operations**: Concurrent plugin execution for better performance

## Architecture Overview

Plux is built on a modular architecture that separates concerns between different components, enabling flexible and maintainable plugin management.

### Core Components

#### 🔌 Plugin

Self-contained modules that extend application functionality. Each plugin includes:
- Executable code
- Configuration files
- Required resources (libraries, assets, documentation)
- Platform-specific binaries (when needed)

#### ⚙️ Engine

The central component responsible for:
- Dynamic loading and unloading of plugins
- Plugin lifecycle management
- Code execution isolation
- Performance monitoring
- Security enforcement
- API exposure to the host application

#### 🔗 Manager

Specialized adapters that provide:
- Standardized interfaces for plugin integration
- Type validation and safety
- Error handling mechanisms
- Communication between plugins and the engine

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
plux = { version = "1.0.0", features = ["full"] }
```

## Quick Start

### Loading a Simple Plugin

1. Add the required dependencies to your `Cargo.toml`:

```toml
[dependencies]
plux = { version = "1.0.0", features = ["derive"] }
plux-lua-manager = "0.1.0"  # For running Lua plugins
```

2. Create your main application:

```rust
use plux::prelude::*;
use plux_lua_manager::LuaManager;

// Declare a function that will be available to all plugins
// The `derive` feature is required for the `plux::function` macro
#[plux::function]
fn add(_: (), a: &i32, b: &i32) -> i32 {
    a + b
}

fn main() {
    // Create a new plugin loader
    let mut loader = Loader::new();

    // Configure the loader with context
    loader.context(move |mut ctx| {
        // Register the plugin manager
        // You can register multiple managers for different plugin types (Lua, Rust, WASM, etc.)
        ctx.register_manager(LuaManager::new())?;

        // Register functions that will be available to plugins
        ctx.register_function(add());

        // Define a request that plugins must implement
        ctx.register_request(Request::new("main".to_string(), vec![], None));
        
        Ok::<(), Box<dyn std::error::Error>>(())
    }).unwrap();

    // Load a single plugin
    // Format: {id}-v{version}.{format}
    // The format is determined by the plugin manager (e.g., "lua" for LuaManager)
    let bundle = loader.load_plugin_now("my_plugin-v1.0.0.lua").unwrap();

    // Alternatively, load multiple plugins at once
    loader.load_plugins(vec![
        "calculator-v1.0.0.lua",
        "logger-v1.0.0.lua",
        "paint-v1.0.0.lua",
    ]).unwrap();

    // Access a loaded plugin by its bundle name
    let plugin = loader.get_plugin_by_bundle(&bundle).unwrap();
    println!("Plugin loaded - Path: {:?}, Bundle: {}", 
             plugin.info().path, 
             plugin.info().bundle);

    // Call the 'main' request defined in the plugin
    if let Err(e) = plugin.call_request("main", &[]).unwrap() {
        eprintln!("Plugin error: {}", e);
    }

    // Call a function exposed by the plugin
    if let Ok(Some(result)) = plugin.call_function("echo", &["Hello world".into()]).unwrap() {
        println!("Plugin responded: {}", result);
    }

    // Unload the plugin when done (optional)
    loader.unload_plugin_by_bundle(&bundle).unwrap();
    
    // Stop the loader (optional)
    loader.stop().unwrap();
}
```

### Creating a Custom Plugin Manager

To create a custom plugin manager, implement the `Manager` trait.

For a complete example, see `examples/custom_manager.rs` in the repository.

## Examples

The Plux repository includes several examples to help you get started:

### Basic Examples

- [Basic Plugin](./examples/basic_plugin.rs) - A minimal "Hello World" plugin implementation
- [Parallel Plugins](./examples/parallel_plugins.rs) - Shows how to load and manage multiple plugins concurrently
- [Plugin Dependencies](./examples/plugin_dependencies.rs) - Implements plugins with inter-dependencies

### Advanced Examples

- [Custom Manager](./examples/custom_manager.rs) - Demonstrates creating a custom plugin manager
- [Hot Reloading](./examples/hot_reload.rs) - Demonstrates hot-reloading plugins at runtime
- [Performance Benchmark](./examples/benchmark.rs) - Measures plugin loading and execution performance

### Integration Examples

- [Web Server Plugin](./examples/web_server_plugin.rs) - Creates a plugin that extends a web server
- [CLI Application](./examples/cli_application.rs) - Builds a command-line tool with plugin support
- [GUI Application](./examples/gui_application.rs) - Demonstrates plugin-based UI extensions

Each example includes detailed comments and can be run using Cargo:

```bash
cargo run --example basic_plugin
```

## Features

Plux provides several feature flags to customize functionality:

### Core Features

- `full` - Enables all features (recommended for most use cases)
- `default` - Includes essential features for basic plugin functionality

### Plugin Development

- `derive` - Enables derive macros for implementing plugin traits
  - `#[plux::function]` - Expose Rust functions to plugins

### Plugin Packaging

- `archive` - Adds support for packaging plugins as zip archives
  - `plux::utils::archive::zip` - Bundle plugin files into an archive
  - `plux::utils::archive::unzip` - Extract plugin files from an archive

### Serialization (enabled by default)

> [!WARNING]
> There is currently none. It will be implemented in 2.0.

- `serde` - Enables serialization/deserialization of plugin data
  - Automatic derive support for `Serialize` and `Deserialize`
  - Integration with common formats (JSON, MessagePack, etc.)

### Concurrency (enabled by default)

> [!WARNING]
> There is currently none. It will be implemented in 2.0.

- `async` - Enables async/await support for plugin operations
  - Asynchronous plugin loading and execution
  - Non-blocking I/O operations

### Logging (enabled by default)

> [!WARNING]
> There is currently none. It will be implemented in 2.0.

- `log` - Integrates with the `log` crate for plugin logging
  - Structured logging support
  - Plugin-specific log filtering

## Available Plugin Managers

Plux supports various plugin types through specialized managers:

- [plux-lua-manager](https://github.com/BleynChannel/plux-lua-manager) - Execute Lua scripts as plugins
- [plux-native-manager](https://github.com/BleynChannel/plux-native-manager) - Load and execute native Rust plugins
- [plux-wasm-manager](https://github.com/BleynChannel/plux-wasm-manager) - Run WebAssembly modules as plugins with sandboxed execution
