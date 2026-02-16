use std::fs;
use std::path::PathBuf;
use anyhow::{Ok};
use dirs;

// creates a config file for the cli if its the first time running
pub fn config_file() -> anyhow::Result<PathBuf> {
    let config_path = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
        .join("Qol")
        .join("Templates"); // config file will be saved at root/.config/Qol/Templates

    if !config_path.exists(){
        println!("Creating templates directory at {:?}", config_path);
        fs::create_dir_all(&config_path)?; // creates the config path 
    }

    Ok(config_path)
}
