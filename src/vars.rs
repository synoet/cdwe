use crate::config::{Config, EnvDirectory};
use std::collections::HashMap;

#[derive(Debug)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

pub fn get_vars_to_set(config: &Config, new_path: &str) -> Vec<EnvVar> {
    config
        .directories
        .iter()
        .find(|dir| dir.path == new_path)
        .unwrap_or(&EnvDirectory {
            path: String::from(""),
            vars: HashMap::new(),
        })
        .vars
        .iter()
        .map(|var| EnvVar {
            key: var.0.clone(),
            value: var.1.clone(),
        })
        .collect()
}

pub fn get_vars_to_unset(config: &Config, old_path: &str) -> Vec<String> {
    get_vars_to_set(config, old_path)
        .iter()
        .map(|var| var.key.clone())
        .collect()
}
