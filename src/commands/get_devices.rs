use crate::api_client::ApiResponse;
use crate::commands::traits::{RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse};
use crate::commands::types::Device;
use crate::err::http::RequestError;
use log::error;
use reqwest::Body;
use reqwest::header::HeaderMap;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GetDevicesCommand;

impl SomfyApiRequestCommand for GetDevicesCommand {
    fn to_request(&self) -> RequestData {
        RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/setup/devices".to_string(),
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type GetDevicesResponse = Vec<Device>;

impl SomfyApiRequestResponse for GetDevicesResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp = serde_json::from_str(body);

        if let Err(e) = resp {
            error!("{e:?}");
            Err(RequestError::InvalidBody)
        } else {
            Ok(ApiResponse::GetDevices(resp.unwrap()))
        }
    }
}
