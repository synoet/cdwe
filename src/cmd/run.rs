use super::Shell;
use crate::cache::{Cache, DirCache};
use crate::config::{EnvAlias, EnvVariable, LocalConfig};
use crate::utils::trim_quotes;
use anyhow::{anyhow, Result};
use std::path::Path;

/// Parses the content of an .env file with the following structure
/// ```
/// SOME_VAR=test
/// ANOTHER_VAR="test"
/// ````
///
/// Supports values with or without quotes
fn parse_env_file(content: &str, file_name: &str) -> Result<Vec<EnvVariable>> {
    let lines = content
        .lines()
        .filter(|line| !line.contains('#') && !line.trim().is_empty());

    let mut vars = vec![];

    for (index, line) in lines.enumerate() {
        let split = line
            .split_once('=')
            .ok_or_else(|| anyhow!("Invalid line in file: {}:{}: {}", file_name, index, line))?;

        let key = trim_quotes(split.0);
        let value = trim_quotes(split.1);

        vars.push(EnvVariable {
            name: key.to_string(),
            value: value.to_string(),
        });
    }

    Ok(vars)
}

fn get_vars_from_env_file(base_path: &str, file_path: &str) -> Option<Vec<EnvVariable>> {
    let env_path = Path::new(&base_path).join(file_path);
    if let Ok(content) = std::fs::read_to_string(&env_path) {
        match parse_env_file(&content, &env_path.to_string_lossy()) {
            Ok(vars) => Some(vars),
            Err(_) => None,
        }
    } else {
        None
    }
}

/// Given a cache unsets the environment variables for the old directory
/// variables are taken from the dir and from any .env files specified in the config
pub fn unset_variables(
    variables: &Vec<EnvVariable>,
    load_from: Option<&Vec<String>>,
    path: Option<&str>,
) {
    for var in variables.iter() {
        println!("unset {}", var.name);
    }

    // Unload variables from .env files specified in config
    // for the old directory

    if let (Some(path), Some(load_from)) = (path, load_from) {
        for file in load_from {
            let vars = get_vars_from_env_file(path, file);
            if let Some(vars) = vars {
                for var in vars {
                    println!("unset {}", var.name);
                }
            }
        }
    }
}

/// Given a cache sets the environment variables for the new directory
/// variables are taken from the dir and from any .enf files specified in the config
pub fn set_variables(
    variables: &Vec<EnvVariable>,
    load_from: Option<&Vec<String>>,
    path: Option<&str>,
) {
    for var in variables {
        println!("export {}=\"{}\"", var.name, var.value);
    }

    if let (Some(path), Some(load_from)) = (path, load_from) {
        // Load variables from .env files specified in config
        for file in load_from {
            let vars = get_vars_from_env_file(path, &file);
            if let Some(vars) = vars {
                for var in vars {
                    println!("export {}=\"{}\"", var.name, var.value);
                }
            }
        }
    }
}

pub fn set_aliases(aliases: &Vec<EnvAlias>, shell: &str) -> Result<()> {
    let (start_str, end_str) = Shell::from_string(shell)?.get_alias_command();
    for alias in aliases.iter() {
        let mut alias_string = start_str.clone().replace("{{{alias_name}}}", &alias.name);
        for cmd in &alias.commands {
            alias_string.push_str(&format!("{}\n", cmd));
        }

        println!("{}\n{}\n", &alias_string, &end_str);
    }

    Ok(())
}

pub fn unset_aliases(aliases: &Vec<EnvAlias>) {
    for alias in aliases.iter() {
        println!("unset -f {} &> /dev/null", alias.name);
    }
}

pub fn run_commands(commands: &Vec<String>) {
    for command in commands.iter() {
        println!("{}", command);
    }
}

pub fn run(cache: &Cache, old_path: String, new_path: String) -> Result<()> {
    let old_dir: Option<&DirCache> = cache.get(&old_path);
    let new_dir: Option<&DirCache> = cache.get(&new_path);

    if old_dir.is_none() && new_dir.is_none() {
        return Ok(());
    }

    // Unset old environment variables
    if let Some(old_dir) = old_dir {
        unset_variables(
            &old_dir.variables,
            Some(&old_dir.load_from),
            Some(&old_path),
        );
        unset_aliases(&old_dir.aliases);
    }

    if let Some(new_dir) = new_dir {
        set_variables(
            &new_dir.variables,
            Some(&new_dir.load_from),
            Some(&new_path),
        );
        set_aliases(&new_dir.aliases, &cache.shell)?;
        run_commands(&new_dir.run);
    }

    Ok(())
}

pub fn run_local(
    old_local_config: Option<&LocalConfig>,
    new_local_config: Option<&LocalConfig>,
    shell: &str,
) -> Result<()> {
    if old_local_config.is_none() && new_local_config.is_none() {
        return Ok(());
    }

    if let Some(old_local_config) = old_local_config {
        if let Some(vars) = &old_local_config.variables {
            unset_variables(vars, None, None);
        }

        if let Some(aliases) = &old_local_config.aliases {
            unset_aliases(aliases);
        }
    }

    if let Some(new_local_config) = new_local_config {
        if let Some(vars) = &new_local_config.variables {
            set_variables(vars, None, None);
        }

        if let Some(aliases) = &new_local_config.aliases {
            // This might be the only error worth reporting to the user
            // since it means that they have misconfigured cdwe
            if let Err(err) = set_aliases(aliases, shell) {
                eprintln!("ERROR: misconfigured shell { }", err);
            }
        }

        if let Some(commands) = &new_local_config.commands {
            run_commands(commands);
        }
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_env_file() {
        use super::parse_env_file;
        use crate::config::EnvVariable;

        let test_content = "\
            # THIS IS A TEST COMMMENT\n\
            TEST_VAR=true\n\
            ANOTHER_VAR=123\n\
            QUOTED_VAR=\"test\"\n\
            # ANOTHER TEST COMMENT\n\
            SINGLE_QUOTED_VAR='test'\n\
            ANOTHER_VAR=hello world this is a test\n\
        ";

        let expected: Vec<EnvVariable> = vec![
            EnvVariable {
                name: "TEST_VAR".to_string(),
                value: "true".to_string(),
            },
            EnvVariable {
                name: "ANOTHER_VAR".to_string(),
                value: "123".to_string(),
            },
            EnvVariable {
                name: "QUOTED_VAR".to_string(),
                value: "test".to_string(),
            },
            EnvVariable {
                name: "SINGLE_QUOTED_VAR".to_string(),
                value: "test".to_string(),
            },
            EnvVariable {
                name: "ANOTHER_VAR".to_string(),
                value: "hello world this is a test".to_string(),
            },
        ];

        assert_eq!(parse_env_file(test_content, "/.env").unwrap(), expected);
    }
}
