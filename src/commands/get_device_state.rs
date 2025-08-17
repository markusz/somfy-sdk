use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::DeviceState;
use reqwest::header::HeaderMap;
use reqwest::Body;
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceStateCommand<'a> {
    pub device_url: &'a str,
    pub state_name: &'a str,
}

impl SomfyApiRequestCommand for GetDeviceStateCommand<'_> {
    type Response = GetDeviceStateResponse;
    fn to_request(&self) -> RequestData {
        let encoded_device_url = encode(self.device_url);
        let encoded_state_name = encode(self.state_name);
        RequestData {
            path: format!(
                "/enduser-mobile-web/1/enduserAPI/setup/devices/{encoded_device_url}/states/{encoded_state_name}"
            ),
            method: HttpMethod::GET,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type GetDeviceStateResponse = DeviceState;

impl SomfyApiRequestResponse for GetDeviceStateResponse {}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    use crate::commands::types::DeviceStateValue;
    let body = r#"{
        "name": "core:StatusState",
        "type": 3,
        "value": "available"
    }"#;
    let resp = GetDeviceStateResponse::from_body(body).expect("should parse valid body correctly");

    assert_eq!(resp.name, "core:StatusState");
    assert_eq!(resp.state_type, 3);
    if let DeviceStateValue::String(value) = &resp.value {
        assert_eq!(value, "available");
    } else {
        panic!("Expected string value");
    }
}

#[test]
fn url_encoding_works_correctly() {
    let command = GetDeviceStateCommand {
        device_url: "io://0000-1111-2222/12345678",
        state_name: "core:StatusState",
    };
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/setup/devices/io%3A%2F%2F0000-1111-2222%2F12345678/states/core%3AStatusState"
    );
}

#[test]
fn errs_for_invalid_body() {}
