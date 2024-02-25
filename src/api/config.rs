use serde::{Deserialize, Serialize};

use crate::cli;

use super::AppError;

#[derive(Default, Deserialize, Serialize)]
pub struct ConfigFile {
    pub api_key: String,
    api_key_initialized: bool,
}

impl ConfigFile {
    fn new(api_key: String) -> Self {
        Self {
            api_key,
            api_key_initialized: true,
        }
    }

    fn load_config() -> Result<ConfigFile, AppError> {
        let config: ConfigFile = confy::load("mixtral_cli", "config")?;
        Ok(config)
    }

    fn set_api_key() -> Result<ConfigFile, AppError> {
        let api_key = cli::get_api_key()?;

        let config = Self::new(api_key);

        confy::store("mixtral_cli", "config", &config)?;

        Ok(config)
    }

    pub fn check_config() -> Result<String, AppError> {
        let reset_key = cli::get_reset_api();
        let mut config = Self::load_config()?;
        // 1. cli: get the reset from cli
        // 3. api: check if the api key is good by sending a request and get the response
        // 2. if not initialized or reset asked by user
        if !config.api_key_initialized || reset_key {
            config = Self::set_api_key()?;
        }

        Ok(config.api_key)
    }
}
