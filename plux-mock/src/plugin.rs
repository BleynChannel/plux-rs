use plux_rs::{Api, LoadPluginContext, StdInfo, utils::ManagerResult};

pub struct MockPlugin<'a, O: Send + Sync + 'static> {
    pub(crate) info: Option<StdInfo>,
    pub(crate) on_load_plugin: Option<
        Box<
            dyn FnOnce(LoadPluginContext<'a, '_, O, StdInfo>, Api<O, StdInfo>) -> ManagerResult<()>
                + Send
                + Sync,
        >,
    >,
}

impl<'a, O: Send + Sync> MockPlugin<'a, O> {
    pub fn new<F>(info: StdInfo, on_load_plugin: F) -> Self
    where
        F: FnOnce(LoadPluginContext<'a, '_, O, StdInfo>, Api<O, StdInfo>) -> ManagerResult<()>
            + Send
            + Sync
            + 'static,
    {
        Self {
            info: Some(info),
            on_load_plugin: Some(Box::new(on_load_plugin)),
        }
    }
}
