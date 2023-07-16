mod cmd;
mod config;
use anyhow::Result;
use clap::Parser;
use cmd::{Cli, init_shell, run};
use config::Config;

fn main() -> Result<()> {
    let matches = Cli::parse();
    let config = Config::from_config_file(&format!("{}/{}", std::env::var("HOME").unwrap(), "cdwe.toml"))?;

    match matches.command {
        cmd::Commands::Init { shell } => {
            init_shell(shell.unwrap())?;
        }
        cmd::Commands::Run { old_dir, new_dir } => {
            run(&config, old_dir, new_dir)?;
        }
    }

    Ok(())
}
