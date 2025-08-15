use crate::api_client::ApiResponse;
use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::ActionGroupExecution;
use crate::err::http::RequestError;
use reqwest::header::HeaderMap;
use reqwest::Body;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GetCurrentExecutionsCommand;

impl SomfyApiRequestCommand for GetCurrentExecutionsCommand {
    fn to_request(&self) -> RequestData {
        RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/exec/current".to_string(),
            method: HttpMethod::GET,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type GetCurrentExecutionsResponse = Vec<ActionGroupExecution>;

impl SomfyApiRequestResponse for GetCurrentExecutionsResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp: GetCurrentExecutionsResponse = serde_json::from_str(body)?;
        Ok(ApiResponse::GetCurrentExecutions(resp))
    }
}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"[
    {
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
    }
    ]"#;
    let parsed = GetCurrentExecutionsResponse::from_response_body(body)
        .expect("should parse valid body correctly");

    let ApiResponse::GetCurrentExecutions(payload) = parsed else {
        panic!("should have correct type")
    };

    assert_eq!(payload.len(), 1);
    assert_eq!(payload[0].id, "123");
}

#[test]
fn parse_empty_array_correctly() {
    let body = r#"[]"#;
    let parsed = GetCurrentExecutionsResponse::from_response_body(body)
        .expect("should parse empty array correctly");

    let ApiResponse::GetCurrentExecutions(payload) = parsed else {
        panic!("should have correct type")
    };

    assert_eq!(payload.len(), 0);
}

#[test]
fn generates_correct_request_path() {
    let command = GetCurrentExecutionsCommand;
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/exec/current"
    );
    assert_eq!(request_data.method, HttpMethod::GET);
}

#[test]
fn errs_for_invalid_body() {}
