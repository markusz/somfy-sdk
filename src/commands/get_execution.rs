use crate::api_client::ApiResponse;
use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::ActionGroupExecution;
use crate::err::http::RequestError;
use reqwest::Body;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Debug, Clone, PartialEq)]
pub struct GetExecutionCommand {
    pub execution_id: String,
}

impl SomfyApiRequestCommand for GetExecutionCommand {
    fn to_request(&self) -> RequestData {
        let encoded_execution_id = encode(&self.execution_id);
        RequestData {
            path: format!("/enduser-mobile-web/1/enduserAPI/exec/current/{encoded_execution_id}"),
            method: HttpMethod::GET,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type GetExecutionResponse = ActionGroupExecution;

impl SomfyApiRequestResponse for GetExecutionResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp: GetExecutionResponse = serde_json::from_str(body)?;
        Ok(ApiResponse::GetExecution(resp))
    }
}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"{
    "owner": "string",
    "id": "123",
    "executionType": "Immediate execution",
    "executionSubType": "MANUAL_CONTROL",
    "description": "string",
    "startTime": 0,
    "actionGroup": {
      "label": "string",
      "actions": [
        {
          "commands": [
            {
              "type": 0,
              "name": "string",
              "parameters": [
                "string"
              ]
            }
          ],
          "deviceURL": "string"
        }
      ]
    },
    "state": "INITIALIZED"
  }"#;
    let parsed =
        GetExecutionResponse::from_response_body(body).expect("should parse valid body correctly");

    let ApiResponse::GetExecution(payload) = parsed else {
        panic!("should have correct type")
    };

    assert_eq!(payload.id, "123");
}

#[test]
fn generates_correct_request_path() {
    let command = GetExecutionCommand {
        execution_id: "exec-12345678-1234-5678-9012-123456789012".to_string(),
    };
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/exec/current/exec-12345678-1234-5678-9012-123456789012"
    );
    assert_eq!(request_data.method, HttpMethod::GET);
}

#[test]
fn url_encoding_works_correctly() {
    let command = GetExecutionCommand {
        execution_id: "test-execution-id-with-special-chars!@#".to_string(),
    };
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/exec/current/test-execution-id-with-special-chars%21%40%23"
    );
}

#[test]
fn errs_for_invalid_body() {}
