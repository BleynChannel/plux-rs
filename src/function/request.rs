use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::variable::VariableType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub inputs: Vec<VariableType>,
    pub output: Option<VariableType>,
}

impl Request {
    pub fn new<S: Into<String>>(
        name: S,
        inputs: Vec<VariableType>,
        output: Option<VariableType>,
    ) -> Self {
        Self {
            name: name.into(),
            inputs,
            output,
        }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}) -> {}",
            self.name,
            self.inputs
                .iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<_>>()
                .join(", "),
            match self.output {
                Some(x) => format!("{x}"),
                None => "void".to_string(),
            }
        )
    }
}
