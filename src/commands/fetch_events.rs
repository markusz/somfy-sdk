use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::Event;
use reqwest::header::HeaderMap;
use reqwest::Body;
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Debug, Clone, PartialEq)]
pub struct FetchEventsCommand {
    pub listener_id: String,
}

impl SomfyApiRequestCommand for FetchEventsCommand {
    type Response = FetchEventsResponse;
    fn to_request(&self) -> RequestData {
        let encoded_listener_id = encode(&self.listener_id);
        RequestData {
            path: format!("/enduser-mobile-web/1/enduserAPI/events/{encoded_listener_id}/fetch"),
            method: HttpMethod::POST,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type FetchEventsResponse = Vec<Event>;

impl SomfyApiRequestResponse for FetchEventsResponse {}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"[
      {
        "name": "ExecutionStateChangedEvent"
      },
      {
        "name": "DeviceProtocolUnavailableEvent",
        "protocolType": 0
      }
    ]"#; // Events can be empty array
    let resp = FetchEventsResponse::from_body(body).expect("should parse valid body correctly");

    assert_eq!(resp.len(), 2);
}

#[test]
fn generates_correct_request_path() {
    let command = FetchEventsCommand {
        listener_id: "12345678-1234-5678-9012-123456789012".to_string(),
    };
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/events/12345678-1234-5678-9012-123456789012/fetch"
    );
}

#[test]
fn url_encoding_works_correctly() {
    let command = FetchEventsCommand {
        listener_id: "test-id-with-special-chars!@#".to_string(),
    };
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/events/test-id-with-special-chars%21%40%23/fetch"
    );
}

#[test]
fn errs_for_invalid_body() {}
