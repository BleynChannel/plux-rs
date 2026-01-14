pub mod utils {
    use std::{path::PathBuf, sync::Arc};

    use plux_rs::prelude::{Function, FunctionOutput};

    pub type FunctionPointer = Arc<dyn Function<Output = FunctionOutput>>;

    pub fn get_plugin_path(filename: impl AsRef<str>) -> PathBuf {
        std::env::current_dir()
            .unwrap()
            .join(format!("./examples/plugins/{}", filename.as_ref()))
    }
}

pub mod hello_v1 {
    use std::{collections::HashMap, sync::LazyLock};

    use plux_mock::MockPlugin;
    use plux_rs::{
        Api, Bundle, StdInfo,
        function::{Function, FunctionOutput},
        function_call,
    };

    #[allow(dead_code)]
    pub const FILENAME: &str = "hello-v1.0.0.mock";
    #[allow(dead_code)]
    pub const BUNDLE: LazyLock<Bundle> = LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
    #[allow(dead_code)]
    pub const INFO: StdInfo = StdInfo::new();

    #[allow(dead_code)]
    pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
        map.insert(
            BUNDLE.clone(),
            MockPlugin::new(INFO.clone(), |mut context, api| {
                let requests = context.requests();

                let main = main(api);
                if requests.iter().any(|r| r.name == main.name()) {
                    context.register_request(main)?;
                }

                Ok(())
            }),
        );
    }

    #[plux_rs::function]
    fn main(api: &Api<FunctionOutput, StdInfo>) {
        let greet = api.registry().iter().find(|f| f.name() == "greet").unwrap();
        let _ = function_call!(greet, 30).unwrap();
    }
}

pub mod benchmark {
    pub mod plugin1_v1 {
        use std::{collections::HashMap, sync::LazyLock};

        use plux_mock::MockPlugin;
        use plux_rs::{Api, Bundle, StdInfo, function::FunctionOutput, function_call};

        #[allow(dead_code)]
        pub const FILENAME: &str = "plugin1-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, api| {
                    let requests = context.requests();

                    // If this plugin needs to implement the 'compute' request, register it
                    if requests.iter().any(|r| r.name == "compute") {
                        context.register_request(compute(api))?;
                    }
                    Ok(())
                }),
            );
        }

        // Define an actual function implementation for the 'compute' request
        #[plux_rs::function]
        fn compute(api: &Api<FunctionOutput, StdInfo>, a: &i32, b: &i32) -> i32 {
            let calculate = api
                .registry()
                .iter()
                .find(|f| f.name() == "calculate")
                .unwrap();
            let result = function_call!(calculate, *a, *b).unwrap().unwrap();

            result.parse::<i32>()
        }
    }

    pub mod plugin2_v1 {
        use std::{collections::HashMap, sync::LazyLock};

        use plux_mock::MockPlugin;
        use plux_rs::{Api, Bundle, StdInfo, function::FunctionOutput, function_call};

        #[allow(dead_code)]
        pub const FILENAME: &str = "plugin2-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, api| {
                    let requests = context.requests();

                    // If this plugin needs to implement the 'compute' request, register it
                    if requests.iter().any(|r| r.name == "compute") {
                        context.register_request(compute(api))?;
                    }
                    Ok(())
                }),
            );
        }

        // Define an actual function implementation for the 'compute' request
        #[plux_rs::function]
        fn compute(api: &Api<FunctionOutput, StdInfo>, a: &i32, b: &i32) -> i32 {
            let calculate = api
                .registry()
                .iter()
                .find(|f| f.name() == "calculate")
                .unwrap();
            let result = function_call!(calculate, *a, *b).unwrap().unwrap();

            result.parse::<i32>() * 2
        }
    }

    pub mod plugin3_v1 {
        use std::{collections::HashMap, sync::LazyLock};

        use plux_mock::MockPlugin;
        use plux_rs::{Api, Bundle, StdInfo, function::FunctionOutput, function_call};

        #[allow(dead_code)]
        pub const FILENAME: &str = "plugin3-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, api| {
                    let requests = context.requests();

                    // If this plugin needs to implement the 'compute' request, register it
                    if requests.iter().any(|r| r.name == "compute") {
                        context.register_request(compute(api))?;
                    }
                    Ok(())
                }),
            );
        }

        // Define an actual function implementation for the 'compute' request
        #[plux_rs::function]
        fn compute(api: &Api<FunctionOutput, StdInfo>, a: &i32, b: &i32) -> i32 {
            let calculate = api
                .registry()
                .iter()
                .find(|f| f.name() == "calculate")
                .unwrap();
            let result = function_call!(calculate, *a, *b).unwrap().unwrap();

            result.parse::<i32>() * 3
        }
    }
}

