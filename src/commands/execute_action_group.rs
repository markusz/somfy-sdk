use crate::commands::traits::SomfyApiRequestResponse;
use crate::commands::types::ActionGroupExecutionId;

#[cfg(feature = "generic-exec")]
#[derive(Debug, Clone, PartialEq)]
pub struct ExecuteActionGroupCommand<'a> {
    pub action_group: &'a crate::commands::types::ActionGroup,
}

#[cfg(feature = "generic-exec")]
impl crate::commands::traits::SomfyApiRequestCommand for ExecuteActionGroupCommand<'_> {
    type Response = ExecuteActionGroupResponse;
    fn to_request(
        &self,
    ) -> Result<crate::commands::traits::RequestData, crate::err::http::RequestError> {
        let body_json = serde_json::to_string(&self.action_group)?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "content-type",
            reqwest::header::HeaderValue::from_str("application/json")
                .map_err(|e| crate::err::http::RequestError::Server(e.into()))?,
        );

        Ok(crate::commands::traits::RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/exec/apply".to_string(),
            method: crate::commands::traits::HttpMethod::POST,
            body: reqwest::Body::from(body_json),
            query_params: std::collections::HashMap::default(),
            header_map: headers,
        })
    }
}

pub type ExecuteActionGroupResponse = ActionGroupExecutionId;

impl SomfyApiRequestResponse for ExecuteActionGroupResponse {}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "generic-exec")]
    #[test]
    fn test_to_request() {
        use crate::commands::traits::SomfyApiRequestCommand;
        use crate::commands::types::{Action, ActionGroup};

        let action_group = ActionGroup {
            label: Some("Test Action".to_string()),
            actions: vec![Action {
                device_url: "io://test".to_string(),
                commands: vec![],
            }],
        };

        let command = ExecuteActionGroupCommand {
            action_group: &action_group,
        };
        let request = command
            .to_request()
            .expect("should create valid request data");

        assert_eq!(request.path, "/enduser-mobile-web/1/enduserAPI/exec/apply");
        assert_eq!(request.method, crate::commands::traits::HttpMethod::POST);
        assert!(request.query_params.is_empty());
        assert_eq!(
            request.header_map.get("content-type").unwrap(),
            "application/json"
        );
        assert!(!request
            .body
            .as_bytes()
            .expect("should read body bytes")
            .is_empty());
    }

    #[test]
    fn parse_valid_body_correctly() {
        let body = r#"{
        "execId": "exec-12345678-1234-5678-9012-123456789012"
    }"#;
        let resp =
            ExecuteActionGroupResponse::from_body(body).expect("should parse valid body correctly");

        assert_eq!(resp.exec_id, "exec-12345678-1234-5678-9012-123456789012");
    }
}

#[cfg(feature = "generic-exec")]
#[cfg(test)]
mod execute_action_group {
    use crate::commands::traits::{HttpMethod, SomfyApiRequestCommand};

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

        let command = crate::commands::execute_action_group::ExecuteActionGroupCommand {
            action_group: &execute_request,
        };
        let request_data = command.to_request().expect("should not err");
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

        let command = crate::commands::execute_action_group::ExecuteActionGroupCommand {
            action_group: &execute_request,
        };
        let request_data = command.to_request().expect("should not err");

        let content_type = request_data.header_map.get("content-type");
        assert_eq!(
            content_type
                .expect("should be Some")
                .to_str()
                .expect("should unwrap"),
            "application/json"
        );
    }
}
