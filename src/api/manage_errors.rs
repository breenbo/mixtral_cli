use std::{fmt, io};

//
// manage errors for all sources
// Here, we define a custom error type called AppError, which wraps various other error types such as IO errors, JSON parsing errors, network errors, and API errors. We implement the From trait to convert these underlying errors into instances of AppError. We also define a helper function new_api_error within the MixtralAiApi struct to construct a new instance of MixtralAiApiError. Finally, we implement the From trait to convert MixtralAiApiError into AppError.
//
//
#[derive(Debug)]
pub struct MistralAiApiError {
    pub code: i64,
    pub msg: &'static str,
}
impl fmt::Display for MistralAiApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Mistral AI error {}: {}", self.code, self.msg)
    }
}
//
//
//
#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Json(serde_json::Error),
    Reqwest(reqwest::Error),
    Api(MistralAiApiError),
    Confy(confy::ConfyError),
    Arboard(arboard::Error),
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
impl From<MistralAiApiError> for AppError {
    fn from(err: MistralAiApiError) -> Self {
        AppError::Api(err)
    }
}
impl From<confy::ConfyError> for AppError {
    fn from(err: confy::ConfyError) -> Self {
        AppError::Confy(err)
    }
}
impl From<arboard::Error> for AppError {
    fn from(err: arboard::Error) -> Self {
        AppError::Arboard(err)
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Json(err) => write!(f, "JSON error: {}", err),
            AppError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            AppError::Api(err) => write!(f, "Mixtral API error: {}", err),
            AppError::Confy(err) => write!(f, "Confy error: {}", err),
            AppError::Arboard(err) => write!(f, "Confy error: {}", err),
        }
    }
}
