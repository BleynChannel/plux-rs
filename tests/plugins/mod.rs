pub mod function_plugin_v1 {
    use std::{collections::HashMap, sync::LazyLock};

    use plux_mock::MockPlugin;
    use plux_rs::{
        Api, Bundle, StdInfo, function::{Function, FunctionOutput}, function_call,
    };

    #[allow(dead_code)]
    pub const FILENAME: &str = "function_plugin-v1.0.0.mock";
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

                let mul = mul();
                if requests.iter().any(|r| r.name == mul.name()) {
                    context.register_request(mul)?;
                }

                let main = main(api);
                if requests.iter().any(|r| r.name == main.name()) {
                    context.register_request(main)?;
                }

                let echo = echo();
                if requests.iter().any(|r| r.name == echo.name()) {
                    context.register_request(echo)?;
                }

                Ok(())
            }),
        );
    }

    #[plux_rs::function]
    fn mul(_: (), a: &i32, b: &i32) -> i32 {
        a * b
    }

    #[plux_rs::function]
    fn main(api: &Api<FunctionOutput, StdInfo>) {
        let add = api.registry().iter().find(|f| f.name() == "add").unwrap();
        let sub = api.registry().iter().find(|f| f.name() == "sub").unwrap();
        let mul = mul();

        println!("4 + 6 = {}", function_call!(add, 4, 6).unwrap().unwrap());
        println!("9 - 3 = {}", function_call!(sub, 9, 3).unwrap().unwrap());
        println!("8 * 3 = {}", function_call!(mul, 8, 3).unwrap().unwrap());
    }

    #[plux_rs::function]
    fn echo(_: (), message: &String) -> String {
        format!("Message v.1.0.0: {message}")
    }
}

pub mod function_plugin_v2 {
    use std::{collections::HashMap, sync::LazyLock};

    use plux_mock::MockPlugin;
    use plux_rs::{Api, Bundle, StdInfo, function::{Function, FunctionOutput}, function_call};

    #[allow(dead_code)]
    pub const FILENAME: &str = "function_plugin-v2.0.0.mock";
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
                
                let mul = mul();
                if requests.iter().any(|r| r.name == mul.name()) {
                    context.register_request(mul)?;
                }

                let main = main(api);
                if requests.iter().any(|r| r.name == main.name()) {
                    context.register_request(main)?;
                }

                let echo = echo();
                if requests.iter().any(|r| r.name == echo.name()) {
                    context.register_request(echo)?;
                }

                Ok(())
            }),
        );
    }

    #[plux_rs::function]
    fn mul(_: (), a: &i32, b: &i32) -> i32 {
        a * b
    }

    #[plux_rs::function]
    fn main(api: &Api<FunctionOutput, StdInfo>) {
        let add = api.registry().iter().find(|f| f.name() == "add").unwrap();
        let sub = api.registry().iter().find(|f| f.name() == "sub").unwrap();
        let mul = mul();

        println!("4 + 6 = {}", function_call!(add, 4, 6).unwrap().unwrap());
        println!("9 - 3 = {}", function_call!(sub, 9, 3).unwrap().unwrap());
        println!("8 * 3 = {}", function_call!(mul, 8, 3).unwrap().unwrap());
    }

    #[plux_rs::function]
    fn echo(_: (), message: &String) -> String {
        format!("Message v.2.0.0: {message}")
    }
}

pub mod parallel_plugins {
    #[allow(dead_code)]
    pub const PATH: &str = "parallel_plugins";

    pub mod one_plugin {
        use std::{collections::HashMap, sync::LazyLock};

        use plux_mock::MockPlugin;
        use plux_rs::{Bundle, StdInfo, function::{Function, FunctionOutput}};

        #[allow(dead_code)]
        pub const FILENAME: &str = "one_plugin-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, _| {
                    let requests = context.requests();
                    
                    let main = main();
                    if requests.iter().any(|r| r.name == main.name()) {
                        context.register_request(main)?;
                    }
                    
