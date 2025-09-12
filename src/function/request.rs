use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::variable::VariableType;

/// Represents a function request that plugins must implement.
///
/// A Request defines the interface that plugins are expected to provide.
/// It specifies the function signature that plugins should implement,
/// including input and output types.
///
/// # Fields
///
/// * `name` - The name of the requested function
/// * `inputs` - List of input parameter types
/// * `output` - Optional output type (None for void functions)
///
/// # Examples
///
/// ```rust
/// use plux_rs::function::Request;
/// use plux_rs::variable::VariableType;
///
/// // Request a function that takes two integers and returns their sum
/// let add_request = Request::new(
///     "add",
///     vec![VariableType::I32, VariableType::I32],
///     Some(VariableType::I32)
/// );
///
/// // Request a logging function that takes a string and returns nothing
/// let log_request = Request::new(
///     "log",
///     vec![VariableType::String],
///     None
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    /// The name of the requested function
    pub name: String,
    /// List of input parameter types
    pub inputs: Vec<VariableType>,
    /// Optional output type (None for void functions)
    pub output: Option<VariableType>,
}

impl Request {
    /// Creates a new function request.
    ///
    /// # Parameters
    ///
    /// * `name` - The function name (will be converted to String)
    /// * `inputs` - Vector of input parameter types
    /// * `output` - Optional output type (None for void functions)
    ///
    /// # Returns
    ///
    /// Returns a new Request instance.
    ///
    /// # Type Parameters
    ///
    /// * `S` - Type that can be converted into String
    ///
    /// # Example
    ///
    /// ```rust
    /// use plux_rs::function::Request;
    /// use plux_rs::variable::VariableType;
    ///
    /// let request = Request::new(
    ///     "calculate_area",
    ///     vec![VariableType::F64, VariableType::F64], // width, height
    ///     Some(VariableType::F64) // area
    /// );
    /// ```
    pub fn new<S: Into<String>>(
        name: S,
        inputs: Vec<VariableType>,
        output: Option<VariableType>,
    ) -> Self {
        Self {
            name: name.into(),
            inputs,
            output,
        }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}) -> {}",
            self.name,
            self.inputs
                .iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<_>>()
                .join(", "),
            match self.output {
                Some(x) => format!("{x}"),
                None => "void".to_string(),
            }
        )
    }
}
