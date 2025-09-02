pub mod context;
pub mod utils;

mod api;
mod bundle;
mod info;
mod loader;
mod manager;
mod plugin;

pub mod function;
pub mod variable;

use std::sync::Arc;

pub use api::*;
pub use bundle::*;
pub use context::*;
pub use info::*;
pub use loader::*;
pub use manager::*;
pub use plugin::*;

use function::{Function, Request};

pub type Registry<O> = Vec<Arc<dyn Function<Output = O>>>;
pub type Requests = Vec<Request>;

#[cfg(feature = "derive")]
pub use plux_codegen::*;


/// Re-export of common types for manager development.
pub mod prelude {
}