use crate::api_client::ApiResponse;
use crate::err::http::RequestError;
use reqwest::Body;
use reqwest::header::HeaderMap;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum HttpMethod {
    #[default]
    GET,
    POST,
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
    pub(crate) fn get_content_length(&self) -> String {
        self.body.as_bytes().unwrap_or_default().len().to_string()
    }
}

pub trait SomfyApiRequestCommand {
    fn to_request(&self) -> RequestData;
}

pub trait SomfyApiRequestResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError>;
}
