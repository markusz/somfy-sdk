use crate::api_client::ApiResponse;
use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::CancelExecutionResult;
use crate::err::http::RequestError;
use reqwest::Body;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Debug, Clone, PartialEq)]
pub struct CancelExecutionCommand {
    pub execution_id: String,
}

impl SomfyApiRequestCommand for CancelExecutionCommand {
    fn to_request(&self) -> RequestData {
        let encoded_execution_id = encode(&self.execution_id);
        RequestData {
            path: format!(
                "/enduser-mobile-web/1/enduserAPI/exec/current/setup/{encoded_execution_id}"
            ),
            method: HttpMethod::DELETE,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type CancelExecutionResponse = CancelExecutionResult;

impl SomfyApiRequestResponse for CancelExecutionResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp: CancelExecutionResponse = serde_json::from_str(body)?;
        Ok(ApiResponse::CancelExecution(resp))
    }
}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"{}"#; // Empty object
    let parsed = CancelExecutionResponse::from_response_body(body)
        .expect("should parse valid body correctly");

    let ApiResponse::CancelExecution(_) = parsed else {
        panic!("should have correct type")
    };
}

#[test]
fn generates_correct_request_path() {
    let command = CancelExecutionCommand {
        execution_id: "exec-12345678-1234-5678-9012-123456789012".to_string(),
    };
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/exec/current/setup/exec-12345678-1234-5678-9012-123456789012"
    );
    assert_eq!(request_data.method, HttpMethod::DELETE);
}

#[test]
fn url_encoding_works_correctly() {
    let command = CancelExecutionCommand {
        execution_id: "test-execution-id-with-special-chars!@#".to_string(),
    };
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/exec/current/setup/test-execution-id-with-special-chars%21%40%23"
    );
}

#[test]
fn errs_for_invalid_body() {}
