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
