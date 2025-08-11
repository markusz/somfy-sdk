use crate::api_client::ApiResponse;
use crate::commands::traits::{RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse};
use crate::commands::types::DeviceState;
use crate::err::http::RequestError;
use reqwest::Body;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceStateCommand {
    pub device_url: String,
    pub state_name: String,
}

impl SomfyApiRequestCommand for GetDeviceStateCommand {
    fn to_request(&self) -> RequestData {
        let encoded_device_url = encode(&self.device_url);
        let encoded_state_name = encode(&self.state_name);
        RequestData {
            path: format!(
                "/enduser-mobile-web/1/enduserAPI/setup/devices/{encoded_device_url}/states/{encoded_state_name}"
            ),
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type GetDeviceStateResponse = DeviceState;

impl SomfyApiRequestResponse for GetDeviceStateResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp: GetDeviceStateResponse = serde_json::from_str(body)?;
        Ok(ApiResponse::GetDeviceState(resp))
    }
}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    use crate::commands::types::DeviceStateValue;
    let body = r#"{
        "name": "core:StatusState",
        "type": 3,
        "value": "available"
    }"#;
    let parsed = GetDeviceStateResponse::from_response_body(body)
        .expect("should parse valid body correctly");

    let ApiResponse::GetDeviceState(payload) = parsed else {
        panic!("should have correct type")
    };

    assert_eq!(payload.name, "core:StatusState");
    assert_eq!(payload.state_type, 3);
    if let DeviceStateValue::String(value) = &payload.value {
        assert_eq!(value, "available");
    } else {
        panic!("Expected string value");
    }
}

#[test]
fn url_encoding_works_correctly() {
    let command = GetDeviceStateCommand {
        device_url: "io://0000-1111-2222/12345678".to_string(),
        state_name: "core:StatusState".to_string(),
    };
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/setup/devices/io%3A%2F%2F0000-1111-2222%2F12345678/states/core%3AStatusState"
    );
}

#[test]
fn errs_for_invalid_body() {}
