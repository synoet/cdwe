use super::super::config::Config;
use anyhow::Result;
use std::path::Path;

#[derive(Debug)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

// pub fn get_vars_to_set(config: &Config, new_path: &str) -> Vec<EnvVar> {
//     config
//         .directories
//         .iter()
//         .find(|dir| {
//             let path_to_check = Path::new(&dir.path);
//             let path = Path::new(new_path);
//             path.starts_with(path_to_check) || path_to_check == path
//         })
//         .unwrap_or(&EnvDirectory {
//             path: String::from(""),
//             vars: HashMap::new(),
//         })
//         .vars
//         .iter()
//         .map(|var| EnvVar {
//             key: var.0.clone(),
//             value: var.1.clone(),
//         })
//         .collect()
// }

pub fn get_vars_to_set(config: &Config, new_path: &str) -> Vec<EnvVar> {
    config
        .directories
        .iter()
        .filter(|dir| {
            let path_to_check = Path::new(&dir.path);
            let path = Path::new(new_path);
            path.starts_with(path_to_check) || path_to_check == path
        })
        .flat_map(|dir| {
            dir.vars
                .iter()
                .map(|var| EnvVar {
                    key: var.0.clone(),
                    value: var.1.clone(),
                })
                .collect::<Vec<EnvVar>>()
        })
        .collect()
}

pub fn get_vars_to_unset(config: &Config, old_path: &str) -> Vec<String> {
    get_vars_to_set(config, old_path)
        .iter()
        .map(|var| var.key.clone())
        .collect()
}

pub fn run(config: &Config, old_path: String, new_path: String) -> Result<(), anyhow::Error> {
    if old_path == new_path {
        return Ok(());
    }

    let to_set = get_vars_to_set(&config, &new_path);
    let to_unset = get_vars_to_unset(&config, &old_path);

    for var in to_unset {
        println!("unset {}", var);
    }

    for var in to_set {
        println!("export {}=\"{}\"", var.key, var.value);
    }

    Ok(())
}
