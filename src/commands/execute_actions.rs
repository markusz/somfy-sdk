use crate::api_client::ApiResponse;
use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::{ActionGroup, ActionGroupExecutionId};
use crate::err::http::RequestError;
use reqwest::Body;
use reqwest::header::HeaderMap;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ExecuteActionsCommand {
    pub execute_request: ActionGroup,
}

impl SomfyApiRequestCommand for ExecuteActionsCommand {
    fn to_request(&self) -> RequestData {
        let body_json = serde_json::to_string(&self.execute_request).unwrap_or_default();

        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/exec/apply".to_string(),
            method: HttpMethod::POST,
            body: Body::from(body_json),
            query_params: HashMap::default(),
            header_map: headers,
        }
    }
}

pub type ExecuteActionsResponse = ActionGroupExecutionId;

impl SomfyApiRequestResponse for ExecuteActionsResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp: ExecuteActionsResponse = serde_json::from_str(body)?;
        Ok(ApiResponse::ExecuteActions(resp))
    }
}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"{
        "id": "exec-12345678-1234-5678-9012-123456789012"
    }"#;
    let parsed = ExecuteActionsResponse::from_response_body(body)
        .expect("should parse valid body correctly");

    let ApiResponse::ExecuteActions(payload) = parsed else {
        panic!("should have correct type")
    };

    assert_eq!(payload.id, "exec-12345678-1234-5678-9012-123456789012");
}

#[test]
fn generates_correct_request_path() {
    use crate::commands::types::{Action, ActionGroup, Command};

    let execute_request = ActionGroup {
        label: Some("Test execution".to_string()),
        actions: vec![Action {
            device_url: "io://0000-1111-2222/12345678".to_string(),
            commands: vec![Command {
                name: "open".to_string(),
                parameters: vec![],
            }],
        }],
    };

    let command = ExecuteActionsCommand { execute_request };
    let request_data = command.to_request();
    assert_eq!(
        request_data.path,
        "/enduser-mobile-web/1/enduserAPI/exec/apply"
    );
    assert_eq!(request_data.method, HttpMethod::POST);
}

#[test]
fn includes_json_content_type_header() {
    use crate::commands::types::{Action, ActionGroup, Command};

    let execute_request = ActionGroup {
        label: None,
        actions: vec![Action {
            device_url: "io://0000-1111-2222/12345678".to_string(),
            commands: vec![Command {
                name: "close".to_string(),
                parameters: vec![],
            }],
        }],
    };

    let command = ExecuteActionsCommand { execute_request };
    let request_data = command.to_request();

    let content_type = request_data.header_map.get("content-type");
    assert!(content_type.is_some());
    assert_eq!(content_type.unwrap().to_str().unwrap(), "application/json");
}

#[test]
fn errs_for_invalid_body() {}
