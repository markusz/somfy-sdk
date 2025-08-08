use crate::api_client::ApiResponse;
use crate::err::http::RequestError;
use reqwest::Body;
use reqwest::header::HeaderMap;
use std::collections::HashMap;

#[derive(Default)]
pub struct RequestData {
    pub header_map: HeaderMap,
    pub body: Body,
    pub query_params: HashMap<String, String>,
    pub path: String,
}

pub trait SomfyApiRequestCommand {
    fn to_request(&self) -> RequestData;
}

pub trait SomfyApiRequestResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError>;
}
