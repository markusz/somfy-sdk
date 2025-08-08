use reqwest::{Error as ReqwestError, StatusCode};

// Errors related to executing the HTTP request
#[derive(Debug, Clone, PartialEq)]
pub enum RequestError {
    AuthError,
    InvalidRequestError,
    UnknownError,
    NotFoundError,
    ServerError,
    InvalidBody,
}

impl From<ReqwestError> for RequestError {
    fn from(e: ReqwestError) -> Self {
        match e.status() {
            Some(code) if [StatusCode::FORBIDDEN, StatusCode::UNAUTHORIZED].contains(&code) => {
                RequestError::AuthError
            }
            Some(code) if [StatusCode::BAD_REQUEST].contains(&code) => {
                RequestError::InvalidRequestError
            }
            Some(code) if [StatusCode::NOT_FOUND].contains(&code) => RequestError::NotFoundError,
            Some(code) if code >= StatusCode::INTERNAL_SERVER_ERROR => RequestError::ServerError,
            None => RequestError::UnknownError,
            _ => RequestError::UnknownError,
        }
    }
}
