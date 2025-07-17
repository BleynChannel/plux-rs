use std::{collections::HashMap, fs, path::PathBuf};

use august_plugin_system::{utils::ManagerResult, Depend, StdInfo};
use semver::VersionReq;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub description: String,
    pub author: String,
    pub license: Option<String>,
    pub depends: Option<HashMap<String, VersionReq>>,
    pub optional_depends: Option<HashMap<String, VersionReq>>,
}

#[derive(thiserror::Error, Debug)]
pub enum RegisterPluginError {
    #[error("Does not contain config")]
    DoesNotContainConfig,
}

pub fn load_config(plugin_path: &PathBuf) -> ManagerResult<(Config, StdInfo)> {
    // Получаем конфигурацию плагина
    let config_path = plugin_path.join("config.toml");
    if !config_path.exists() {
        return Err(Box::new(RegisterPluginError::DoesNotContainConfig));
    }

    let config_content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;

    //Заполняем информацию про плагин
    let info = StdInfo {
        depends: config.depends.clone().map_or(vec![], |depends| {
            depends
                .into_iter()
                .map(|(id, version)| Depend::new(id, version))
                .collect()
        }),
        optional_depends: config.optional_depends.clone().map_or(vec![], |depends| {
            depends
                .into_iter()
                .map(|(id, version)| Depend::new(id, version))
                .collect()
        }),
    };

    Ok((config, info))
}
