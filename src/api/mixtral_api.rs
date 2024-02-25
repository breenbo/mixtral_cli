use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{manage_errors::MixtralAiApiError, AppError};
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
pub struct MixtralAiApi<'a> {
    base_url: &'a str,
    token: &'a str,
}
//
impl<'a> MixtralAiApi<'_> {
    //
    // define url endpoint
    //
    const ENDPOINT: &'a str = "https://api.mistral.ai/v1/chat/completions";
    //
    //
    pub fn new(token: &str) -> MixtralAiApi {
        MixtralAiApi {
            base_url: Self::ENDPOINT,
            token,
        }
    }

    pub fn new_api_error(code: i64, msg: &'static str) -> MixtralAiApiError {
        MixtralAiApiError { code, msg }
    }

    fn prepare_request<'b>(&'b self, question: &'b str, model: &'b str) -> Request<'_> {
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
            .post(self.base_url)
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
