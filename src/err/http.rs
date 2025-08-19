use reqwest::{Error as ReqwestError, StatusCode};
use serde_json::Error as SerdeError;
use std::error::Error;
use std::option::Option;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("server error occurred: {0}")]
pub struct CustomServerError(pub String);

#[derive(Debug, Error)]
#[error("error in request / response mapping")]
pub struct RequestResponseMappingError;

impl From<RequestResponseMappingError> for RequestError {
    fn from(_value: RequestResponseMappingError) -> Self {
        RequestError::Server(
            CustomServerError("Error in request / response mapping".to_string()).into(),
        )
    }
}

impl From<CustomServerError> for RequestError {
    fn from(value: CustomServerError) -> Self {
        RequestError::Server(CustomServerError(value.0.to_string()).into())
    }
}

// Errors related to executing the HTTP request
#[derive(Debug, Error)]
pub enum RequestError {
    /// Network / protocol problems (DNS, TLS, timeouts, malformed HTTP…)
    #[error("transport error")]
    Transport(#[source] ReqwestError),

    /// Non-2xx from the server
    #[error("http status {status}")]
    Status {
        status: StatusCode,
        // keep the original for extra context if you want
        #[source]
        source: Option<reqwest::Error>,
    },

    /// Authentication/authorization problems
    #[error("authentication error: {message}")]
    Auth {
        message: String,
        status: Option<StatusCode>,
        #[source]
        source: Option<reqwest::Error>, // e.g., 401/403 response or JWT parse error’s source
    },

    /// Body could not be serialized/deserialized
    #[error("invalid body")]
    Body(#[source] serde_json::Error),

    /// Certificate problems (useful in your project)
    #[error("tls certificate rejected")]
    Cert,

    /// Any other server error
    #[error(transparent)]
    Server(#[from] anyhow::Error),
}

impl From<SerdeError> for RequestError {
    fn from(e: SerdeError) -> Self {
        RequestError::Body(e)
    }
}

impl From<ReqwestError> for RequestError {
    fn from(e: ReqwestError) -> Self {
        match e.status() {
            Some(code) if [StatusCode::FORBIDDEN, StatusCode::UNAUTHORIZED].contains(&code) => {
                RequestError::Auth {
                    message: "auth failed".to_string(),
                    status: Some(code),
                    source: Some(e),
                }
            }
            Some(other_code) => RequestError::Status {
                status: other_code,
                source: Some(e),
            },
            None => {
                // Since certs are self-signed this will be a common source of error,
                // so we treat certificate errors explicitly.

                let mut source = e.source();
                while let Some(e) = source {
                    let err_str = e.to_string().to_ascii_lowercase();
                    if err_str.contains("certificate")
                        || err_str.contains("tls")
                        || err_str.contains("ssl")
                    {
                        return RequestError::Cert;
                    }
                    source = e.source()
                }

                RequestError::Transport(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_serde_error() {
        let serde_error = serde_json::from_str::<i32>("invalid json").unwrap_err();
        let request_error = RequestError::from(serde_error);

        match request_error {
            RequestError::Body(_) => {}
            _ => panic!("Expected Body error"),
        }
    }

    #[test]
    fn test_from_custom_server_error() {
        let custom_error = CustomServerError("test error".to_string());
        let request_error = RequestError::from(custom_error);

        match request_error {
            RequestError::Server(_) => {}
            _ => panic!("Expected Server error"),
        }
    }

    #[test]
    fn test_from_request_response_mapping_error() {
        let mapping_error = RequestResponseMappingError;
        let request_error = RequestError::from(mapping_error);

        match request_error {
            RequestError::Server(_) => {}
            _ => panic!("Expected Server error"),
        }
    }

    #[test]
    fn test_error_display() {
        let custom_error = CustomServerError("test message".to_string());
        assert_eq!(
            custom_error.to_string(),
            "server error occurred: test message"
        );

        let mapping_error = RequestResponseMappingError;
        assert_eq!(
            mapping_error.to_string(),
            "error in request / response mapping"
        );
    }
}
