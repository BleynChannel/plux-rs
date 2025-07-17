use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::variable::VariableType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct Arg {
    pub name: String,
    pub ty: VariableType,
}

impl Arg {
    pub fn new<S: Into<String>>(name: S, ty: VariableType) -> Self {
        Self {
            name: name.into(),
            ty,
        }
    }
}

impl Default for Arg {
    fn default() -> Self {
        Self {
            name: "arg".to_string(),
            ty: Default::default(),
        }
    }
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}
