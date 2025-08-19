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
    type Response = GetCurrentExecutionsResponse;
    fn to_request(&self) -> Result<RequestData, RequestError> {
        Ok(RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/exec/current".to_string(),
            method: HttpMethod::GET,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        })
    }
}

pub type GetCurrentExecutionsResponse = Vec<ActionGroupExecution>;

impl SomfyApiRequestResponse for GetCurrentExecutionsResponse {}

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
    let resp =
        GetCurrentExecutionsResponse::from_body(body).expect("should parse valid body correctly");

    assert_eq!(resp.len(), 1);
    assert_eq!(resp[0].id, "123");
}

#[test]
fn parse_empty_array_correctly() {
    let body = r#"[]"#;
    let resp =
        GetCurrentExecutionsResponse::from_body(body).expect("should parse empty array correctly");

    assert_eq!(resp.len(), 0);
}

#[test]
fn generates_correct_request_path() {
    let command = GetCurrentExecutionsCommand;
    let request_data = command.to_request().expect("should not err");
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/exec/current"
    );
    assert_eq!(request_data.method, HttpMethod::GET);
}

#[test]
fn errs_for_invalid_body() {}