pub mod cli {
    pub mod help_v1 {
        use std::{collections::HashMap, sync::LazyLock};

        use plux_mock::MockPlugin;
        use plux_rs::{Bundle, StdInfo, function::FunctionOutput, prelude::Variable};

        #[allow(dead_code)]
        pub const FILENAME: &str = "help-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(StdInfo::new(), |mut context, _api| {
                    let requests = context.requests();

                    // If this plugin needs to implement the 'get_commands' request, register it
                    if requests.iter().any(|r| r.name == "get_commands") {
                        context.register_request(get_commands())?;
                    }
                    // If this plugin needs to implement the 'handle_command' request, register it
                    if requests.iter().any(|r| r.name == "handle_command") {
                        context.register_request(handle_command())?;
                    }
                    Ok(())
                }),
            );
        }

        // Define an actual function implementation for the 'get_commands' request
        #[plux_rs::function]
        fn get_commands(_: ()) -> Vec<String> {
            vec!["help".to_string()]
        }

        // Define an actual function implementation for the 'handle_command' request
        #[plux_rs::function]
        fn handle_command(_: (), command: &String, _args: &Vec<Variable>) -> i32 {
            if command == "help" {
                println!("Available commands:");
                println!("  help          - Show this help message");
                println!("  user-info     - Display user information");
                println!("  exit          - Exit the application");
                println!("");
                println!("For more information on a specific command, try:");
                println!("  help <command>");
                0 // Success exit code
            } else {
                // Command not handled by this plugin
                1 // Indicate not handled
            }
        }
    }

    pub mod user_info_v1 {
        use std::{collections::HashMap, sync::LazyLock};

        use plux_mock::MockPlugin;
        use plux_rs::{
            Api, Bundle, StdInfo, function::FunctionOutput, function_call, prelude::Variable,
        };

        #[allow(dead_code)]
        pub const FILENAME: &str = "user_info-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(StdInfo::new(), |mut context, api| {
                    let requests = context.requests();

                    // If this plugin needs to implement the 'get_commands' request, register it
                    if requests.iter().any(|r| r.name == "get_commands") {
                        context.register_request(get_commands())?;
                    }
                    // If this plugin needs to implement the 'handle_command' request, register it
                    if requests.iter().any(|r| r.name == "handle_command") {
                        context.register_request(handle_command(api))?;
                    }
                    Ok(())
                }),
            );
        }

        // Define an actual function implementation for the 'get_commands' request
        #[plux_rs::function]
        fn get_commands(_: ()) -> Vec<String> {
            vec!["user-info".to_string()]
        }

        // Define an actual function implementation for the 'handle_command' request
        #[plux_rs::function]
        fn handle_command(
            api: &Api<FunctionOutput, StdInfo>,
            command: &String,
            args: &Vec<Variable>,
        ) -> i32 {
            let get_user_info = api
                .registry()
                .iter()
                .find(|f| f.name() == "get_user_info")
                .unwrap();

            if command == "user-info" {
                if args.is_empty() {
                    println!("Usage: user-info <username>");
                    return 1; // Error exit code
                }

                let username = args[0].clone().parse::<String>();
                let user_data = function_call!(get_user_info, username.clone())
                    .unwrap()
                    .unwrap()
                    .parse::<Vec<Variable>>();

                if user_data[0].parse_ref::<String>() == "Unknown" {
                    println!("User '{username}' not found");
                    return -1;
                } else {
                    println!("User Information:");
                    println!("  Username: {username}");
                    println!("  Name: {}", user_data[0].parse_ref::<String>());
                    println!("  Age: {}", user_data[1].parse_ref::<i32>());
                    return 0; // Success exit code
                }
            } else {
                // Command not handled by this plugin
                return 1; // Indicate not handled
            }
        }
    }
}

