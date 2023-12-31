mod cmd;
mod config;
use anyhow::{Context, Result};
use clap::Parser;
use cmd::{init_shell, remove_shell, run, Cli};
use config::Config;

fn main() -> Result<()> {
    let matches = Cli::parse();

    match matches.command {
        cmd::Commands::Init { shell } => {
            init_shell(None, shell.unwrap())?;
        }
        cmd::Commands::Run { old_dir, new_dir } => {
            let config = Config::from_config_file(&format!(
                "{}/{}",
                std::env::var("HOME").context("failed to get $HOME env var")?,
                "cdwe.toml"
            ))?;
            run(&config, old_dir, new_dir)?;
        }
        cmd::Commands::Reload { shell } => {
            let config = Config::from_config_file(&format!(
                "{}/{}",
                std::env::var("HOME").context("failed to get $HOME env var")?,
                "cdwe.toml"
            ))?;
            init_shell(Some(config), shell.unwrap())?;
        }
        cmd::Commands::Remove { shell } => {
            remove_shell(shell.context("no shell passed")?)?;
        }
    }

    Ok(())
}
