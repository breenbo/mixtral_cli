use dotenv::dotenv;
use std::{
    env,
    io::{self, Write},
};

mod api;
use api::{AppError, MixtralAiApi};
mod cli;
use cli::get_ai_model;

fn main() -> Result<(), AppError> {
    // set the api key
    dotenv()?;
    let api_key = match env::var("MIXTRAL_API_KEY") {
        Ok(val) => val,
        Err(_) => panic!("API key not set"),
    };
    //
    //
    // define url endpoint
    let endpoint = "https://api.mistral.ai/v1/chat/completions";
    // set model as cli argument, default to tiny
    let ai_model = get_ai_model();

    let mixtral_ai = MixtralAiApi::new(endpoint.to_string(), api_key);
    //
    // prompt the user in a loop
    loop {
        print!("\n -----------------------------------");
        print!("\n  --- Mixtral AI - {} ---", ai_model);
        print!("\n -----------------------------------");
        print!("\nEnter your question: ");
        io::stdout().flush()?;
        let mut question = String::new();
        io::stdin().read_line(&mut question)?;
        //
        // check if user wants to quit (break the loop)
        //
        let test_string = question.trim().to_lowercase();
        if [
            String::from("quit"),
            String::from("exit"),
            String::from("q"),
            String::from("ex"),
        ]
        .contains(&test_string)
        {
            break;
        }
        //
        // Send the question to Mixtral
        //
        let result = mixtral_ai.post_request(&question, &ai_model);

        match result {
            Ok(data) => {
                // Extract the answer from the JSON object
                let answer = data["choices"][0]["message"]["content"].as_str().ok_or(
                    MixtralAiApi::new_api_error(
                        500,
                        "Could not find answer field in JSON response",
                    ),
                )?;

                print!("\n {}", answer);
            }
            Err(e @ AppError::Api(..)) | Err(e @ AppError::Reqwest(..)) => {
                // Print out API errors or connection errors
                eprintln!("Error: {}", e);
            }
            Err(e) => {
                // Propagate any other errors upwards
                return Err(e);
            }
        }
    }
    //
    Ok(())
}
