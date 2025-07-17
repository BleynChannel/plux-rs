use std::fmt::{Debug, Display};

use crate::variable::Variable;

use super::Arg;

pub trait Function: Send + Sync {
    type Output: Send + Sync;

    fn name(&self) -> String;
    fn inputs(&self) -> Vec<Arg>;
    fn output(&self) -> Option<Arg>;
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
        //TODO: Внедрить описание функций в August
        // // Комментарий в виде описания функции
        // write!(f, "# {}\n", self.description);

        // Функция
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

pub type FunctionOutput = Result<Option<Variable>, Box<dyn std::error::Error + Send + Sync>>;

pub struct DynamicFunction {
    name: String,
    inputs: Vec<Arg>,
    output: Option<Arg>,
    ptr: Box<dyn Fn(&[Variable]) -> FunctionOutput + Send + Sync>,
}

impl DynamicFunction {
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
    let c = func.call(&[1.into(), 2.into()]);

    assert!(c.is_ok());
    assert_eq!(c.unwrap(), Some(3.into()));
}

#[test]
fn parallel_call() {
    use crate::variable::VariableType;
    use std::{sync::Arc, thread, time::Duration};

    // Создание функции
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

    // Вызов функции в нескольких потоках
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
