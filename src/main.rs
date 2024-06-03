mod cache;
mod cmd;
mod config;
mod utils;
use anyhow::{Context, Result};
use clap::Parser;
use cmd::{init_shell, remove_shell, run, run_local, Cli};
use config::{Config, LocalConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Cli::parse();
    let home = std::env::var("HOME").context("no $HOME set")?;
    let config_path = format!("{}/{}", &home, "cdwe.toml");
    let cache_path = format!("{}/{}", &home, ".cdwe_cache.json");

    match matches.command {
        cmd::Commands::Init { shell } => init_shell(None, shell.unwrap())?,
        cmd::Commands::Run { old_dir, new_dir } => {
            let local_config_path = format!("{}/{}", new_dir, "cdwe.toml");
            let old_local_config_path = format!("{}/{}", old_dir, "cdwe.toml");

            let contents = std::fs::read_to_string(&config_path)
                .with_context(|| format!("Could not read config file at {}", &config_path))?;
            let config_hash = utils::get_content_hash(&contents);
            let cache_contents: Option<String> = std::fs::read_to_string(cache_path).ok();
            let (cache, did_create_cache) =
                cache::get_or_create_cache(cache_contents.as_deref(), &contents, &config_hash)?;
            let shell = cache.shell.clone();

            run(&cache, old_dir, new_dir)?;

            if did_create_cache {
                cache::write_cache(&cache, &home)?;
            }

            let old_local_config = match std::fs::read_to_string(&old_local_config_path) {
                Ok(contents) => Some(LocalConfig::from_str(&contents)?),
                Err(_) => None,
            };

            let new_local_config = match std::fs::read_to_string(&local_config_path) {
                Ok(contents) => Some(LocalConfig::from_str(&contents)?),
                Err(_) => None,
            };

            if old_local_config.is_some() || new_local_config.is_some() {
                run_local(old_local_config.as_ref(), new_local_config.as_ref(), &shell)?;
            }
        }
        cmd::Commands::Reload { shell } => {
            let config: Config = Config::from_config_file(&config_path)?;
            init_shell(Some(config), shell.unwrap())?;
        }
        cmd::Commands::Remove { shell } => remove_shell(shell.context("no shell passed")?)?,
    }

    Ok(())
}
