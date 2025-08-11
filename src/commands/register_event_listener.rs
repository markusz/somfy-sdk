use crate::api_client::ApiResponse;
use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::EventListener;
use crate::err::http::RequestError;
use reqwest::Body;
use reqwest::header::HeaderMap;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct RegisterEventListenerCommand;

impl SomfyApiRequestCommand for RegisterEventListenerCommand {
    fn to_request(&self) -> RequestData {
        RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/events/register".to_string(),
            method: HttpMethod::POST,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type RegisterEventListenerResponse = EventListener;

impl SomfyApiRequestResponse for RegisterEventListenerResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp: RegisterEventListenerResponse = serde_json::from_str(body)?;
        Ok(ApiResponse::RegisterEventListener(resp))
    }
}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"{
        "id": "12345678-1234-5678-9012-123456789012"
    }"#;
    let parsed = RegisterEventListenerResponse::from_response_body(body)
        .expect("should parse valid body correctly");

    let ApiResponse::RegisterEventListener(payload) = parsed else {
        panic!("should have correct type")
    };

    assert_eq!(payload.id, "12345678-1234-5678-9012-123456789012");
}

#[test]
fn generates_correct_request_path() {
    let command = RegisterEventListenerCommand;
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/events/register"
    );
}

#[test]
fn errs_for_invalid_body() {}
