mod error;
mod ptr;

pub use error::*;
pub use ptr::*;

/// Archive utilities for plugin packaging.
///
/// This module provides functions for compressing and extracting plugin bundles
/// using ZIP archives. It requires the `archive` feature to be enabled.
#[cfg(feature = "archive")]
pub mod archive;
