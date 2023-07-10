use std::io::{Read, Write};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;

use anyhow::{anyhow, Context, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::get_path()?;

        if !path.exists() {
            return Err(anyhow!("The config file does not exists"));
        }

        let mut file = File::open(path).context("Unable to open the config file")?;
        let mut config = String::new();
        file.read_to_string(&mut config)
            .context("Unable to read the config file")?;

        let config: Config = toml::from_str(&config).context("Unable to parse the config file")?;

        Ok(config)
    }

    pub fn create(token: &str) -> Result<Self> {
        let dir_path = Self::get_dir_path()?;
        fs::create_dir_all(dir_path).context("Unable to create the directories")?;

        let config_path = Self::get_path()?;

        let mut file = File::create(config_path).context("Unable to create the file")?;

        let config = Self {
            token: token.to_owned(),
        };

        let toml_config = toml::to_string(&config).context("Unable to parse the config")?;

        file.write_all(toml_config.as_bytes())
            .context("Unable to write the config file")?;

        Ok(config)
    }

    fn get_path() -> Result<PathBuf> {
        let config_dir = Self::get_dir_path()?;
        let config_path = format!("{}/config.toml", config_dir.display());
        Ok(Path::new(&config_path).to_owned())
    }

    fn get_dir_path() -> Result<PathBuf> {
        let config_dir = ProjectDirs::from("dev", "mageas", "simple_login_purge")
            .context("Unable to determine the config path")?;
        Ok(config_dir.config_dir().to_owned())
    }
}
