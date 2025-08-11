use crate::api_client::ApiResponse;
use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::err::http::RequestError;
use reqwest::Body;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Debug, Clone, PartialEq)]
pub struct GetDevicesByControllableCommand {
    pub controllable_name: String,
}

impl SomfyApiRequestCommand for GetDevicesByControllableCommand {
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

impl SomfyApiRequestResponse for GetDevicesByControllableResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp: GetDevicesByControllableResponse = serde_json::from_str(body)?;
        Ok(ApiResponse::GetDevicesByControllable(resp))
    }
}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"[
      "io://0000-1111-2222/12345678",
      "io://0000-1111-2222/87654321"
    ]"#;
    let parsed = GetDevicesByControllableResponse::from_response_body(body)
        .expect("should parse valid body correctly");

    let ApiResponse::GetDevicesByControllable(payload) = parsed else {
        panic!("should have correct type")
    };

    assert_eq!(payload.len(), 2);
    assert_eq!(payload[0], "io://0000-1111-2222/12345678");
    assert_eq!(payload[1], "io://0000-1111-2222/87654321");
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
