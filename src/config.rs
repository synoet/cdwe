use crate::cmd::Shell;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type EnvVariableVec = Vec<EnvVariable>;
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum EnvVariableStruct {
    HashMap(HashMap<String, String>),
    EnvVariableVec(EnvVariableVec),
}

impl Default for EnvVariableStruct {
    fn default() -> Self {
        EnvVariableStruct::EnvVariableVec(vec![])
    }
}

impl From<EnvVariableStruct> for EnvVariableVec {
    fn from(env_variable: EnvVariableStruct) -> Self {
        match env_variable {
            EnvVariableStruct::EnvVariableVec(dir_env_variable) => dir_env_variable,
            EnvVariableStruct::HashMap(hash_map) => {
                // Convert the HashMap into a Vec<DirEnvVariable>
                hash_map
                    .into_iter()
                    .map(|(name, value)| EnvVariable { name, value })
                    .collect()
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub config: Option<GlobalConfig>,
    #[serde(rename = "directory")]
    pub directories: Vec<EnvDirectory>,
    #[serde(rename = "env_variable")]
    pub variables: Option<Vec<DirEnvVariable>>,
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

impl Config {
    pub fn default_for_shell(shell: Shell) -> Self {
        Config {
            config: Some(GlobalConfig {
                shell: Some(shell.to_string()),
                ..Default::default()
            }),
            directories: vec![EnvDirectory {
                path: "~".to_string(),
                vars: None,
                load_from: None,
                run: None,
                aliases: None,
            }],
            variables: None,
            commands: None,
            files: None,
            aliases: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GlobalConfig {
    pub shell: Option<String>,
    pub cd_command: Option<String>,
    pub env_hints: Option<bool>,
    pub run_hints: Option<bool>,
    pub alias_hints: Option<bool>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        GlobalConfig {
            shell: None,
            cd_command: None,
            env_hints: Some(true),
            run_hints: Some(true),
            alias_hints: Some(true),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EnvDirectory {
    pub path: String,
    pub vars: Option<EnvVariableStruct>,
    pub load_from: Option<Vec<String>>,
    pub run: Option<Vec<String>>,
    pub aliases: Option<Vec<EnvAlias>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EnvAlias {
    pub name: String,
    pub commands: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DirectoryEnvAlias {
    pub name: String,
    pub commands: Vec<String>,
    pub paths: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DirEnvVariable {
    pub name: String,
    pub value: String,
    pub dirs: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EnvVariable {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EnvCommand {
    pub run: String,
    pub dirs: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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
