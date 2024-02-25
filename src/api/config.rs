use std::io::{self, Write};

use serde::{Deserialize, Serialize};

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
        println!("\nEnter your MixtralAi api key: ");
        io::stdout().flush()?;
        let mut key = String::new();
        io::stdin().read_line(&mut key)?;
        let api_key = String::from(key.trim_end());

        let config = Self::new(api_key);

        confy::store("mixtral_cli", "config", &config)?;

        Ok(config)
    }

    pub fn check_config(reset_key: bool) -> Result<String, AppError> {
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
