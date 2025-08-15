use crate::api_client::ApiResponse;
use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::Device;
use crate::err::http::RequestError;
use reqwest::header::HeaderMap;
use reqwest::Body;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GetDevicesCommand;

impl SomfyApiRequestCommand for GetDevicesCommand {
    fn to_request(&self) -> RequestData {
        RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/setup/devices".to_string(),
            method: HttpMethod::GET,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type GetDevicesResponse = Vec<Device>;

impl SomfyApiRequestResponse for GetDevicesResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp = serde_json::from_str(body)?;

        Ok(ApiResponse::GetDevices(resp))
    }
}
