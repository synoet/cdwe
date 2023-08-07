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
    pub fn get_config_path(&self) -> Result<String> {
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

    pub fn to_string(&self) -> String {
        match self {
            Shell::Bash => "bash".to_string(),
            Shell::Fish => "fish".to_string(),
            Shell::Zsh => "zsh".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Result<Self> {
        match s {
            "bash" => Ok(Shell::Bash),
            "fish" => Ok(Shell::Fish),
            "zsh" => Ok(Shell::Zsh),
            _ => Err(anyhow::anyhow!("invalid shell")),
        }
    }

    pub fn get_shell_script(&self) -> String {
        match self {
            Shell::Bash => include_str!("../../shells/cdwe_bash.txt").to_string(),
            Shell::Fish => include_str!("../../shells/cdwe_fish.txt").to_string(),
            Shell::Zsh => include_str!("../../shells/cdwe_zsh.txt").to_string(),
        }
    }

    pub fn get_shell_script_target(&self) -> Result<String> {
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

    pub fn get_default_command(&self) -> String {
        match self {
            Shell::Bash => "builtin cd".to_string(),
            Shell::Fish => "cd".to_string(),
            Shell::Zsh => "builtin cd".to_string(),
        }
    }

    pub fn get_alias_command(&self) -> (String, String) {
        match self {
            Shell::Zsh => ("{{{alias_name}}}() {{\n".to_string(), "{}\n".to_string()),
            Shell::Bash => ("{{{alias_name}}}() {{\n".to_string(), "{}\n".to_string()),
            Shell::Fish => (
                "function {{{alias_name}}} -d \"{}\"\n".to_string(),
                "end\n".to_string(),
            ),
        }
    }
}
