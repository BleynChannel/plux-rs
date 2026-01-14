use plux_rs::{Api, LoadPluginContext, StdInfo, utils::ManagerResult};

pub struct MockPlugin<'a, O: Send + Sync + 'static> {
    pub(crate) info: StdInfo,
    pub(crate) on_load_plugin: Box<
        dyn Fn(LoadPluginContext<'a, '_, O, StdInfo>, Api<O, StdInfo>) -> ManagerResult<()>
            + Send
            + Sync,
    >,
}

impl<'a, O: Send + Sync> MockPlugin<'a, O> {
    pub fn new<F>(info: StdInfo, on_load_plugin: F) -> Self
    where
        F: Fn(LoadPluginContext<'a, '_, O, StdInfo>, Api<O, StdInfo>) -> ManagerResult<()>
            + Send
            + Sync
            + 'static,
    {
        Self {
            info,
            on_load_plugin: Box::new(on_load_plugin),
        }
    }
}
