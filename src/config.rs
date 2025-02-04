use std::{fs, path::PathBuf};

use anyhow::Result;
use dirs::{config_dir, data_dir};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    /// The root directory where the transactions will be stored
    pub data_dir: PathBuf,
}

impl Config {
    pub fn load() -> Result<Config> {
        let dir = config_dir().expect("Config directory not found"); // todo: change it to proper error
                                                                     // handling
        let app_config_dir = dir.join("finman");
        let config_file_name = "config.toml";

        let config = if let Ok(config) = fs::read_to_string(app_config_dir.join(config_file_name)) {
            let config: Config = toml::from_str(&config)?;
            config
        } else {
            let os_data_dir = data_dir().expect("should have a data dir");
            let app_data_dir = os_data_dir.join("finman");
            Config {
                data_dir: app_data_dir,
            }
        };

        Ok(config)
    }
}
