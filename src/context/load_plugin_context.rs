use crate::{Info, Plugin, Requests, function::Function, utils::RegisterRequestError};

/// Context provided during plugin loading.
///
/// LoadPluginContext gives plugin managers access to the plugin being loaded and
/// the system's function requests. It allows managers to register plugin functions
/// and validate them against the expected interface.
///
/// # Type Parameters
///
/// * `'a` - Lifetime for references within the plugin
/// * `'b` - Lifetime of the context references
/// * `O` - Output type for plugin functions (must implement Send + Sync)
/// * `I` - Plugin information type (must implement Info trait)
///
/// # Fields
///
/// * `plugin` - Mutable reference to the plugin being loaded
/// * `requests` - Reference to the system's function requests
///
/// # Example
///
/// ```rust,no_run
/// use plux_rs::{Manager, LoadPluginContext, Api, StdInfo};
///
/// struct MyManager;
///
/// impl Manager<'_, (), StdInfo> for MyManager {
///     fn format(&self) -> &'static str { "my" }
///
///     fn load_plugin(
///         &mut self,
///         context: LoadPluginContext<'_, '_, (), StdInfo>,
///         api: Api<(), StdInfo>
///     ) -> Result<(), Box<dyn std::error::Error>> {
///         // Register plugin functions that match the requests
///         for request in context.requests() {
///             // Register corresponding function in the plugin
///         }
///         Ok(())
///     }
/// }
/// ```
pub struct LoadPluginContext<'a, 'b, O: Send + Sync, I: Info> {
    plugin: &'b mut Plugin<'a, O, I>,
    requests: &'b Requests,
}

impl<'a, 'b, O: Send + Sync, I: Info> LoadPluginContext<'a, 'b, O, I> {
    /// Creates a new load plugin context.
    ///
    /// This is an internal constructor used by the loader when loading plugins.
    ///
    /// # Parameters
    ///
    /// * `plugin` - Mutable reference to the plugin being loaded
    /// * `requests` - Reference to the system's function requests
    ///
    /// # Returns
    ///
    /// Returns a new LoadPluginContext instance.
    pub(crate) fn new(plugin: &'b mut Plugin<'a, O, I>, requests: &'b Requests) -> Self {
        Self { plugin, requests }
    }

    /// Gets a reference to the plugin being loaded.
    ///
    /// # Returns
    ///
    /// Returns an immutable reference to the plugin.
    pub const fn plugin(&'b self) -> &'b Plugin<'a, O, I> {
        self.plugin
    }

    /// Gets a reference to the system's function requests.
    ///
    /// # Returns
    ///
    /// Returns a reference to the requests that plugins should implement.
    pub const fn requests(&self) -> &'b Requests {
        self.requests
    }

    /// Registers a function that implements a system request.
    ///
    /// This method validates that the provided function matches the signature of
    /// a registered system request and then registers it with the plugin.
    ///
    /// # Parameters
    ///
    /// * `request` - The function that implements a system request
    ///
    /// # Returns
    ///
    /// Returns `Result<(), RegisterRequestError>` indicating success or failure.
    /// Fails if the function doesn't match any registered request or has incorrect arguments.
    ///
    /// # Type Parameters
    ///
    /// * `F` - Type of the function (must implement Function trait)
    pub fn register_request<F>(&mut self, request: F) -> Result<(), RegisterRequestError>
    where
        F: Function<Output = O> + 'static,
    {
        if let Some(req) = self.requests.iter().find(|req| *req.name == request.name()) {
            for input in req.inputs.iter() {
                request
                    .inputs()
                    .iter()
                    .find(|arg| *input == arg.ty)
                    .ok_or(RegisterRequestError::ArgumentsIncorrectly)?;
            }

            if req.output != request.output().map(|arg| arg.ty) {
                return Err(RegisterRequestError::ArgumentsIncorrectly);
            }
        } else {
            return Err(RegisterRequestError::NotFound);
        }

        self.plugin.requests.push(Box::new(request));

        Ok(())
    }
}
