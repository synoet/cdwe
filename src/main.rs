mod config;
mod vars;
use anyhow::{anyhow, Result};
use clap::Parser;
use config::Config;

#[derive(Parser)]
struct Args {
    #[arg(long = "old_dir", required=true)]
    old_dir: String,
    #[arg(long = "new_dir", required=true)]
    new_dir: String,
}

fn main() -> Result<()> {
    let matches = Args::parse();
    #[allow(deprecated)]
    let home_dir = match std::env::home_dir() {
        Some(path) => path,
        None => return Err(anyhow!("Could not find home directory")),
    };

    let config =
        Config::from_config_file(&format!("{}/{}", home_dir.to_str().unwrap(), "cdwe.toml"))?;

    let to_set = vars::get_vars_to_set(&config, &matches.new_dir);
    let to_unset = vars::get_vars_to_unset(&config, &matches.old_dir);

    for var in to_set {
        println!("export {}=\"{}\"", var.key, var.value);
    }
    for var in to_unset {
        println!("unset {}", var);
    }

    Ok(())
}
