use std::fmt::{Debug, Display};

use crate::variable::Variable;

use super::Arg;

/// Trait for defining callable functions in the plugin system.
///
/// The Function trait represents a callable entity that can be invoked with arguments
/// and returns a result. Functions can be registered with the loader and called by plugins.
///
/// # Type Parameters
///
/// * `Output` - The return type of the function (must implement Send + Sync)
///
/// # Required Methods
///
/// * `name` - Returns the function name as a string
/// * `inputs` - Returns the list of input arguments
/// * `output` - Returns the output argument (if any)
/// * `call` - Executes the function with the given arguments
///
/// # Example
///
/// ```rust
/// use plux_rs::function::{Function, Arg, DynamicFunction, FunctionOutput};
/// use plux_rs::variable::{Variable, VariableType};
///
/// // Create a simple add function
/// let add_func = DynamicFunction::new(
///     "add",
///     vec![
///         Arg::new("a", VariableType::I32),
///         Arg::new("b", VariableType::I32),
///     ],
///     Some(Arg::new("result", VariableType::I32)),
///     |args| -> FunctionOutput {
///         let a: i32 = args[0].parse_ref();
///         let b: i32 = args[1].parse_ref();
///         Ok(Some((a + b).into()))
///     }
/// );
///
/// // Call the function
/// let result = add_func.call(&[1.into(), 2.into()]);
/// assert_eq!(result.unwrap(), Some(3.into()));
/// ```
pub trait Function: Send + Sync {
    /// The output type of the function
    type Output: Send + Sync;

    /// Returns the name of the function.
    ///
    /// # Returns
    ///
    /// Returns the function name as a String.
    fn name(&self) -> String;

    /// Returns the input arguments of the function.
    ///
    /// # Returns
    ///
    /// Returns a vector of Arg describing the function's input parameters.
    fn inputs(&self) -> Vec<Arg>;

    /// Returns the output argument of the function.
    ///
    /// # Returns
    ///
    /// Returns `Some(Arg)` if the function has an output, `None` for void functions.
    fn output(&self) -> Option<Arg>;

    /// Calls the function with the given arguments.
    ///
    /// # Parameters
    ///
    /// * `args` - Slice of Variable arguments to pass to the function
    ///
    /// # Returns
    ///
    /// Returns the function's output of type `Self::Output`.
    fn call(&self, args: &[Variable]) -> Self::Output;
}

impl<O: Send + Sync> PartialEq for dyn Function<Output = O> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
            && self.inputs() == other.inputs()
            && self.output() == other.output()
    }
}

impl<O: Send + Sync> Eq for dyn Function<Output = O> {}

impl<O: Send + Sync> Display for dyn Function<Output = O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO: Implement function descriptions in August
        // // Comment as function description
        // write!(f, "# {}\n", self.description);

        // Function
        write!(
            f,
            "{}({}) -> {}",
            self.name(),
            self.inputs()
                .iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<_>>()
                .join(", "),
            match self.output() {
                Some(arg) => format!("{}({})", arg.name, arg.ty),
                None => "void".to_string(),
            }
        )
    }
}

impl<O: Send + Sync> Debug for dyn Function<Output = O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

/// Standard output type for dynamic functions.
///
/// This type alias represents the result of calling a dynamic function.
/// It can either succeed with an optional Variable result or fail with an error.
pub type FunctionOutput = Result<Option<Variable>, Box<dyn std::error::Error + Send + Sync>>;

