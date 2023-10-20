use super::super::config::{Config, EnvAlias, EnvVariable};
use super::Shell;
use anyhow::{anyhow, Context, Result};
use std::path::Path;

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

pub fn get_vars_to_set(config: &Config, new_path: &str) -> Result<Vec<EnvVariable>> {
    // Env variables defined in the config
    let mut variables: Vec<EnvVariable> = vec![];
    variables.extend(
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
                dir.vars
                    .unwrap_or(vec![])
                    .iter()
                    .map(|var| EnvVariable {
                        name: var.name.clone(),
                        value: var.value.clone(),
                    })
                    .collect::<Vec<EnvVariable>>()
            })
            .collect::<Vec<EnvVariable>>(),
    );

    let mut file_vars: Vec<EnvVariable> = vec![];

    // Env variabled defined in the file specified in the config
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

            file_vars.extend(parse_env_file(&content, &file.to_string())?);
        }
    }

    // Variables that where individually defined
    variables.extend(
        config
            .variables
            .clone()
            .unwrap_or(vec![])
            .into_iter()
            .filter(|var| {
                var.dirs.iter().any(|path| {
                    let base_path = Path::new(path);
                    let path = Path::new(new_path);
                    path.starts_with(base_path) || base_path == path
                })
            })
            .map(|var| EnvVariable {
                name: var.name,
                value: var.value,
            })
            .collect::<Vec<EnvVariable>>(),
    );

    let mut ext_file_vars: Vec<EnvVariable> = vec![];

    let matched_files = config
        .files
        .clone()
        .unwrap_or(vec![])
        .into_iter()
        .filter(|file| {
            file.dirs.iter().any(|path| {
                let base_path = Path::new(path);
                let path = Path::new(new_path);
                path.starts_with(base_path) || base_path == path
            })
        });

    for env_file in matched_files {
        let base_path = Path::new(new_path);
        let path = Path::new(&env_file.load_from);

        let file_path = base_path.join(Path::new(&path));
        let content = std::fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", env_file.load_from))?;

        ext_file_vars.extend(parse_env_file(&content, &env_file.load_from)?);
    }

    variables.extend(file_vars);
    variables.extend(ext_file_vars);
    Ok(variables)
}

pub fn get_vars_to_unset(config: &Config, old_path: &str) -> Vec<String> {
    get_vars_to_set(config, old_path)
        .unwrap_or(vec![])
        .iter()
        .map(|var| var.name.clone())
        .collect()
}

pub fn get_commands_to_run(config: &Config, new_path: &str) -> Vec<String> {
    let mut commands: Vec<String> = vec![];
    commands.extend(
        config
            .directories
            .clone()
            .into_iter()
            .filter(move |dir| {
                let path_to_check = Path::new(&dir.path);
                let path = Path::new(new_path);
                // only run commands for actual directory
                // ignore sub directories
                path_to_check == path
            })
            .flat_map(|dir| {
                dir.run
                    .unwrap_or(vec![])
                    .iter()
                    .map(|cmd| cmd.clone())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>(),
    );

    commands.extend(
        config
            .commands
            .clone()
            .unwrap_or(vec![])
            .into_iter()
            .filter(|cmd| {
                cmd.dirs.iter().any(|path| {
                    let base_path = Path::new(path);
                    let path = Path::new(new_path);
                    // only exact match
                    base_path == path
                })
            })
            .map(|cmd| cmd.run)
            .collect::<Vec<String>>(),
    );

    commands
}

pub fn get_aliases_to_set(config: &Config, new_path: &str) -> Result<Vec<EnvAlias>> {
    let mut aliases: Vec<EnvAlias> = vec![];

    aliases.extend(
        config
            .directories
            .clone()
            .into_iter()
            .filter(move |dir| {
                let path_to_check = Path::new(&dir.path);
                let path = Path::new(new_path);
                path.starts_with(path_to_check) || path_to_check == path
            })
            .flat_map(|dir| dir.aliases.unwrap_or(vec![]))
            .collect::<Vec<EnvAlias>>(),
    );

    aliases.extend(
        config
            .aliases
            .clone()
            .unwrap_or(vec![])
            .into_iter()
            .filter(|alias| {
                alias.paths.iter().any(|path| {
                    let base_path = Path::new(path);
                    let path = Path::new(new_path);
                    path.starts_with(base_path) || base_path == path
                })
            })
            .map(|alias| EnvAlias {
                name: alias.name,
                commands: alias.commands,
            })
            .collect::<Vec<EnvAlias>>(),
    );

    Ok(aliases)
}

pub fn get_aliases_to_unset(config: &Config, old_path: &str) -> Vec<String> {
    get_aliases_to_set(config, old_path)
        .unwrap_or(vec![])
        .iter()
        .map(|alias| alias.name.clone())
        .collect()
}

pub fn run(config: &Config, old_path: String, new_path: String) -> Result<()> {
    let global_config = config.clone().config.unwrap_or_default();
    let to_set = get_vars_to_set(&config, &new_path)?;
    let to_unset = get_vars_to_unset(&config, &old_path);

    for var in to_unset {
        println!("unset {}", var);
    }

    if global_config.env_hints.unwrap_or(false) && to_set.len() > 0 {
        let gray_start = r"\e[90m";
        let gray_end = r"\e[0m";
        println!(
            "echo \"{}[cdwe] available env vars: {}{}\"",
            gray_start,
            to_set
                .iter()
                .map(|var| var.name.clone())
                .collect::<Vec<String>>()
                .join(", "),
            gray_end
        );
    }

    for var in to_set {
        println!("export {}=\"{}\"", var.name, var.value);
    }

    let commands = get_commands_to_run(&config, &new_path);

    for cmd in commands {
        if global_config.run_hints.unwrap_or(false) {
            let gray_start = r"\e[90m";
            let gray_end = r"\e[0m";
            println!(
                "echo \"{}[cdwe] running command: {}{}\"",
                gray_start, cmd, gray_end
            );
        }
        println!("{}", cmd);
    }

    let aliases_to_unset = get_aliases_to_unset(&config, &old_path);

    for alias in aliases_to_unset {
        println!("unset -f {} &> /dev/null", alias);
    }

    let aliases = get_aliases_to_set(&config, &new_path)?;

    if global_config.alias_hints.unwrap_or(false) && aliases.len() > 0 {
        let gray_start = r"\e[90m";
        let gray_end = r"\e[0m";
        println!(
            "echo \"{}[cdwe] available aliases: {}{}\"",
            gray_start,
            aliases
                .iter()
                .map(|alias| alias.name.clone())
                .collect::<Vec<String>>()
                .join(", "),
            gray_end
        );
    }

    for alias in aliases {
        let (start_str, end_str) = Shell::from_string(
            &config
                .config
                .clone()
                .unwrap_or_default()
                .shell
                .context("failed to get configured shell, make sure it is in your config")?
                .to_string(),
        )?
        .get_alias_command();

        let mut alias_string = start_str.replace("{{{alias_name}}}", &alias.name);

        for cmd in alias.commands {
            alias_string.push_str(&format!("{}\n", cmd));
        }

        println!("{}\n{}\n", alias_string, end_str);
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
