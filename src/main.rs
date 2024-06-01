mod cache;
mod cmd;
mod config;
mod utils;
use anyhow::{Context, Result};
use clap::Parser;
use cmd::{init_shell, remove_shell, run, Cli};
use config::Config;

fn main() -> Result<()> {
    let start_time = std::time::Instant::now(); 
    let matches = Cli::parse();
    let home = std::env::var("HOME").context("no $HOME set")?;
    let config_path = format!("{}/{}", &home, "cdwe.toml");
    let cache_path = format!("{}/{}", &home, ".cdwe_cache.json");

    match matches.command {
        cmd::Commands::Init { shell } => init_shell(None, shell.unwrap())?,
        cmd::Commands::Run { old_dir, new_dir } => {
            let contents = std::fs::read_to_string(&config_path)
                .with_context(|| format!("Could not read config file at {}", &config_path))?;
            let config = Config::from_str(&contents).context("failed to parse config")?;
            let config_hash = utils::get_content_hash(&contents);
            let cache_contents: Option<String> = std::fs::read_to_string(&cache_path).ok();
            let (cache, did_create_cache) =
                cache::get_or_create_cache(cache_contents.as_deref(), &config, &config_hash)?;

            run(&config, &cache, old_dir, new_dir)?;

            if did_create_cache {
                cache::write_cache(&cache, &home)?;
            }
        }
        cmd::Commands::Reload { shell } => {
            let config: Config = Config::from_config_file(&config_path)?;
            init_shell(Some(config), shell.unwrap())?;
        }
        cmd::Commands::Remove { shell } => remove_shell(shell.context("no shell passed")?)?,
    }
    let end_time = std::time::Instant::now();

    println!("command took {:?}", end_time - start_time);
    Ok(())
}
