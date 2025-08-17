use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use reqwest::header::HeaderMap;
use reqwest::Body;
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Debug, Clone, PartialEq)]
pub struct GetDevicesByControllableCommand {
    pub controllable_name: String,
}

impl SomfyApiRequestCommand for GetDevicesByControllableCommand {
    type Response = GetDevicesByControllableResponse;
    fn to_request(&self) -> RequestData {
        let encoded_controllable_name = encode(&self.controllable_name);
        RequestData {
            path: format!(
                "/enduser-mobile-web/1/enduserAPI/setup/devices/controllables/{encoded_controllable_name}"
            ),
            method: HttpMethod::GET,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type GetDevicesByControllableResponse = Vec<String>;

impl SomfyApiRequestResponse for GetDevicesByControllableResponse {}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"[
      "io://0000-1111-2222/12345678",
      "io://0000-1111-2222/87654321"
    ]"#;
    let resp = GetDevicesByControllableResponse::from_body(body)
        .expect("should parse valid body correctly");

    assert_eq!(resp.len(), 2);
    assert_eq!(resp[0], "io://0000-1111-2222/12345678");
    assert_eq!(resp[1], "io://0000-1111-2222/87654321");
}

#[test]
fn url_encoding_works_correctly() {
    let command = GetDevicesByControllableCommand {
        controllable_name: "io:StackComponent".to_string(),
    };
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/setup/devices/controllables/io%3AStackComponent"
    );
}

#[test]
fn errs_for_invalid_body() {}
