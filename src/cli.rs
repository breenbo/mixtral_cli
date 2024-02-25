use std::io::{self, Write};

use clap::Parser;
use serde_json::Value;

use crate::api::{AppError, MixtralAiApi};

#[derive(Parser, Debug)]
struct Args {
    #[arg(
        long,
        short = 'm',
        help = "Choose AI model: tiny, small, medium",
        default_value = "tiny"
    )]
    model: String,
    #[arg(long, short = 'r', help = "Ask to reset the api key", num_args = 0)]
    reset: bool,
}
// able to choose modele to use, default to tiny
pub fn get_ai_model() -> String {
    let args = Args::parse();
    //
    // check the ai model. If not in list, default to tiny
    //
    let ai_model = match args.model.as_str() {
        "small" => "mistral-small",
        "medium" => "mistral-medium",
        _ => "mistral-tiny",
    };
    //

    ai_model.to_string()
}

pub fn get_reset_api() -> bool {
    let args = Args::parse();

    args.reset
}

fn display_header(ai_model: &str) {
    print!("\n\n -----------------------------------");
    print!("\n  --- Mixtral AI - {} ---", ai_model);
    print!("\n -----------------------------------");
    print!("\nEnter your question: ");
}

pub fn get_question(ai_model: &str) -> Result<String, AppError> {
    display_header(ai_model);
    //
    io::stdout().flush()?;
    let mut question = String::new();
    io::stdin().read_line(&mut question)?;

    Ok(question)
}

pub fn check_quit(question: &str) -> bool {
    ["quit", "exit", "q", "ex", ""].contains(&question)
}

pub fn get_api_key() -> Result<String, AppError> {
    println!("\nEnter your MixtralAi api key: ");
    io::stdout().flush()?;
    let mut key = String::new();
    io::stdin().read_line(&mut key)?;
    let api_key = String::from(key.trim_end());

    Ok(api_key)
}

pub fn display_response(result: Result<Value, AppError>) -> Result<(), AppError> {
    match result {
        Ok(data) => {
            // Extract the answer from the JSON object
            let answer = data["choices"][0]["message"]["content"].as_str().ok_or(
                MixtralAiApi::new_api_error(500, "Could not find answer field in JSON response"),
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

    Ok(())
}
