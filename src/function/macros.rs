/// Macro for convenient function calling with automatic argument conversion.
///
/// This macro simplifies calling functions by automatically converting arguments
/// to Variables and handling the function call syntax.
///
/// # Examples
///
/// ```rust
/// use plux_rs::function::{DynamicFunction, Arg, FunctionOutput, function_call};
/// use plux_rs::variable::VariableType;
///
/// let add = DynamicFunction::new(
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
/// // Call with arguments
/// let result = function_call!(add, 5, 3);
/// assert_eq!(result.unwrap(), Some(8.into()));
///
/// // Call without arguments
/// let no_args_func = DynamicFunction::new(
///     "hello",
///     vec![],
///     Some(Arg::new("message", VariableType::String)),
///     |_| -> FunctionOutput { Ok(Some("Hello!".into())) }
/// );
/// let message = function_call!(no_args_func);
/// ```
#[macro_export]
macro_rules! function_call {
	($function: ident, $($args:expr), +) => {
        // Call a function with multiple arguments
		$function.call(&[$($args.into()), +])
	};
	($function: ident) => {
        // Call a function with no arguments
		$function.call(&[])
	};
}

#[test]
fn run() {
    use crate::{
        function::{Arg, DynamicFunction, Function, FunctionOutput},
        variable::VariableType,
    };

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
    let c = function_call!(func, 1, 2);

    assert!(c.is_ok());
    assert_eq!(c.unwrap(), Some(3.into()));
}