pub mod gui_application {}

pub mod hot_reload_v1 {
    use std::{collections::HashMap, sync::LazyLock};

    use plux_mock::MockPlugin;
    use plux_rs::{Bundle, StdInfo, function::FunctionOutput};

    #[allow(dead_code)]
    pub const FILENAME: &str = "hot_reload-v1.0.0.mock";
    #[allow(dead_code)]
    pub const BUNDLE: LazyLock<Bundle> = LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
    #[allow(dead_code)]
    pub const INFO: StdInfo = StdInfo::new();

    #[allow(dead_code)]
    pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
        map.insert(
            BUNDLE.clone(),
            MockPlugin::new(INFO.clone(), |mut context, _api| {
                let requests = context.requests();

                // If this plugin needs to implement the 'status' request, register it
                if requests.iter().any(|r| r.name == "status") {
                    context.register_request(status())?;
                }
                Ok(())
            }),
        );
    }

    // Define an actual function implementation for the 'status' request
    #[plux_rs::function]
    fn status(_: ()) -> i32 {
        // Simple fibonacci calculation for demonstration
        const STEP: i32 = 5; // Fixed step for mock example
        if STEP <= 1 {
            STEP
        } else {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=STEP {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

pub mod parallel_one_v1 {
    use std::{collections::HashMap, sync::LazyLock};

    use plux_mock::MockPlugin;
    use plux_rs::{Bundle, StdInfo, function::FunctionOutput};

    #[allow(dead_code)]
    pub const FILENAME: &str = "parallel_one-v1.0.0.mock";
    #[allow(dead_code)]
    pub const BUNDLE: LazyLock<Bundle> = LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
    #[allow(dead_code)]
    pub const INFO: StdInfo = StdInfo::new();

    #[allow(dead_code)]
    pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
        map.insert(
            BUNDLE.clone(),
            MockPlugin::new(INFO.clone(), |mut context, _api| {
                let requests = context.requests();

                // If this plugin needs to implement the 'process' request, register it
                if requests.iter().any(|r| r.name == "process") {
                    context.register_request(process())?;
                }
                Ok(())
            }),
        );
    }

    // Define an actual function implementation for the 'process' request
    #[plux_rs::function]
    fn process(_: (), value: &i32) -> i32 {
        // Simulate some processing time
        std::thread::sleep(std::time::Duration::from_millis(100));
        value * 2 // Process the value
    }
}

pub mod parallel_two_v1 {
    use std::{collections::HashMap, sync::LazyLock};

    use plux_mock::MockPlugin;
    use plux_rs::{Bundle, StdInfo, function::FunctionOutput};

    #[allow(dead_code)]
    pub const FILENAME: &str = "parallel_two-v1.0.0.mock";
    #[allow(dead_code)]
    pub const BUNDLE: LazyLock<Bundle> = LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
    #[allow(dead_code)]
    pub const INFO: StdInfo = StdInfo::new();

    #[allow(dead_code)]
    pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
        map.insert(
            BUNDLE.clone(),
            MockPlugin::new(INFO.clone(), |mut context, _api| {
                let requests = context.requests();

                // If this plugin needs to implement the 'process' request, register it
                if requests.iter().any(|r| r.name == "process") {
                    context.register_request(process())?;
                }
                Ok(())
            }),
        );
    }

    // Define an actual function implementation for the 'process' request
    #[plux_rs::function]
    fn process(_: (), value: &i32) -> i32 {
        // Simulate some processing time
        std::thread::sleep(std::time::Duration::from_millis(100));
        value * 2 + 1 // Process the value
    }
}

pub mod dependency {
    pub mod plugin_a_v1 {
        use std::{collections::HashMap, sync::LazyLock};

        use plux_mock::MockPlugin;
        use plux_rs::{Api, Bundle, Depend, StdInfo, function::FunctionOutput, function_call};
        use semver::VersionReq;

        #[allow(dead_code)]
        pub const FILENAME: &str = "plugin_a-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: LazyLock<StdInfo> = LazyLock::new(|| {
            let mut info = StdInfo::new();
            info.depends = vec![Depend::new(
                "plugin_d".to_string(),
                VersionReq::parse("1.0.0").unwrap(),
            )];
            info.optional_depends = vec![Depend::new(
                "plugin_b".to_string(),
                VersionReq::parse("1.0.0").unwrap(),
            )];
            info
        });

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, api| {
                    let requests = context.requests();

                    // If this plugin needs to implement the 'process' request, register it
                    if requests.iter().any(|r| r.name == "main") {
                        context.register_request(main(api))?;
                    }
                    Ok(())
                }),
            );
        }

        // Define an actual function implementation for the 'process' request
        #[plux_rs::function]
        fn main(api: &Api<FunctionOutput, StdInfo>) {
            let log_message = api
                .registry()
                .iter()
                .find(|f| f.name() == "log_message")
                .unwrap();

            function_call!(log_message, 1).unwrap(); // Plugin #1
        }
    }

    pub mod plugin_b_v1 {
        use std::{collections::HashMap, sync::LazyLock};

        use plux_mock::MockPlugin;
        use plux_rs::{Api, Bundle, StdInfo, function::FunctionOutput, function_call};

        #[allow(dead_code)]
        pub const FILENAME: &str = "plugin_b-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: LazyLock<StdInfo> = LazyLock::new(|| StdInfo::new());

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, api| {
                    let requests = context.requests();

                    // If this plugin needs to implement the 'process' request, register it
                    if requests.iter().any(|r| r.name == "main") {
                        context.register_request(main(api))?;
                    }
                    Ok(())
                }),
            );
        }

        // Define an actual function implementation for the 'process' request
        #[plux_rs::function]
        fn main(api: &Api<FunctionOutput, StdInfo>) {
            let log_message = api
                .registry()
                .iter()
                .find(|f| f.name() == "log_message")
                .unwrap();

            function_call!(log_message, 2).unwrap(); // Plugin #2
        }
    }

    pub mod plugin_c_v1 {
        use std::{collections::HashMap, sync::LazyLock};

        use plux_mock::MockPlugin;
        use plux_rs::{Api, Bundle, Depend, StdInfo, function::FunctionOutput, function_call};
        use semver::VersionReq;

        #[allow(dead_code)]
        pub const FILENAME: &str = "plugin_c-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        pub const INFO: LazyLock<StdInfo> = LazyLock::new(|| {
            let mut info = StdInfo::new();
            info.depends = vec![Depend::new(
                "plugin_a".to_string(),
                VersionReq::parse("1.0.0").unwrap(),
            )];
            info.optional_depends = vec![Depend::new(
                "plugin_d".to_string(),
                VersionReq::parse("1.0.0").unwrap(),
            )];
            info
        });

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, api| {
                    let requests = context.requests();

                    // If this plugin needs to implement the 'process' request, register it
                    if requests.iter().any(|r| r.name == "main") {
                        context.register_request(main(api))?;
                    }
                    Ok(())
                }),
            );
        }

        // Define an actual function implementation for the 'process' request
        #[plux_rs::function]
        fn main(api: &Api<FunctionOutput, StdInfo>) {
            let log_message = api
                .registry()
                .iter()
                .find(|f| f.name() == "log_message")
                .unwrap();

            function_call!(log_message, 3).unwrap(); // Plugin #3
        }
    }

    pub mod plugin_d_v1 {
        use std::{collections::HashMap, sync::LazyLock};

        use plux_mock::MockPlugin;
        use plux_rs::{Api, Bundle, StdInfo, function::FunctionOutput, function_call};

        #[allow(dead_code)]
        pub const FILENAME: &str = "plugin_d-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: LazyLock<StdInfo> = LazyLock::new(|| StdInfo::new());

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, api| {
                    let requests = context.requests();

                    // If this plugin needs to implement the 'process' request, register it
                    if requests.iter().any(|r| r.name == "main") {
                        context.register_request(main(api))?;
                    }
                    Ok(())
                }),
            );
        }

        // Define an actual function implementation for the 'process' request
        #[plux_rs::function]
        fn main(api: &Api<FunctionOutput, StdInfo>) {
            let log_message = api
                .registry()
                .iter()
                .find(|f| f.name() == "log_message")
                .unwrap();

            function_call!(log_message, 4).unwrap(); // Plugin #4
        }
    }
}

