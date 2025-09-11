//! # Plux - Extensible Plugin System for Rust
//!
//! Plux is a comprehensive plugin system for Rust applications, offering a robust and flexible
//! architecture for extending application functionality through plugins. It enables seamless
//! integration of third-party code while maintaining security and stability.
//!
//! ## Key Features
//!
//! - **Language Agnostic**: Write plugins in any programming language
//! - **Hot Reloading**: Update plugins without restarting the host application
//! - **Dynamic Loading**: Load and unload plugins at runtime
//! - **Type Safety**: Rust's type system ensures safe plugin interactions
//! - **Cross-Platform**: Works on all major platforms (Windows, macOS, Linux)
//! - **Performance Optimized**: Efficient loading and caching of plugins
//! - **Isolated Execution**: Secure sandboxing for plugin execution
//!
//! ## Core Components
//!
//! - **Loader**: Central component for managing plugin lifecycle and execution
//! - **Manager**: Adapters providing standardized interfaces for plugin integration
//! - **Plugin**: Self-contained modules that extend application functionality
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use plux_rs::prelude::*;
//! use plux_lua_manager::LuaManager;
//!
//! #[function]
//! fn add(_: (), a: &i32, b: &i32) -> i32 {
//!     a + b
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut loader = Loader::new();
//!     
//!     loader.context(move |mut ctx| {
//!         ctx.register_manager(LuaManager::new())?;
//!         ctx.register_function(add());
//!         
//!         // Load and manage plugins here
//!         Ok::<(), Box<dyn std::error::Error>>(())
//!     })?;
//!     
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![doc(html_logo_url = "https://example.com/logo.png")]
#![doc(html_favicon_url = "https://example.com/favicon.ico")]

/// Context types used during plugin loading and registration.
///
/// This module provides the context types that are passed to plugin managers
/// during various lifecycle events such as registration and loading.
pub mod context;

/// Function and request definitions for the plugin system.
///
/// This module defines the core function and request types that enable
/// communication between plugins and the host application.
pub mod function;

/// Utility types and functions for the plugin system.
///
/// This module contains various utility types, error definitions, and helper
/// functions used throughout the plugin system.
pub mod utils;

/// Variable types used for data exchange between plugins and host.
///
/// This module defines the Variable and VariableType enums that represent
/// the data types that can be passed between plugins and the host application.
pub mod variable;

mod api;
mod bundle;
mod info;
mod loader;
mod manager;
mod plugin;

pub use api::*;
pub use bundle::*;
pub use context::*;
pub use info::*;
pub use loader::*;
pub use manager::*;
pub use plugin::*;

use function::{Function, Request};
use std::sync::Arc;

/// Registry of functions that can be called by plugins.
/// This type alias represents a collection of functions exposed by the host application
/// that plugins can invoke during execution.
pub type Registry<O> = Vec<Arc<dyn Function<Output = O>>>;

/// Collection of function requests from plugins.
/// This type alias represents a collection of requests that plugins can make to the host
/// application, typically for accessing host-provided functionality.
pub type Requests = Vec<Request>;

/// Re-exports for procedural macros when the `derive` feature is enabled.
///
/// # Macros
///
/// ## `#[function]`
///
/// A procedural macro that transforms a Rust function into a plugin-compatible function.
/// This macro enables the function to be called from plugins and handles serialization
/// of arguments and return values.
///
/// ### Usage
///
/// ```rust,no_run
/// // Basic usage with primitive types
/// #[plux_rs::function]
/// fn add(_: (), a: i32, b: i32) -> i32 {
///     a + b
/// }
///
/// // With references for better performance
/// #[plux_rs::function]
/// fn concat(_: (), a: &str, b: &str) -> String {
///     format!("{} {}", a, b)
/// }
///
/// // With context parameter (first parameter is always the context)
/// #[plux_rs::function]
/// fn greet(message: &String, name: &str) -> String {
///     format!("{} {}", message, name)
/// }
/// 
/// let mut loader = Loader::new();
/// loader.context(move |mut ctx| {
///     ctx.register_function(add());
///     ctx.register_function(concat());
///     ctx.register_function(greet("Hello world".to_string()));
///     
///     Ok::<(), Box<dyn std::error::Error>>(())
/// }).unwrap();
/// 
/// let registry = loader.get_registry();
/// 
/// let add_function = registry.get(0).unwrap();
/// let concat_function = registry.get(1).unwrap();
/// let greet_function = registry.get(2).unwrap();
/// let great_function = registry.get(3).unwrap();
/// 
/// let result = add_function.call(&[1.into(), 2.into()]).unwrap().unwrap();
/// assert_eq!(result, 3.into());
/// 
/// let result = concat_function.call(&["Hello".into(), "world".into()]).unwrap().unwrap();
/// assert_eq!(result, "Hello world".into());
/// 
/// let result = great_function.call(&[true.into()]).unwrap().unwrap();
/// assert_eq!(result, "Hello world".into());
/// ```
///
/// ### Features
///
/// - **Type Safety**: Compile-time type checking of function signatures
/// - **Zero-Copy**: Always uses references to avoid unnecessary cloning
/// - **Context Support**: First parameter can be a context object
///
/// ### Notes
///
/// - The first parameter can be used for context (use `_` if not needed)
/// - Supported parameter types: primitive types, `&T` and `Vec<&T>`
/// - The function will be available to plugins under its Rust name by default
#[cfg(feature = "derive")]
pub use plux_codegen::function;

/// Re-export of common types for plugin implementation.
/// This module provides convenient access to the most commonly used types when
/// implementing plugins.
pub mod prelude {
    pub use crate::LoaderContext;
    pub use crate::api::*;
    pub use crate::bundle::*;
    pub use crate::function::*;
    pub use crate::info::{Depend, Info, StdInfo};
    pub use crate::loader::*;
    pub use crate::plugin::*;
    pub use crate::utils::*;
    pub use crate::variable::*;

    #[cfg(feature = "derive")]
    pub use plux_codegen::*;
}
