use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::variable::VariableType;

/// Represents a function argument with name and type.
///
/// An Arg describes a single parameter or return value of a function,
/// including both its name and data type.
///
/// # Fields
///
/// * `name` - The argument name (used for documentation and debugging)
/// * `ty` - The data type of the argument
///
/// # Examples
///
/// ```rust
/// use plux_rs::function::Arg;
/// use plux_rs::variable::VariableType;
///
/// // Create input arguments
/// let x_arg = Arg::new("x", VariableType::F64);
/// let y_arg = Arg::new("y", VariableType::F64);
///
/// // Create output argument
/// let result_arg = Arg::new("result", VariableType::F64);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct Arg {
    /// The name of the argument
    pub name: String,
    /// The data type of the argument
    pub ty: VariableType,
}

impl Arg {
    /// Creates a new argument with the given name and type.
    ///
    /// # Parameters
    ///
    /// * `name` - The argument name (will be converted to String)
    /// * `ty` - The data type of the argument
    ///
    /// # Returns
    ///
    /// Returns a new Arg instance.
    ///
    /// # Type Parameters
    ///
    /// * `S` - Type that can be converted into String
    ///
    /// # Example
    ///
    /// ```rust
    /// use plux_rs::function::Arg;
    /// use plux_rs::variable::VariableType;
    ///
    /// let arg = Arg::new("count", VariableType::I32);
    /// assert_eq!(arg.name, "count");
    /// assert_eq!(arg.ty, VariableType::I32);
    /// ```
    pub fn new<S: Into<String>>(name: S, ty: VariableType) -> Self {
        Self {
            name: name.into(),
            ty,
        }
    }
}

impl Default for Arg {
    fn default() -> Self {
        Self {
            name: "arg".to_string(),
            ty: Default::default(),
        }
    }
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}
