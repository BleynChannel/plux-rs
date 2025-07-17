use std::sync::Arc;

use crate::{
    function::{Function, Request},
    utils::RegisterManagerError,
    Info, Loader, Manager,
};

pub struct LoaderContext<'a, 'b, O: Send + Sync, I: Info> {
    loader: &'b mut Loader<'a, O, I>,
}

impl<'a, 'b, O: Send + Sync, I: Info> LoaderContext<'a, 'b, O, I> {
    pub(crate) fn new(loader: &'b mut Loader<'a, O, I>) -> Self {
        Self { loader }
    }

    pub fn register_manager<M>(&mut self, manager: M) -> Result<(), RegisterManagerError>
    where
        M: Manager<'a, O, I> + 'static,
    {
        self.loader.register_manager(manager)
    }

    //TODO: Добавить параллельную версию метода
    pub fn register_managers<M>(&mut self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoIterator<Item = Box<dyn Manager<'a, O, I>>>,
    {
        self.loader.register_managers(managers)
    }

    pub fn register_request(&mut self, request: Request) {
        self.loader.requests.push(request);
    }

    //TODO: Добавить параллельную версию метода
    pub fn register_requests<IT>(&mut self, requests: IT)
    where
	IT: IntoIterator<Item = Request>,
    {
        self.loader.requests.extend(requests);
    }

    pub fn register_function<F>(&mut self, function: F)
    where
        F: Function<Output = O> + 'static,
    {
        self.loader.registry.push(Arc::new(function));
    }

    //TODO: Добавить параллельную версию метода
    pub fn register_functions<F, IT>(&mut self, functions: IT)
    where
        F: Function<Output = O> + 'static,
        IT: IntoIterator<Item = F>,
    {
        self.loader.registry.extend(
            functions
                .into_iter()
                .map(|f| Arc::new(f) as Arc<dyn Function<Output = O>>),
        );
    }
}
