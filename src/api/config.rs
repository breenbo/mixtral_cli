use std::io::{self, Write};

use serde::{Deserialize, Serialize};

use super::AppError;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct ConfigFile {
    api_key: String,
    pub api_key_initialized: bool,
}

impl ConfigFile {
    fn new(api_key: String) -> Self {
        Self {
            api_key,
            api_key_initialized: true,
        }
    }

    pub fn load_config() -> Result<ConfigFile, AppError> {
        let config: ConfigFile = confy::load("mixtral_cli", "config")?;
        Ok(config)
    }

    pub fn set_api_key() -> Result<(), AppError> {
        println!("Enter your MixtralAi api key: ");
        io::stdout().flush()?;
        let mut key = String::new();
        io::stdin().read_line(&mut key)?;
        let api_key = String::from(key.trim_end());

        let config = ConfigFile::new(api_key);
        println!("{:#?}", config);

        confy::store("mixtral_cli", "config", config)?;

        Ok(())
    }
}
