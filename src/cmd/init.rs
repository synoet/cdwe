use anyhow::Result;
use clap::ValueEnum;

#[derive(Debug, ValueEnum, Clone)]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
}

impl Shell {
    fn get_config_path(&self) -> String {
        let home = std::env::home_dir().unwrap();
        match self {
            Shell::Bash => std::path::Path::join(home.as_path(), ".bashrc")
                .to_str()
                .unwrap()
                .to_string(),
            Shell::Fish => std::path::Path::join(home.as_path(), "/config/fish/config.fish")
                .to_str()
                .unwrap()
                .to_string(),
            Shell::Zsh => std::path::Path::join(home.as_path(), ".zshrc")
                .to_str()
                .unwrap()
                .to_string(),
        }
    }

    fn get_shell_script(&self) -> String {
        match self {
            Shell::Bash => include_str!("../../shells/cdwe_bash.txt").to_string(),
            Shell::Fish => include_str!("../../shells/cdwe_fish.txt").to_string(),
            Shell::Zsh => include_str!("../../shells/cdwe_zsh.txt").to_string(),
        }
    }

    fn get_shell_script_target(&self) -> String {
        let home = std::env::home_dir().unwrap();
        match self {
            Shell::Bash => std::path::Path::join(home.as_path(), ".cdwe.sh")
                .to_str()
                .unwrap()
                .to_string(),
            Shell::Fish => std::path::Path::join(home.as_path(), ".cdwe.fish")
                .to_str()
                .unwrap()
                .to_string(),
            Shell::Zsh => std::path::Path::join(home.as_path(), ".cdwe.zsh")
                .to_str()
                .unwrap()
                .to_string(),
        }
    }
}

pub fn init_shell(shell: Shell) -> Result<()> {
    let config_path = shell.get_config_path();
    let mut shell_script = shell.get_shell_script();

    let exe_path = std::env::current_exe().unwrap();
    shell_script = shell_script.replace("{{{exec_path}}}", exe_path.to_str().unwrap());

    let shell_script_target = shell.get_shell_script_target();
    std::fs::write(&shell_script_target, shell_script).unwrap();

    let source_string = format!("source {}", &shell_script_target);

    let mut config = std::fs::read_to_string(&config_path).unwrap();
    if !config.contains(&source_string) {
        config.push_str(&format!("\n{}", source_string));
        std::fs::write(&config_path, config).unwrap();
    }
    Ok(())
}
