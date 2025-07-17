#[macro_export]
macro_rules! function_call {
	($function: ident, $($args:expr), +) => {
		$function.call(&[$($args.into()), +])
	};
	($function: ident) => {
		$function.call(&[])
	};
}

#[test]
fn run() {
    use crate::{
        function::{Arg, DynamicFunction, Function, FunctionOutput},
        variable::VariableType,
    };

    // Создание функции
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

    // Запуск функции
    let c = function_call!(func, 1, 2);

    assert!(c.is_ok());
    assert_eq!(c.unwrap(), Some(3.into()));
}
