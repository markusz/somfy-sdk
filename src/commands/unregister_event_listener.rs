use crate::api_client::ApiResponse;
use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::err::http::RequestError;
use reqwest::header::HeaderMap;
use reqwest::Body;
use serde_json::Value;
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Debug, Clone, PartialEq)]
pub struct UnregisterEventListenerCommand {
    pub listener_id: String,
}

impl SomfyApiRequestCommand for UnregisterEventListenerCommand {
    fn to_request(&self) -> RequestData {
        let encoded_listener_id = encode(&self.listener_id);
        RequestData {
            path: format!(
                "/enduser-mobile-web/1/enduserAPI/events/{encoded_listener_id}/unregister"
            ),
            method: HttpMethod::POST,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type UnregisterEventListenerResponse = Vec<Value>; // Empty array response

impl SomfyApiRequestResponse for UnregisterEventListenerResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp: UnregisterEventListenerResponse = serde_json::from_str(body)?;
        Ok(ApiResponse::UnregisterEventListener(resp))
    }
}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"[]"#;
    let parsed = UnregisterEventListenerResponse::from_response_body(body)
        .expect("should parse valid body correctly");

    let ApiResponse::UnregisterEventListener(payload) = parsed else {
        panic!("should have correct type")
    };

    assert_eq!(payload.len(), 0);
}

#[test]
fn generates_correct_request_path() {
    let command = UnregisterEventListenerCommand {
        listener_id: "12345678-1234-5678-9012-123456789012".to_string(),
    };
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/events/12345678-1234-5678-9012-123456789012/unregister"
    );
}

#[test]
fn url_encoding_works_correctly() {
    let command = UnregisterEventListenerCommand {
        listener_id: "test-id-with-special-chars!@#".to_string(),
    };
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/events/test-id-with-special-chars%21%40%23/unregister"
    );
}

#[test]
fn errs_for_invalid_body() {}
