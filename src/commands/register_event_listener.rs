use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::EventListener;
use reqwest::header::HeaderMap;
use reqwest::Body;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct RegisterEventListenerCommand;

impl SomfyApiRequestCommand for RegisterEventListenerCommand {
    type Response = RegisterEventListenerResponse;
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

impl SomfyApiRequestResponse for RegisterEventListenerResponse {}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"{
        "id": "12345678-1234-5678-9012-123456789012"
    }"#;
    let resp =
        RegisterEventListenerResponse::from_body(body).expect("should parse valid body correctly");

    assert_eq!(resp.id, "12345678-1234-5678-9012-123456789012");
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
