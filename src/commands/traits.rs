use crate::err::http::RequestError;
use reqwest::header::HeaderMap;
use reqwest::Body;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum HttpMethod {
    #[default]
    GET,
    POST,
    DELETE,
}

#[derive(Default)]
pub struct RequestData {
    pub header_map: HeaderMap,
    pub body: Body,
    pub query_params: HashMap<String, String>,
    pub path: String,
    pub method: HttpMethod,
}

impl RequestData {
    pub fn get_content_length(&self) -> String {
        self.body.as_bytes().unwrap_or_default().len().to_string()
    }

    pub fn default_post_headers() -> Result<HeaderMap, RequestError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "content-type",
            reqwest::header::HeaderValue::from_str("application/json")
                .map_err(|e| RequestError::Server(e.into()))?,
        );

        Ok(headers)
    }
}

pub trait SomfyApiRequestCommand {
    type Response: SomfyApiRequestResponse;
    fn to_request(&self) -> Result<RequestData, RequestError>;
}

pub trait SomfyApiRequestResponse: DeserializeOwned {
    fn from_body(body: &str) -> Result<Self, RequestError>
    where
        Self: DeserializeOwned,
    {
        Ok(serde_json::from_str(body)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_data_defaults() {
        let request = RequestData::default();

        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.path, "");
        assert!(request.query_params.is_empty());
        assert!(request.header_map.is_empty());
        assert!(request
            .body
            .as_bytes()
            .expect("should read body bytes")
            .is_empty());
    }

    #[test]
    fn test_http_method_default() {
        let method = HttpMethod::default();
        assert_eq!(method, HttpMethod::GET);
    }

    #[test]
    fn test_get_content_length_empty_body() {
        let request = RequestData::default();
        assert_eq!(request.get_content_length(), "0");
    }

    #[test]
    fn test_get_content_length_with_body() {
        let request = RequestData {
            body: reqwest::Body::from("test body"),
            ..Default::default()
        };
        assert_eq!(request.get_content_length(), "9");
    }

    #[test]
    fn test_default_post_headers() {
        let headers =
            RequestData::default_post_headers().expect("should create default post headers");
        assert_eq!(headers.get("content-type").unwrap(), "application/json");
    }
}
