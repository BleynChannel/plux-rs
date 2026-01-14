use std::{collections::HashMap, path::PathBuf};

use plux_mock::MockPlugin;
use plux_rs::{Manager, prelude::*};

pub fn get_plugin_path(filename: impl AsRef<str>) -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join(format!("./tests/plugins/{}", filename.as_ref()))
}

#[allow(dead_code)]
pub fn loader_init<'a, M>(manager: M) -> Loader<'a, FunctionOutput, StdInfo>
where
    M: Manager<'a, FunctionOutput, StdInfo> + 'static,
{
    let mut loader = Loader::new();
    loader
        .context(move |mut ctx| ctx.register_manager(manager))
        .unwrap();
    loader
}

#[allow(dead_code)]
pub fn plugins_init<'a>(
    plugin_filenames: Vec<&str>,
) -> HashMap<Bundle, MockPlugin<'a, FunctionOutput>> {
    plugin_filenames
        .into_iter()
        .map(|filename| {
            let bundle = Bundle::from_filename(filename).unwrap();
            let plugin = MockPlugin::new(StdInfo::new(), |_context, _api| Ok(()));
            (bundle, plugin)
        })
        .collect::<HashMap<Bundle, MockPlugin<'a, FunctionOutput>>>()
}

#[allow(dead_code)]
pub fn benchmark<F, R>(f: F) -> (std::time::Duration, R)
where
    F: FnOnce() -> R,
{
    let timer = std::time::Instant::now();
    let data = f();
    (timer.elapsed(), data)
}
