#[cfg(test)]
mod main {
    use plux::{function::Function, function_call, variable::Variable};

    extern crate plux;

    mod functions {
        use plux::variable::Variable;
        use plux_codegen::function;

        #[function]
        fn add(_: (), a: Vec<&i32>, b: &String) -> Vec<i32> {
            let mut c = [0; 1];
            c[0] = a[0] + b.parse::<i32>().unwrap();
            println!("{} + {} = {}", a[0], b, c[0]);
            c.to_vec()
        }

        #[function(name = "Sub function")]
        fn sub(_: (), a: &i32, b: &i32) -> i32 {
            let c = a - b;
            println!("{} - {} = {}", a, b, c);
            c
        }

        #[function]
        fn contain_a(_: (), strs: Vec<&String>) -> Variable {
            for s in strs {
                if !s.contains("a") {
                    return Variable::Null;
                }
            }
            true.into()
        }

        #[function(name = "Logging")]
        fn log((title, code): (&Option<String>, &i32), message: &String) {
            let title = title.clone().unwrap_or("[INFO]".to_string());
            println!("{title} #{code}: {message}");
        }
    }

    #[test]
    fn serialize_add() {
        let add = functions::add();
        println!("`add` name: {}", add.name(),);

        let result = function_call!(add, vec![1], "2");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Variable::List(vec![3.into()])));
    }

    #[test]
    fn serialize_sub() {
        let sub = functions::sub();
        println!("`sub` name: {}", sub.name(),);

        let result = function_call!(sub, 3, 2);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(1.into()));
    }

    #[test]
    fn serialize_contain_a() {
        let contain_a = functions::contain_a();
        println!("`contain_a` name: {}", contain_a.name());

        let mut result = function_call!(contain_a, vec!["apple", "banana"]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(true.into()));

        result = function_call!(contain_a, vec!["moon", "sun"]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Variable::Null));
    }

    #[test]
    fn serialize_log() {
        let log = functions::log(Some("[ERROR]".to_string()), 264);
        println!("`log` name: {}", log.name(),);

        let mut result = function_call!(log, "It's error");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);

        result = function_call!(log, "It's also error");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }
}
