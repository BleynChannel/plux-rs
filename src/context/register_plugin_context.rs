use std::path::PathBuf;

use crate::Bundle;

pub struct RegisterPluginContext<'a> {
    pub path: &'a PathBuf,
    pub bundle: &'a Bundle,
}
