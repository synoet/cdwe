use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub config: Option<GlobalConfig>,
    #[serde(rename = "directory")]
    pub directories: Vec<EnvDirectory>,
    #[serde(rename = "env_variable")]
    pub variables: Option<Vec<EnvVariable>>,
    #[serde(rename = "command")]
    pub commands: Option<Vec<EnvCommand>>,
    #[serde(rename = "env_file")]
    pub files: Option<Vec<EnvFile>>,
    #[serde(rename = "alias")]
    pub aliases: Option<Vec<DirectoryEnvAlias>>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            config: None,
            directories: vec![],
            variables: None,
            commands: None,
            files: None,
            aliases: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct GlobalConfig {
    pub cd_command: Option<String>,
    pub env_hints: Option<bool>,
    pub run_hints: Option<bool>,
    pub alias_hints: Option<bool>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        GlobalConfig {
            cd_command: None,
            env_hints: Some(true),
            run_hints: Some(true),
            alias_hints: Some(true),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct EnvDirectory {
    pub path: String,
    pub vars: Option<HashMap<String, String>>,
    pub load_from: Option<Vec<String>>,
    pub run: Option<Vec<String>>,
    pub aliases: Option<Vec<EnvAlias>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EnvAlias {
    pub name: String,
    pub commands: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DirectoryEnvAlias {
    pub name: String,
    pub commands: Vec<String>,
    pub paths: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EnvVariable {
    pub name: String,
    pub value: String,
    pub dirs: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EnvCommand {
    pub run: String,
    pub dirs: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EnvFile {
    pub load_from: String,
    pub dirs: Vec<String>,
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
