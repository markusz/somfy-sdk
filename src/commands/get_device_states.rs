use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::DeviceState;
use reqwest::header::HeaderMap;
use reqwest::Body;
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceStatesCommand {
    pub device_url: String,
}

impl SomfyApiRequestCommand for GetDeviceStatesCommand {
    type Response = GetDeviceStatesResponse;
    fn to_request(&self) -> RequestData {
        let encoded_device_url = encode(&self.device_url);
        RequestData {
            path: format!(
                "/enduser-mobile-web/1/enduserAPI/setup/devices/{encoded_device_url}/states"
            ),
            method: HttpMethod::GET,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type GetDeviceStatesResponse = Vec<DeviceState>;

impl SomfyApiRequestResponse for GetDeviceStatesResponse {}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    use crate::commands::types::DeviceStateValue;
    let body = r#"[
        {
            "name": "core:StatusState",
            "type": 3,
            "value": "available"
        }
    ]"#;
    let resp = GetDeviceStatesResponse::from_body(body).expect("should parse valid body correctly");

    assert_eq!(resp.len(), 1);
    assert_eq!(resp[0].name, "core:StatusState");
    assert_eq!(resp[0].state_type, 3);
    if let DeviceStateValue::String(value) = &resp[0].value {
        assert_eq!(value, "available");
    } else {
        panic!("Expected string value");
    }
}

#[test]
fn url_encoding_works_correctly() {
    let command = GetDeviceStatesCommand {
        device_url: "io://0000-1111-2222/12345678".to_string(),
    };
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/setup/devices/io%3A%2F%2F0000-1111-2222%2F12345678/states"
    );
}

#[test]
fn errs_for_invalid_body() {}
