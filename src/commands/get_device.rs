use crate::api_client::ApiResponse;
use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::Device;
use crate::err::http::RequestError;
use reqwest::Body;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceCommand {
    pub device_url: String,
}

impl SomfyApiRequestCommand for GetDeviceCommand {
    fn to_request(&self) -> RequestData {
        let device_url = &self.device_url;
        let path = format!(
            "/enduser-mobile-web/1/enduserAPI/setup/devices/{}",
            encode(device_url)
        );
        RequestData {
            path: path.to_string(),
            method: HttpMethod::GET,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type GetDeviceResponse = Device;

impl SomfyApiRequestResponse for GetDeviceResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp = serde_json::from_str(body)?;

        Ok(ApiResponse::GetDevice(resp))
    }
}
