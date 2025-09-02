# Plux (formerly August Plugin System)

[![Crates.io](https://img.shields.io/crates/v/plux)](https://crates.io/crates/plux)
[![Documentation](https://docs.rs/plux/badge.svg)](https://docs.rs/plux)
[![License](https://img.shields.io/crates/l/plux)](LICENSE)
[![Rust](https://github.com/your-org/plux-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/your-org/plux-rs/actions)

Plux is a powerful and extensible plugin system for Rust applications, designed to provide a flexible and type-safe way to extend application functionality through plugins. It supports dynamic loading, hot-reloading, and cross-language plugin development.

## Features

- **Language Agnostic**: Write plugins in any language that can expose a C-compatible FFI
- **Type Safety**: Rust's type system ensures safe plugin interactions
- **Hot Reloading**: Update plugins without restarting the host application
- **Sandboxing**: Run plugins in isolated environments for security
- **Async Support**: First-class support for async/await
- **Cross-Platform**: Works on all major platforms (Windows, macOS, Linux)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
plux = "1.0.0-alpha.3"
```

## Quick Start

### Creating a Simple Plugin

1. Create a new Rust library:
```bash
cargo new --lib my_plugin
cd my_plugin
```

2. Add to `Cargo.toml`:
```toml
[lib]
crate-type = ["cdylib"]

dependencies.rustversion = "1.75"

[dependencies]
plux = { version = "1.0.0-alpha.3", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
```

3. Implement your plugin in `src/lib.rs`:
```rust
use plux::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GreeterConfig {
    pub name: String,
}

#[plux_plugin]
impl PluxPlugin for Greeter {
    type Config = GreeterConfig;
    
    fn new(config: Self::Config) -> Self {
        Greeter { config }
    }
    
    fn on_load(&self) {
        println!("Plugin loaded: Greeter");
    }
    
    fn on_unload(&self) {
        println!("Plugin unloaded: Greeter");
    }
}
```

### Loading a Plugin in Your Application

```rust
use plux::prelude::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = PluginManager::new();
    
    // Load a plugin
    let plugin = manager.load_plugin(
        Path::new("target/debug/libmy_plugin.so"),
        r#"{ "name": "World" }"#
    ).await?;
    
    // Use the plugin
    // ...
    
    // Unload when done
    manager.unload_plugin(plugin.id()).await?;
    
    Ok(())
}
```

## Examples

Check out the [examples](./examples) directory for more comprehensive examples, including:

- [Basic Plugin](./examples/basic_plugin.rs) - A simple "Hello World" plugin
- [Async Plugin](./examples/async_plugin.rs) - Demonstrates async plugin functionality
- [Cross-Language Plugin](./examples/ffi_plugin.rs) - Shows how to create plugins in other languages
- [Plugin Manager](./examples/plugin_manager.rs) - Advanced plugin management and lifecycle

## Features

- `derive` - Enables derive macros for easier plugin implementation
- `archive` - Support for loading plugins from zip archives

## Safety

This crate uses `unsafe` for FFI operations. All public APIs are designed to be safe when used as documented.

## License

Licensed under either of:

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.