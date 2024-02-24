use dotenv::dotenv;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    env, fmt,
    io::{self, Write},
};

//
// Mixtral AI struct
//
#[derive(Debug, Serialize, Deserialize)]
struct QuestionMessage<'a> {
    role: &'a str,
    content: &'a str,
}
#[derive(Debug, Serialize)]
struct Request<'a> {
    model: String,
    messages: Vec<QuestionMessage<'a>>,
}
//
//
struct MixtralAiApi {
    base_url: String,
    token: String,
}
impl MixtralAiApi {
    fn new(base_url: String, token: String) -> Self {
        MixtralAiApi { base_url, token }
    }

    fn new_api_error(code: i64, msg: &'static str) -> MixtralAiApiError {
        MixtralAiApiError { code, msg }
    }

    fn prepare_request<'a>(&'a self, question: &'a str) -> Request<'_> {
        let message = QuestionMessage {
            role: "user",
            content: question,
        };
        let model = String::from("mistral-tiny");
        Request {
            model,
            // model: AiModel::MistralTiny.to_string(),
            messages: vec![message],
        }
    }

    fn post_request(&self, question: &str) -> Result<Value, AppError> {
        let request = self.prepare_request(question);
        //
        // create client to make http request
        //
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        //
        // make post request
        //
        let response = client
            .post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&request)
            .send()?
            .text()?;
        // Parse the JSON response
        let data: Value = serde_json::from_str(&response)?;
        //
        Ok(data)
    }
}
//
//
#[derive(Debug)]
struct MixtralAiApiError {
    code: i64,
    msg: &'static str,
}
impl fmt::Display for MixtralAiApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Mixtral AI error {}: {}", self.code, self.msg)
    }
}

//
// manage errors for all sources
// Here, we define a custom error type called AppError, which wraps various other error types such as IO errors, JSON parsing errors, network errors, and API errors. We implement the From trait to convert these underlying errors into instances of AppError. We also define a helper function new_api_error within the MixtralAiApi struct to construct a new instance of MixtralAiApiError. Finally, we implement the From trait to convert MixtralAiApiError into AppError.
//
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Json(serde_json::Error),
    Reqwest(reqwest::Error),
    Api(MixtralAiApiError),
    Dotenv(dotenv::Error),
}
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Json(err)
    }
}
impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Reqwest(err)
    }
}
impl From<MixtralAiApiError> for AppError {
    fn from(err: MixtralAiApiError) -> Self {
        AppError::Api(err)
    }
}
impl From<dotenv::Error> for AppError {
    fn from(err: dotenv::Error) -> Self {
        AppError::Dotenv(err)
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "App error")
    }
}

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
    let mixtral_ai = MixtralAiApi::new(endpoint.to_string(), api_key);
    //
    // prompt the user in a loop
    loop {
        print!("\n\nEnter your question ('quit' to exit): ");
        io::stdout().flush()?;
        let mut question = String::new();
        io::stdin().read_line(&mut question)?;
        //
        // check if user wants to quit (break the loop)
        //
        if question.trim().to_lowercase() == "quit" {
            break;
        }
        //
        // Send the question to Mixtral
        //
        let result = mixtral_ai.post_request(&question);

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
