use crate::api_client::ApiResponse;
use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::CancelAllExecutionsResult;
use crate::err::http::RequestError;
use reqwest::Body;
use reqwest::header::HeaderMap;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct CancelAllExecutionsCommand;

impl SomfyApiRequestCommand for CancelAllExecutionsCommand {
    fn to_request(&self) -> RequestData {
        RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/exec/current/setup".to_string(),
            method: HttpMethod::DELETE,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type CancelAllExecutionsResponse = CancelAllExecutionsResult;

impl SomfyApiRequestResponse for CancelAllExecutionsResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp: CancelAllExecutionsResponse = serde_json::from_str(body)?;
        Ok(ApiResponse::CancelAllExecutions(resp))
    }
}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"{}"#; // Empty object
    let parsed = CancelAllExecutionsResponse::from_response_body(body)
        .expect("should parse valid body correctly");

    let ApiResponse::CancelAllExecutions(_) = parsed else {
        panic!("should have correct type")
    };
}

#[test]
fn generates_correct_request_path() {
    let command = CancelAllExecutionsCommand;
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/exec/current/setup"
    );
    assert_eq!(request_data.method, HttpMethod::DELETE);
}

#[test]
fn errs_for_invalid_body() {}
