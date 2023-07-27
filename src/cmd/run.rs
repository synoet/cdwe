use super::super::config::{Config, EnvAlias};
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

fn trim_quotes(s: &str) -> String {
    if s.len() < 2 {
        return s.to_string();
    }
    let mut chars = s.chars();
    match (chars.next(), chars.next_back()) {
        (Some('"'), Some('"')) => chars.collect(),
        (Some('\''), Some('\'')) => chars.collect(),
        _ => s.to_string(),
    }
}

pub fn get_vars_to_set(config: &Config, new_path: &str) -> Result<Vec<EnvVar>> {
    let dir_vars = config
        .directories
        .clone()
        .into_iter()
        .filter(move |dir| {
            let path_to_check = Path::new(&dir.path);
            let path = Path::new(new_path);
            path.starts_with(path_to_check) || path_to_check == path
        })
        .flat_map(|dir| {
            dir.vars
                .unwrap_or(HashMap::new())
                .iter()
                .map(|var| EnvVar {
                    key: var.0.clone(),
                    value: var.1.clone(),
                })
                .collect::<Vec<EnvVar>>()
        })
        .collect();

    let mut file_vars: Vec<EnvVar> = vec![];

    for dir in config.directories.clone().into_iter() {
        let base_path = Path::new(&dir.path);
        let path = Path::new(new_path);
        if !path.starts_with(base_path) || base_path != path {
            continue;
        }
        for file in dir.load_from.unwrap_or(vec![]) {
            let file_path = base_path.join(Path::new(&file));
            let content = std::fs::read_to_string(file_path)
                .with_context(|| format!("Failed to read file: {}", file))?;

            let lines = content.lines();

            let mut vars = vec![];

            for (index, line) in lines.enumerate() {
                let mut split = line.split('=');
                if split.clone().count() != 2 {
                    return Err(anyhow!(
                        "Invalid line in file: {}:{}: {}",
                        file,
                        index,
                        line
                    ));
                }

                let key = trim_quotes(split.next().unwrap());
                let value = trim_quotes(split.next().unwrap());

                vars.push(EnvVar {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            }
            file_vars.extend(vars);
        }
    }

    let mut vars: Vec<EnvVar> = dir_vars;
    vars.extend(file_vars);
    Ok(vars)
}

pub fn get_vars_to_unset(config: &Config, old_path: &str) -> Vec<String> {
    get_vars_to_set(config, old_path)
        .unwrap_or(vec![])
        .iter()
        .map(|var| var.key.clone())
        .collect()
}

pub fn get_commands_to_run(config: &Config, new_path: &str) -> Vec<String> {
    config
        .directories
        .clone()
        .into_iter()
        .filter(move |dir| {
            let path_to_check = Path::new(&dir.path);
            let path = Path::new(new_path);
            path.starts_with(path_to_check) || path_to_check == path
        })
        .flat_map(|dir| {
            dir.run
                .unwrap_or(vec![])
                .iter()
                .map(|cmd| cmd.clone())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>()
}

pub fn get_aliases_to_set(config: &Config, new_path: &str) -> Result<Vec<EnvAlias>> {
    Ok(config
        .directories
        .clone()
        .into_iter()
        .filter(move |dir| {
            let path_to_check = Path::new(&dir.path);
            let path = Path::new(new_path);
            path.starts_with(path_to_check) || path_to_check == path
        })
        .flat_map(|dir| dir.aliases.unwrap_or(vec![]))
        .collect::<Vec<EnvAlias>>())
}

pub fn get_aliases_to_unset(config: &Config, old_path: &str) -> Vec<String> {
    get_aliases_to_set(config, old_path)
        .unwrap_or(vec![])
        .iter()
        .map(|alias| alias.name.clone())
        .collect()
}

pub fn run(config: &Config, old_path: String, new_path: String) -> Result<()> {
    let to_set = get_vars_to_set(&config, &new_path)?;
    let to_unset = get_vars_to_unset(&config, &old_path);

    for var in to_unset {
        println!("unset {}", var);
    }

    for var in to_set {
        println!("export {}=\"{}\"", var.key, var.value);
    }

    let commands = get_commands_to_run(&config, &new_path);

    for cmd in commands {
        println!("{}", cmd);
    }

    let aliases = get_aliases_to_set(&config, &new_path)?;

    for alias in aliases {
        let mut alias_string = format!("{}(){{\n", alias.name);
        for cmd in alias.commands {
            alias_string.push_str(&format!("{}\n", cmd));
        }
        println!("{}\n}}\n", alias_string);
    }

    let aliases_to_unset = get_aliases_to_unset(&config, &old_path);

    for alias in aliases_to_unset {
        println!("unset -f {}", alias);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trim_quotes() {
        use super::trim_quotes;
        assert_eq!(trim_quotes("\"test\""), "test");
        assert_eq!(trim_quotes("'test'"), "test");
        assert_eq!(trim_quotes("test"), "test");
        assert_eq!(trim_quotes("\"test"), "\"test");
        assert_eq!(trim_quotes("test'"), "test'");
    }
}