pub mod web_server_plugin {
    use std::{collections::HashMap, sync::LazyLock};

    use plux_mock::MockPlugin;
    use plux_rs::{Bundle, StdInfo, function::Function, function::FunctionOutput};

    pub mod api_v1 {
        use super::*;

        #[allow(dead_code)]
        pub const FILENAME: &str = "api-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, _api| {
                    let requests = context.requests();

                    let handle_request = handle_request();
                    if requests.iter().any(|r| r.name == handle_request.name()) {
                        context.register_request(handle_request)?;
                    }
                    Ok(())
                }),
            );
        }

        #[plux_rs::function]
        fn handle_request(_: ()) -> String {
            r#"{
  "api_version": "v1",
  "endpoints": [
    {"path": "/api/status", "method": "GET", "description": "Get API status"},
    {"path": "/api/users", "method": "GET", "description": "List users"},
    {"path": "/api/time", "method": "GET", "description": "Get current time"}
  ],
  "status": "active"
}"#
            .to_string()
        }
    }

    pub mod status_v1 {
        use plux_rs::{function_call, prelude::Variable};

        use super::*;

        #[allow(dead_code)]
        pub const FILENAME: &str = "status-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, api| {
                    let requests = context.requests();

                    let get_user_data = api
                        .registry()
                        .iter()
                        .find(|f| f.name() == "get_user_data")
                        .unwrap();

                    let handle_request = handle_request(get_user_data.clone());
                    if requests.iter().any(|r| r.name == handle_request.name()) {
                        context.register_request(handle_request)?;
                    }
                    Ok(())
                }),
            );
        }

        #[plux_rs::function]
        fn handle_request(get_user_data: &crate::plugins::utils::FunctionPointer) -> String {
            let user_data = match function_call!(get_user_data).unwrap().unwrap() {
                Variable::List(user_data) => user_data,
                _ => vec![],
            };

            let response = r#"{
  "server_status": "running",
  "uptime": "12345",
  "active_users": 2,
  "current_user": {"name": "{1}", "age": {2}},
  "memory_usage": "45MB",
  "version": "1.0.0"
}"#;

            response
                .replace(
                    "{1}",
                    &user_data
                        .get(0)
                        .map(|v| v.clone().parse::<String>())
                        .unwrap_or("Unknown".to_string()),
                )
                .replace(
                    "{2}",
                    &user_data
                        .get(1)
                        .map(|v| v.clone().parse::<i32>())
                        .unwrap_or(0)
                        .to_string(),
                )
        }
    }

    pub mod user_v1 {
        use plux_rs::{function_call, prelude::Variable};

        use super::*;

        #[allow(dead_code)]
        pub const FILENAME: &str = "user-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, api| {
                    let requests = context.requests();

                    let get_user_data = api
                        .registry()
                        .iter()
                        .find(|f| f.name() == "get_user_data")
                        .unwrap();

                    let handle_request = handle_request(get_user_data.clone());
                    if requests.iter().any(|r| r.name == handle_request.name()) {
                        context.register_request(handle_request)?;
                    }
                    Ok(())
                }),
            );
        }

        #[plux_rs::function]
        fn handle_request(get_user_data: &crate::plugins::utils::FunctionPointer) -> String {
            match function_call!(get_user_data, 1).unwrap().unwrap() {
                Variable::List(user_data) => {
                    let ui = r#"{
  "user": {
    "name": "{1}",
    "age": {2},
    "role": "admin"
  },
  "permissions": ["read", "write", "delete"]
}"#;
                    ui.replace("{1}", &user_data[0].clone().parse::<String>())
                        .replace("{2}", &user_data[1].clone().parse::<i32>().to_string())
                }
                _ => r#"{"error": "Invalid user data format"}"#.to_string(),
            }
        }
    }
}