                    Ok(())
                }),
            );
        }

        #[allow(dead_code)]
        #[derive(Clone)]
        enum Tree {
            Node(Box<Tree>, Box<Tree>),
            Leaf,
        }

        #[allow(dead_code)]
        fn bottom_up_tree(depth: i32) -> Tree {
            if depth > 0 {
                let child_depth = depth - 1;
                let left = bottom_up_tree(child_depth);
                let right = bottom_up_tree(child_depth);
                Tree::Node(Box::new(left), Box::new(right))
            } else {
                Tree::Leaf
            }
        }

        #[allow(dead_code)]
        fn item_check(tree: &Tree) -> i32 {
            match tree {
                Tree::Node(left, right) => 1 + item_check(left) + item_check(right),
                Tree::Leaf => 1,
            }
        }

        #[plux_rs::function]
        fn main(_: (), n: &i32) {
            let min_depth = 4;
            let mut max_depth = min_depth + 2;
            if max_depth < *n {
                max_depth = *n;
            }

            let _stretch_tree = bottom_up_tree(max_depth + 1);

            let _long_lived_tree = bottom_up_tree(max_depth);

            for depth in (min_depth..=max_depth).step_by(2) {
                let iterations = 1 << (max_depth - depth + min_depth); // 2^(maxdepth - depth + mindepth)
                let mut _check = 0;
                for _ in 0..iterations {
                    _check += item_check(&bottom_up_tree(depth));
                }
            }
        }
    }

    pub mod two_plugin {
        use std::{collections::HashMap, sync::LazyLock};

        use plux_mock::MockPlugin;
        use plux_rs::{Bundle, StdInfo, function::{Function, FunctionOutput}};

        #[allow(dead_code)]
        pub const FILENAME: &str = "two_plugin-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |mut context, _| {
                    let requests = context.requests();
                    
                    let main = main();
                    if requests.iter().any(|r| r.name == main.name()) {
                        context.register_request(main)?;
                    }
                    
                    Ok(())
                }),
            );
        }

        #[allow(dead_code)]
        #[derive(Clone)]
        enum Tree {
            Node(Box<Tree>, Box<Tree>),
            Leaf,
        }

        #[allow(dead_code)]
        fn bottom_up_tree(depth: i32) -> Tree {
            if depth > 0 {
                let child_depth = depth - 1;
                let left = bottom_up_tree(child_depth);
                let right = bottom_up_tree(child_depth);
                Tree::Node(Box::new(left), Box::new(right))
            } else {
                Tree::Leaf
            }
        }

        #[allow(dead_code)]
        fn item_check(tree: &Tree) -> i32 {
            match tree {
                Tree::Node(left, right) => 1 + item_check(left) + item_check(right),
                Tree::Leaf => 1,
            }
        }

        #[plux_rs::function]
        fn main(_: (), n: &i32) {
            let min_depth = 4;
            let mut max_depth = min_depth + 2;
            if max_depth < *n {
                max_depth = *n;
            }

            let _stretch_tree = bottom_up_tree(max_depth + 1);

            let _long_lived_tree = bottom_up_tree(max_depth);

            for depth in (min_depth..=max_depth).step_by(2) {
                let iterations = 1 << (max_depth - depth + min_depth); // 2^(maxdepth - depth + mindepth)
                let mut _check = 0;
                for _ in 0..iterations {
                    _check += item_check(&bottom_up_tree(depth));
                }
            }
        }
    }
}

pub mod plugin_function {
    #[allow(dead_code)]
    pub const PATH: &str = "plugin_function";

    pub mod circle {
        use std::{collections::HashMap, sync::LazyLock};

        use plux_mock::MockPlugin;
        use plux_rs::{Bundle, StdInfo, function::FunctionOutput};

        #[allow(dead_code)]
        pub const FILENAME: &str = "circle-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |_, api| {
                    let plugin = api
                        .get_plugin_mut_by_bundle(&BUNDLE)
                        .ok_or("Plugin not found")?;

                    plugin.register_function(circle())?;

