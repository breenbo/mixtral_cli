mod api;
use api::{AppError, ConfigFile, MistralAiApi};

mod cli;

fn main() -> Result<(), AppError> {
    //
    // set the api key
    //
    let api_key = ConfigFile::check_config()?;
    //
    // set model as cli argument, default to tiny
    //
    let ai_model = cli::get_ai_model();

    let mistral_ai = MistralAiApi::new(&api_key);
    //
    // prompt the user in a loop
    //
    loop {
        //
        let question = cli::get_question(&ai_model)?.trim().to_lowercase();
        //
        if cli::check_quit(&question) {
            break;
        }
        //
        // Send the question to Mistral
        //
        let response = mistral_ai.post_request(&question, &ai_model);
        //
        // Display the response
        //
        cli::display_response(response)?;
    }
    //
    Ok(())
}
