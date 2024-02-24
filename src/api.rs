use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fmt, io};
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
    model: &'a str,
    messages: Vec<QuestionMessage<'a>>,
}
//
//
pub struct MixtralAiApi {
    base_url: String,
    token: String,
}
impl MixtralAiApi {
    pub fn new(base_url: String, token: String) -> Self {
        MixtralAiApi { base_url, token }
    }

    pub fn new_api_error(code: i64, msg: &'static str) -> MixtralAiApiError {
        MixtralAiApiError { code, msg }
    }

    fn prepare_request<'a>(&'a self, question: &'a str, model: &'a str) -> Request<'_> {
        let message = QuestionMessage {
            role: "user",
            content: question,
        };
        Request {
            model,
            // model: AiModel::MistralTiny.to_string(),
            messages: vec![message],
        }
    }

    pub fn post_request(&self, question: &str, model: &str) -> Result<Value, AppError> {
        let request = self.prepare_request(question, model);
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
pub struct MixtralAiApiError {
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
pub enum AppError {
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
        match self {
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Json(err) => write!(f, "JSON error: {}", err),
            AppError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            AppError::Api(err) => write!(f, "Mixtral API error: {}", err),
            AppError::Dotenv(err) => write!(f, "Dotenv error: {}", err),
        }
    }
}
