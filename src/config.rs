use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use log::info;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AppConfig {
    pub progress_bar_chars: String,
    pub progress_bar_style: String,
}

pub struct ConfigPath {
    #[allow(dead_code)]
    config_dir: PathBuf,
    config_file: PathBuf,
}

impl ConfigPath {
    pub fn new() -> Self {
        let dir = dirs::config_dir().unwrap().join("rgend");
        let file = dir.join("config.toml");

        ConfigPath {
            config_dir: dir,
            config_file: file,
        }
    }

    pub fn check(&self) {
        if !self.config_file.exists() {
            println!("Creating config file...");

            fs::create_dir_all(self.config_file.parent().unwrap())
                .expect("Failed to create directory");

            let default_config = AppConfig {
                progress_bar_chars: "#>-".to_string(),
                progress_bar_style: "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} | {binary_bytes_per_sec} | eta {eta}".to_string(), // replace with your default config values
            };

            let toml = toml::to_string(&default_config).expect("Failed to serialize user");

            info!("Writing default config file...");
            fs::write(&self.config_file, toml).expect("Failed to create config file");

            println!("Config file created at {:?}", self.config_file);
        }
    }

    pub fn read(&self) -> AppConfig {
        if !self.config_file.exists() {
            self.check();
        }

        info!("Reading config file...");
        let contents = fs::read_to_string(&self.config_file).expect("Failed to read config file");
        let parsed_config: AppConfig = toml::from_str(&contents).expect("Failed to deserialize config file");

        parsed_config
    }
}