/// A dynamic function that can be called at runtime.
///
/// DynamicFunction provides a concrete implementation of the Function trait that
/// wraps a closure. This allows for runtime creation of functions with dynamic behavior.
///
/// # Fields
///
/// * `name` - The function name
/// * `inputs` - List of input arguments
/// * `output` - Optional output argument
/// * `ptr` - The function implementation as a boxed closure
///
/// # Example
///
/// ```rust
/// use plux_rs::function::{DynamicFunction, Arg, FunctionOutput};
/// use plux_rs::variable::VariableType;
///
/// let multiply = DynamicFunction::new(
///     "multiply",
///     vec![
///         Arg::new("x", VariableType::F64),
///         Arg::new("y", VariableType::F64),
///     ],
///     Some(Arg::new("result", VariableType::F64)),
///     |args| -> FunctionOutput {
///         let x: f64 = args[0].parse_ref();
///         let y: f64 = args[1].parse_ref();
///         Ok(Some((x * y).into()))
///     }
/// );
/// ```
pub struct DynamicFunction {
    name: String,
    inputs: Vec<Arg>,
    output: Option<Arg>,
    ptr: Box<dyn Fn(&[Variable]) -> FunctionOutput + Send + Sync>,
}

impl DynamicFunction {
    /// Creates a new dynamic function.
    ///
    /// # Parameters
    ///
    /// * `name` - The function name (will be converted to String)
    /// * `inputs` - Vector of input arguments
    /// * `output` - Optional output argument (None for void functions)
    /// * `ptr` - The function implementation as a closure
    ///
    /// # Returns
    ///
    /// Returns a new DynamicFunction instance.
    ///
    /// # Type Parameters
    ///
    /// * `S` - Type that can be converted into String (for the name)
    /// * `F` - Function type that takes &[Variable] and returns FunctionOutput
    ///
    /// # Example
    ///
    /// ```rust
    /// use plux_rs::function::{DynamicFunction, Arg, FunctionOutput};
    /// use plux_rs::variable::VariableType;
    ///
    /// let greet = DynamicFunction::new(
    ///     "greet",
    ///     vec![Arg::new("name", VariableType::String)],
    ///     Some(Arg::new("message", VariableType::String)),
    ///     |args| -> FunctionOutput {
    ///         let name: String = args[0].parse_ref();
    ///         Ok(Some(format!("Hello, {}!", name).into()))
    ///     }
    /// );
    /// ```
    pub fn new<S, F>(name: S, inputs: Vec<Arg>, output: Option<Arg>, ptr: F) -> Self
    where
        S: Into<String>,
        F: Fn(&[Variable]) -> FunctionOutput + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            inputs,
            output,
            ptr: Box::new(ptr),
        }
    }
}

impl Function for DynamicFunction {
    type Output = FunctionOutput;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn inputs(&self) -> Vec<Arg> {
        self.inputs.clone()
    }

    fn output(&self) -> Option<Arg> {
        self.output.clone()
    }

    fn call(&self, args: &[Variable]) -> Self::Output {
        (self.ptr)(args)
    }
}

#[test]
fn function_call() {
    use crate::variable::VariableType;

    // Creating a function
    let func = DynamicFunction::new(
        "add",
        vec![
            Arg::new("a", VariableType::I32),
            Arg::new("b", VariableType::I32),
        ],
        Some(Arg::new("c", VariableType::I32)),
        |args| -> FunctionOutput {
            let a = args[0].parse_ref::<i32>();
            let b = args[1].parse_ref::<i32>();

            let c = a + b;

            println!("{} + {} = {}", a, b, c);

            Ok(Some(c.into()))
        },
    );

    // Running the function
    let c = func.call(&[1.into(), 2.into()]);

    assert!(c.is_ok());
    assert_eq!(c.unwrap(), Some(3.into()));
}

#[test]
fn parallel_call() {
    use crate::variable::VariableType;
    use std::{sync::Arc, thread, time::Duration};

    // Creating a function
    let func = DynamicFunction::new(
        "log",
        vec![Arg::new("n", VariableType::I32)],
        None,
        |args| -> FunctionOutput {
            let n = args[0].parse_ref::<i32>();

            println!("Step {n}");

            Ok(None)
        },
    );

    // Calling the function in multiple threads
    let func = Arc::new(func);

    let mut handles = vec![];
    for i in 0..10 {
        let func = func.clone();
        let args: Arc<[Variable]> = Arc::new([i.into()]);

        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));

            let result = func.call(&args.as_ref());

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), None);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