pub mod gui {
    use std::{collections::HashMap, sync::LazyLock};

    use plux_mock::MockPlugin;
    use plux_rs::{
        Api, Bundle, StdInfo,
        function::{Function, FunctionOutput},
    };

    pub mod help_v1 {
        use super::*;

        #[allow(dead_code)]
        pub const FILENAME: &str = "help-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, api| {
                    let requests = context.requests();

                    let render = render(api);
                    if requests.iter().any(|r| r.name == render.name()) {
                        context.register_request(render)?;
                    }

                    Ok(())
                }),
            );
        }

        #[plux_rs::function]
        fn render(_api: &Api<FunctionOutput, StdInfo>) -> String {
            r#"{
  "help_title": {"type": "label", "text": "Help Information"},
  "help_desc": {"type": "label", "text": "This is the help plugin. It provides assistance on using the application."},
  "help_commands": {"type": "label", "text": "Available plugins: Help, User, Test"},
  "help_close": {"type": "button", "text": "Close"}
}"#.to_string()
        }
    }

    pub mod user_v1 {
        use plux_rs::{function_call, prelude::Variable};

        use super::*;

        #[allow(dead_code)]
        pub const FILENAME: &str = "user-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, api| {
                    let requests = context.requests();

                    let get_user_data = api
                        .registry()
                        .iter()
                        .find(|f| f.name() == "get_user_data")
                        .unwrap();

                    let render = render(get_user_data.clone());
                    if requests.iter().any(|r| r.name == render.name()) {
                        context.register_request(render)?;
                    }

                    Ok(())
                }),
            );
        }

        #[plux_rs::function]
        fn render(get_user_data: &crate::plugins::utils::FunctionPointer) -> String {
            match function_call!(get_user_data, 1).unwrap().unwrap() {
                Variable::List(user_data) => {
                    // Simplified user plugin
                    let ui = r#"{
              "user_title": {"type": "label", "text": "User Information"},
              "user_name": {"type": "label", "text": "Name: {1}"},
              "user_age": {"type": "label", "text": "Age: {2}"},
              "user_input": {"type": "input", "placeholder": "Enter user ID..."},
              "user_refresh": {"type": "button", "text": "Refresh"}
            }"#;
                    ui.replace("{1}", &user_data[0].clone().parse::<String>())
                        .replace("{2}", &user_data[1].clone().parse::<i32>().to_string())
                }
                _ => r#"{"error": "Invalid user data format"}"#.to_string(),
            }
        }
    }

    pub mod test_v1 {
        use super::*;

        #[allow(dead_code)]
        pub const FILENAME: &str = "test-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, api| {
                    let requests = context.requests();

                    let render = render(api);
                    if requests.iter().any(|r| r.name == render.name()) {
                        context.register_request(render)?;
                    }

                    Ok(())
                }),
            );
        }

        #[plux_rs::function]
        fn render(_api: &Api<FunctionOutput, StdInfo>) -> String {
            r#"{
  "test_title": {"type": "label", "text": "Hello world"},
  "test_desc": {"type": "label", "text": "This is a test plugin demonstrating the GUI interface"},
  "test_input": {"type": "input", "placeholder": "Enter text..."},
  "test_button": {"type": "button", "text": "Click Me"}
}"#
            .to_string()
        }
    }
}