                    Ok(())
                }),
            );
        }

        #[plux_rs::function]
        fn circle(_: (), message: &String) {
            println!("{}:", message);
            println!("#*#");
            println!("***");
            println!("#*#");
        }
    }

    pub mod square {
        use std::{collections::HashMap, sync::LazyLock};

        use plux_mock::MockPlugin;
        use plux_rs::{Bundle, StdInfo, function::FunctionOutput};

        #[allow(dead_code)]
        pub const FILENAME: &str = "square-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: StdInfo = StdInfo::new();

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |_, api| {
                    let plugin = api
                        .get_plugin_mut_by_bundle(&BUNDLE)
                        .ok_or("Plugin not found")?;

                    plugin.register_function(square())?;

                    Ok(())
                }),
            );
        }

        #[plux_rs::function]
        fn square(_: ()) {
            println!("***");
            println!("***");
            println!("***");
        }
    }

    pub mod paint {
        use std::{
            collections::HashMap,
            sync::{Arc, LazyLock},
        };

        use plux_mock::MockPlugin;
        use plux_rs::{Api, Bundle, Depend, StdInfo, function::FunctionOutput};
        use semver::{Version, VersionReq};

        #[allow(dead_code)]
        pub const FILENAME: &str = "paint-v1.0.0.mock";
        #[allow(dead_code)]
        pub const BUNDLE: LazyLock<Bundle> =
            LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
        #[allow(dead_code)]
        pub const INFO: LazyLock<StdInfo> = LazyLock::new(|| StdInfo {
            depends: vec![Depend::new(
                "circle".to_string(),
                VersionReq::parse("1.0.0").unwrap(),
            )],
            optional_depends: vec![Depend::new(
                "square".to_string(),
                VersionReq::parse("1.0.0").unwrap(),
            )],
        });

        #[allow(dead_code)]
        pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
            map.insert(
                BUNDLE.clone(),
                MockPlugin::new(INFO.clone(), |_, api| {
                    let api = Arc::new(api);

                    let plugin = api
                        .get_plugin_mut_by_bundle(&BUNDLE)
                        .ok_or("Plugin not found")?;

                    plugin.register_function(paint(api.clone()))?;

                    Ok(())
                }),
            );
        }

        #[plux_rs::function]
        fn paint(api: &Arc<Api<FunctionOutput, StdInfo>>, is_circle: &bool) {
            if *is_circle {
                let _ = api
                    .call_function_depend(
                        "circle",
                        &Version::parse("1.0.0").unwrap(),
                        "circle",
                        &["Hello world".into()],
                    )
                    .unwrap();
            } else {
                let is_exists = api
                    .call_function_optional_depend(
                        "square",
                        &Version::parse("1.0.0").unwrap(),
                        "square",
                        &[],
                    )
                    .unwrap();
                if is_exists.is_some() {
                    println!("Square function is successfully called");
                }
            }
        }
    }
}

mod template {
    use std::{collections::HashMap, sync::LazyLock};

    use plux_mock::MockPlugin;
    use plux_rs::{Api, Bundle, StdInfo, function::FunctionOutput};

    #[allow(dead_code)]
    pub const FILENAME: &str = "template-v1.0.0.mock";
    #[allow(dead_code)]
    pub const BUNDLE: LazyLock<Bundle> = LazyLock::new(|| Bundle::from_filename(FILENAME).unwrap());
    #[allow(dead_code)]
    pub const INFO: StdInfo = StdInfo::new();

    #[allow(dead_code)]
    pub fn insert_plugin<'a>(map: &mut HashMap<Bundle, MockPlugin<'a, FunctionOutput>>) {
        map.insert(
            BUNDLE.clone(),
            MockPlugin::new(INFO.clone(), |mut context, api| {
                let plugin = api
                    .get_plugin_mut_by_bundle(&BUNDLE)
                    .ok_or("Plugin not found")?;

                plugin.register_function(function())?;

                context.register_request(entrypoint(api))?;

                Ok(())
            }),
        );
    }

    #[plux_rs::function]
    fn function(_: (), a: &i32, b: &i32) -> i32 {
        a * b
    }

    #[plux_rs::function]
    fn entrypoint(_api: &Api<FunctionOutput, StdInfo>) {
        unimplemented!();
    }
}
