use super::super::config::Config;
use super::Shell;
use anyhow::{Context, Result};
use std::path::Path;

pub fn init_shell(config: Option<Config>, shell: Shell) -> Result<()> {
    let home_var = std::env::var("HOME").context("no $HOME set")?;
    let home = Path::new(&home_var);
    let toml_path = std::path::Path::join(home, ".cdwe.toml")
        .to_str()
        .context("failed to get toml path")?
        .to_string();

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

    let cd_command = match config.and_then(|c| c.config) {
        Some(global_config) => global_config
            .cd_command
            .unwrap_or_else(|| shell.get_default_command().to_string()),
        None => shell.get_default_command().to_string(),
    };

    shell_script = shell_script.replace("{{{cd_command}}}", &cd_command);

    std::fs::write(&shell_script_target, shell_script)?;

    let toml_content: String = std::fs::read_to_string(&config_path).unwrap_or("".to_string());

    if toml_content.is_empty() {
        let default_config = Config::default_for_shell(shell);
        std::fs::write(&toml_path, toml::to_string(&default_config)?)
            .context("failed to write default config")?;
    }

    let source_string = format!(
        "if [ -f '{}' ]; then . '{}'; fi",
        &shell_script_target, &shell_script_target
    );

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
    let source_string = format!(
        "if [ -f '{}' ]; then . '{}'; fi",
        &shell_script_target, &shell_script_target
    );
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
