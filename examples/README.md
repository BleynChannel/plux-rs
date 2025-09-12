# Plux Examples

This directory contains various examples demonstrating how to use the Plux plugin system in different scenarios. Each example showcases specific features and capabilities of the framework.

## Overview

Plux is a comprehensive plugin system for Rust applications that allows you to extend your application's functionality through dynamically loaded plugins. These examples will help you understand how to integrate Plux into your projects.

## Examples

### 1. Basic Plugin (`basic_plugin.rs`)

A simple example showing how to:
- Create a plugin loader
- Register functions that plugins can call
- Load and execute a basic plugin
- Properly unload plugins

The example uses a "Hello World" style plugin that calls a host function to greet a user.

### 2. Hot Reload (`hot_reload.rs`)

Demonstrates the hot reloading capability of Plux:
- Load a plugin
- Detect changes to the plugin file
- Reload the plugin without restarting the host application
- See changes take effect immediately

This is useful for development workflows where you want to see changes without restarting your application.

### 3. Parallel Plugins (`parallel_plugins.rs`)

Shows how to:
- Load multiple plugins simultaneously
- Execute plugins concurrently
- Compare sequential vs. parallel execution performance

This example highlights Plux's ability to handle concurrent plugin execution.

### 4. Plugin Dependencies (`plugin_dependencies.rs`)

Illustrates how Plux handles plugin dependencies:
- Define dependencies between plugins
- Automatic dependency resolution
- Proper loading order based on dependencies

### 5. CLI Application (`cli_application.rs`)

A more complex example showing how to build a CLI application with Plux:
- Plugin-based command handling
- Interactive user input
- Data access through plugins
- Multiple plugins working together

### 6. GUI Application (`gui_application.rs`)

Demonstrates using Plux in a graphical application:
- Integration with egui framework
- Plugin-based UI rendering
- User interaction through plugins

### 7. Web Server Plugin (`web_server_plugin.rs`)

Shows how to use Plux in a web server context:
- Plugin-based request handling
- Template rendering with plugins
- Data access for web responses

### 8. Custom Manager (`custom_manager.rs`)

Explains how to create custom plugin managers:
- Implement the Manager trait
- Support custom plugin formats
- Handle plugin registration and execution

This is useful when you want to support plugins in formats other than the default Lua support.

### 9. Benchmark (`benchmark.rs`)

A performance testing example:
- Load multiple plugins
- Measure execution time
- Compare performance characteristics

## Plugin Structure

Each plugin is contained in its own directory with the naming convention `{name}-v{version}.lua`. Inside each plugin directory, you'll find:

- `config.toml`: Plugin metadata including name, description, author, and dependencies
- `main.lua`: The plugin's main code implementing the required functions

## Running Examples

To run any example, use:

```bash
cargo run --example {example_name}
```

For example, to run the basic plugin example:

```bash
cargo run --example basic_plugin
```

## Plugin-Specific Examples

In addition to the Rust examples, the `plugins` directory contains various Lua plugins that demonstrate different aspects of the plugin system:

- `benchmark/`: Plugins used for performance testing
- `cli/`: CLI command plugins
- `dependency/`: Plugins with various dependency relationships
- `gui/`: GUI rendering plugins
- `web/`: Web request handling plugins

Each plugin implements specific functions that are called by their respective host applications.