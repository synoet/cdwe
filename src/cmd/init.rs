use super::super::config::{Config, GlobalConfig};
use anyhow::{Context, Result};
use clap::ValueEnum;
use std::path::Path;

#[derive(Debug, ValueEnum, Clone)]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
}

impl Shell {
    fn get_config_path(&self) -> Result<String> {
        let home_var = std::env::var("HOME").context("no $HOME set")?;
        let home = Path::new(&home_var);
        match self {
            Shell::Bash => Ok(std::path::Path::join(home, ".bashrc")
                .to_str()
                .context("failed to get bash config path")?
                .to_string()),
            Shell::Fish => Ok(std::path::Path::join(home, "/config/fish/config.fish")
                .to_str()
                .context("failed to get fish config path")?
                .to_string()),
            Shell::Zsh => Ok(std::path::Path::join(home, ".zshrc")
                .to_str()
                .context("failed to get zsh config path")?
                .to_string()),
        }
    }

    fn get_shell_script(&self) -> String {
        match self {
            Shell::Bash => include_str!("../../shells/cdwe_bash.txt").to_string(),
            Shell::Fish => include_str!("../../shells/cdwe_fish.txt").to_string(),
            Shell::Zsh => include_str!("../../shells/cdwe_zsh.txt").to_string(),
        }
    }

    fn get_shell_script_target(&self) -> Result<String> {
        let home_var = std::env::var("HOME").context("no $HOME set")?;
        let home = Path::new(&home_var);
        match self {
            Shell::Bash => Ok(std::path::Path::join(home, ".cdwe.bash")
                .to_str()
                .context("failed to get bash target")?
                .to_string()),
            Shell::Fish => Ok(std::path::Path::join(home, ".cdwe.fish")
                .to_str()
                .context("failed to get fish target")?
                .to_string()),
            Shell::Zsh => Ok(std::path::Path::join(home, ".cdwe.zsh")
                .to_str()
                .context("failed to get zsh target")?
                .to_string()),
        }
    }

    fn get_default_command(&self) -> String {
        match self {
            Shell::Bash => "builtin cd".to_string(),
            Shell::Fish => "cd".to_string(),
            Shell::Zsh => "builtin cd".to_string(),
        }
    }
}

pub fn init_shell(config: Option<Config>, shell: Shell) -> Result<()> {
    let config_path = shell.get_config_path()?;
    let mut shell_script = shell.get_shell_script();
    let shell_script_target = shell.get_shell_script_target()?;
    let exe_path = std::env::current_exe().context("failed to get cdwe executable path")?;
    shell_script = shell_script.replace(
        "{{{exec_path}}}",
        exe_path
            .to_str()
            .context("failed to convert path to string")?,
    );

    match config {
        Some(config) => {
            let cd_command = config
                .config
                .unwrap_or(GlobalConfig {
                    cd_command: shell.get_default_command(),
                })
                .cd_command;
            shell_script = shell_script.replace("{{{cd_command}}}", cd_command.as_str());
        }
        _ => {
            shell_script = shell_script.replace("{{{cd_command}}}", &shell.get_default_command());
        }
    }

    std::fs::write(&shell_script_target, shell_script)?;

    let source_string = format!("if [ -f '{}' ]; then . '{}'; fi", &shell_script_target, &shell_script_target);

    let mut config = std::fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read config path {}", config_path))?;
    if !config.contains(&source_string) {
        config.push_str(&format!("\n{}", source_string));
        std::fs::write(&config_path, config)
            .with_context(|| format!("failed to write to config path {}", config_path))?;
    }
    Ok(())
}

pub fn remove_shell(shell: Shell) -> Result<()> {
    let shell_script_target = shell.get_shell_script_target()?;
    let config_path = shell.get_config_path()?;
    let source_string = format!("if [ -f '{}' ]; then . '{}'; fi", &shell_script_target, &shell_script_target);
    let mut config = std::fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read config path {}", config_path))?;

    if config.contains(&source_string) {
        config = config.replace(&source_string, "");
        std::fs::write(&config_path, config)
            .with_context(|| format!("Failed to write to {}", &config_path))?;
    }

    std::fs::remove_file(&shell_script_target)
        .with_context(|| format!("failed to remove config file {}", &shell_script_target))?;

    Ok(())
}
