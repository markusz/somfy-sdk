use reqwest::{Error as ReqwestError, StatusCode};
use serde_json::Error as SerdeError;

// Errors related to executing the HTTP request
#[derive(Debug, Clone, PartialEq)]
pub enum RequestError {
    AuthError,
    InvalidRequestError,
    UnknownError,
    NotFoundError,
    ServerError,
    InvalidBody,
    CertError,
}

impl From<SerdeError> for RequestError {
    fn from(_value: SerdeError) -> Self {
        RequestError::InvalidBody
    }
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
            None => {
                // Since certs are self-signed this will be a common source of error,
                // so we treat certificate errors explicitly.
                let anyhow_err: anyhow::Error = e.into();

                let is_cert_error = anyhow_err.chain().any(|err| {
                    let err_str = err.to_string();
                    println!("{err_str}");
                    err_str.contains("certificate")
                        || err_str.contains("tls")
                        || err_str.contains("ssl")
                });

                if is_cert_error {
                    RequestError::CertError
                } else {
                    RequestError::UnknownError
                }
            }
            _ => RequestError::UnknownError,
        }
    }
}
