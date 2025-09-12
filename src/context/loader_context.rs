use std::sync::Arc;

use crate::{
    Info, Loader, Manager,
    function::{Function, Request},
    utils::RegisterManagerError,
};

/// Context for configuring the plugin loader.
///
/// LoaderContext provides a fluent interface for setting up the plugin loader with
/// managers, functions, and requests. It ensures proper initialization order and
/// provides convenient methods for loader configuration.
///
/// # Type Parameters
///
/// * `'a` - Lifetime for references within the loader
/// * `'b` - Lifetime of the loader reference
/// * `O` - Output type for plugin functions (must implement Send + Sync)
/// * `I` - Plugin information type (must implement Info trait)
///
/// # Fields
///
/// * `loader` - Mutable reference to the underlying loader
///
/// # Example
///
/// ```rust,no_run,ignore
/// use plux_rs::prelude::*;
/// use plux_custom_manager::CustomManager;
///
/// #[plux_rs::function]
/// fn my_function(_: ()) {
///     // Function implementation
/// }
/// 
/// let mut loader = Loader::new();
/// loader.context(|mut ctx| {
///     // Register a manager
///     ctx.register_manager(CustomManager::new())?;
///
///     // Register functions and requests
///     ctx.register_function(my_function());
///     ctx.register_request(Request::new("main".to_string(), vec![], None));
///
///     Ok(())
/// });
/// ```
pub struct LoaderContext<'a, 'b, O: Send + Sync, I: Info> {
    loader: &'b mut Loader<'a, O, I>,
}

impl<'a, 'b, O: Send + Sync, I: Info> LoaderContext<'a, 'b, O, I> {
    /// Creates a new loader context.
    ///
    /// This is an internal constructor used by the loader when creating contexts.
    ///
    /// # Parameters
    ///
    /// * `loader` - Mutable reference to the loader to configure
    ///
    /// # Returns
    ///
    /// Returns a new LoaderContext instance.
    pub(crate) fn new(loader: &'b mut Loader<'a, O, I>) -> Self {
        Self { loader }
    }

    /// Registers a plugin manager with the loader.
    ///
    /// This method registers a manager that can handle plugins of a specific format.
    ///
    /// # Parameters
    ///
    /// * `manager` - The manager instance to register
    ///
    /// # Returns
    ///
    /// Returns `Result<(), RegisterManagerError>` indicating success or failure.
    ///
    /// # Type Parameters
    ///
    /// * `M` - Type of the manager (must implement Manager trait)
    pub fn register_manager<M>(&mut self, manager: M) -> Result<(), RegisterManagerError>
    where
        M: Manager<'a, O, I> + 'static,
    {
        self.loader.register_manager(manager)
    }

    /// Registers multiple plugin managers with the loader.
    ///
    /// This method registers a collection of managers in sequence.
    ///
    /// # Parameters
    ///
    /// * `managers` - Iterator of manager instances to register
    ///
    /// # Returns
    ///
    /// Returns `Result<(), RegisterManagerError>` indicating success or failure.
    pub fn register_managers<M>(&mut self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoIterator<Item = Box<dyn Manager<'a, O, I>>>,
    {
        self.loader.register_managers(managers)
    }

    /// Registers a function request with the loader.
    ///
    /// Function requests define the interface that plugins must implement.
    ///
    /// # Parameters
    ///
    /// * `request` - The request to register
    pub fn register_request(&mut self, request: Request) {
        self.loader.requests.push(request);
    }

    /// Registers multiple function requests with the loader.
    ///
    /// This method registers a collection of requests.
    ///
    /// # Parameters
    ///
    /// * `requests` - Iterator of requests to register
    ///
    /// # Type Parameters
    ///
    /// * `IT` - Type of the iterator containing requests
    pub fn register_requests<IT>(&mut self, requests: IT)
    where
        IT: IntoIterator<Item = Request>,
    {
        self.loader.requests.extend(requests);
    }

    /// Registers a function in the loader's registry.
    ///
    /// Functions registered here are available to all plugins.
    ///
    /// # Parameters
    ///
    /// * `function` - The function to register
    ///
    /// # Type Parameters
    ///
    /// * `F` - Type of the function (must implement Function trait)
    pub fn register_function<F>(&mut self, function: F)
    where
        F: Function<Output = O> + 'static,
    {
        self.loader.registry.push(Arc::new(function));
    }

    /// Registers multiple functions in the loader's registry.
    ///
    /// This method registers a collection of functions.
    ///
    /// # Parameters
    ///
    /// * `functions` - Iterator of functions to register
    ///
    /// # Type Parameters
    ///
    /// * `F` - Type of the functions (must implement Function trait)
    /// * `IT` - Type of the iterator containing functions
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
