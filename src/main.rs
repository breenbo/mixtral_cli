use dotenv::dotenv;
use std::env;

mod api;
use api::{AppError, ConfigFile, MixtralAiApi};

mod cli;

fn main() -> Result<(), AppError> {
    // set the api key
    dotenv()?;
    let api_key = match env::var("MIXTRAL_API_KEY") {
        Ok(val) => val,
        Err(_) => panic!("API key not set"),
    };
    //
    // ConfigFile::set_api_key(false)?;
    //
    let config = ConfigFile::load_config()?;
    // 1. cli: get the reset from cli
    let reset_api = cli::get_reset_api();
    // 3. api: check if the api key is good by sending a request and get the response
    // 2. if not initialized or reset asked by user
    if !config.api_key_initialized || reset_api {
        dbg!(config);
        ConfigFile::set_api_key()?;
    }

    //
    //
    // define url endpoint
    const ENDPOINT: &str = "https://api.mistral.ai/v1/chat/completions";
    // set model as cli argument, default to tiny
    let ai_model = cli::get_ai_model();

    let mixtral_ai = MixtralAiApi::new(ENDPOINT, &api_key);
    //
    // prompt the user in a loop
    //
    loop {
        //
        //
        let question = cli::get_question(&ai_model)?.trim().to_lowercase();
        //
        if cli::check_quit(&question) {
            break;
        }
        //
        // Send the question to Mixtral
        //
        let result = mixtral_ai.post_request(&question, &ai_model);
        //
        // Display the response
        //
        cli::display_response(result)?;
    }
    //
    Ok(())
}
