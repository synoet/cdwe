use crate::config::{Config, EnvAlias, EnvVariable, EnvVariableStruct};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type DirCacheMap = HashMap<String, DirCache>;

#[derive(Serialize, Deserialize)]
pub struct DirCache {
    pub variables: Vec<EnvVariable>,
    pub run: Vec<String>,
    pub aliases: Vec<EnvAlias>,
    pub load_from: Vec<String>,
}

/// Cache is optimized for speed of lookup
/// Config is optimized for readability and usability for the user
/// Cache is stored in a json file ussually ~/.cdwe_cache.json
#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub shell: String,
    pub hash: String,
    values: DirCacheMap,
}

impl Cache {
    pub fn new(shell: String, hash: String, values: DirCacheMap) -> Self {
        Cache {
            shell,
            hash,
            values,
        }
    }

    pub fn from_config(config: &Config, config_hash: &str) -> Self {
        let mut values: DirCacheMap = HashMap::new();

        for directory in &config.directories {
            let variables: Vec<EnvVariable> = match &directory.vars {
                Some(EnvVariableStruct::HashMap(hash_map)) => hash_map
                    .iter()
                    .map(|(name, value)| EnvVariable {
                        name: name.clone(),
                        value: value.clone(),
                    })
                    .collect(),
                Some(EnvVariableStruct::EnvVariableVec(dir_env_variable)) => {
                    dir_env_variable.to_vec()
                }
                None => vec![],
            };

            let mut aliases: Vec<EnvAlias> = vec![];

            if let Some(dir_aliases) = &directory.aliases {
                aliases.extend(dir_aliases.clone());
            }

            let load_from = directory.load_from.clone().unwrap_or(vec![]);

            let run = directory.run.clone().unwrap_or(vec![]);

            let dir_cache: DirCache = DirCache {
                variables,
                run,
                load_from: load_from.clone(),
                aliases,
            };

            values.insert(directory.path.clone(), dir_cache);
        }

        let shell = match &config.config {
            Some(global_config) => global_config.shell.clone().unwrap_or("bash".to_string()),
            None => "bash".to_string(),
        };

        Cache::new(shell, config_hash.to_string(), values)
    }

    pub fn get(&self, path: &str) -> Option<&DirCache> {
        self.values.get(path)
    }
}

/// If a cache doesn't exist create one
/// If a cache exists but the config has changed we create a new cache
/// Returns the cache and a boolean indicating if the cache was created
pub fn get_or_create_cache(
    cache_content: Option<&str>,
    config_content: &str,
    config_hash: &str,
) -> Result<(Cache, bool)> {
    if let Some(cache_content) = cache_content {
        let previous_cache: Cache = serde_json::from_str(cache_content)?;

        if previous_cache.hash == config_hash {
            return Ok((previous_cache, false));
        }
    }

    let config = Config::from_str(config_content).context("failed to parse config")?;

    Ok((Cache::from_config(&config, config_hash), true))
}

pub fn write_cache(cache: &Cache, home: &str) -> Result<()> {
    let cache_content = serde_json::to_string(cache)?;
    let home = home.to_string();
    tokio::spawn(async move {
        std::fs::write(
            home.to_string() + "/.cdwe_cache.json",
            cache_content.as_bytes(),
        )
    });

    Ok(())
}
