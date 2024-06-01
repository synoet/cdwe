use crate::config::{Config, EnvAlias, EnvVariable, EnvVariableStruct, EnvVariableVec};
use crate::cache::Cache;
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

// TODO: implement loading from .env file
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


pub fn run(config: &Config, cache: &Cache, old_path: String, new_path: String) -> Result<()> {
    let old_cached_dir = cache.get(&old_path);
    let new_cached_dir = cache.get(&new_path);

    let global_config = config.clone().config.unwrap_or_default();
    let to_set = match new_cached_dir {
        Some(dir) => dir.variables.clone(),
        None => vec![],
    };

    let to_unset = match old_cached_dir {
        Some(dir) => dir.variables.clone().iter().map(|var| var.name.clone()).collect(),
        None => vec![],
    };

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

    let commands = match new_cached_dir {
        Some(dir) => dir.run.clone(),
        None => vec![],
    };

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

    let aliases_to_unset = match old_cached_dir {
        Some(dir) => dir.aliases.clone().iter().map(|alias| alias.name.clone()).collect(),
        None => vec![],
    };

    for alias in aliases_to_unset {
        println!("unset -f {} &> /dev/null", alias);
    }

    let aliases = match new_cached_dir {
        Some(dir) => dir.aliases.clone(),
        None => vec![],
    };

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
