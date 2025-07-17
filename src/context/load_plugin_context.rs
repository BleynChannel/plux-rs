use crate::{function::Function, utils::RegisterRequestError, Info, Plugin, Requests};

pub struct LoadPluginContext<'a, 'b, O: Send + Sync, I: Info> {
    plugin: &'b mut Plugin<'a, O, I>,
    requests: &'b Requests,
}

impl<'a, 'b, O: Send + Sync, I: Info> LoadPluginContext<'a, 'b, O, I> {
    pub(crate) fn new(plugin: &'b mut Plugin<'a, O, I>, requests: &'b Requests) -> Self {
        Self { plugin, requests }
    }

    pub const fn plugin(&'b self) -> &'b Plugin<'a, O, I> {
		self.plugin
    }

    pub const fn requests(&self) -> &'b Requests {
        self.requests
    }

    //TODO: Добавить параллельную версию
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
