use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "directory")]
    pub directories: Vec<EnvDirectory>,
    pub config: Option<GlobalConfig>,
}

#[derive(Deserialize, Debug)]
pub struct GlobalConfig {
    pub cd_command: String,
}

#[derive(Deserialize, Debug)]
pub struct EnvDirectory {
    pub path: String,
    pub vars: HashMap<String, String>,
}

impl Config {
    pub fn from_config_file(path: &str) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Could not read config file at {}", path))?;

        let config: Config = toml::from_str(&contents)
            .with_context(|| format!("Could not parse config file at {}", path))?;

        Ok(config)
    }
}
