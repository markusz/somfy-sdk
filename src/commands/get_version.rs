use crate::api_client::ApiResponse;
use crate::commands::traits::{RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse};
use crate::err::http::RequestError;
use reqwest::Body;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GetVersionCommand;
impl SomfyApiRequestCommand for GetVersionCommand {
    fn to_request(&self) -> RequestData {
        RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/apiVersion".to_string(),
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVersionCommandResponse {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
}

impl SomfyApiRequestResponse for GetVersionCommandResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp: GetVersionCommandResponse =
            serde_json::from_str(body).map_err(|_| RequestError::InvalidBody)?;
        Ok(ApiResponse::GetVersion(resp))
    }
}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"{ "protocolVersion": "2022.1.3-1" }"#;
    let parsed = GetVersionCommandResponse::from_response_body(body)
        .expect("should parse valid body correctly");

    let ApiResponse::GetVersion(payload) = parsed else {
        panic!("should have correct type")
    };
    assert_eq!(
        payload,
        GetVersionCommandResponse {
            protocol_version: "2022.1.3-1".to_string()
        }
    )
}

#[test]
fn errs_for_invalid_body() {
    let body = r#"{ "protVer": "2022.1.3-1" }"#;
    let parsed = GetVersionCommandResponse::from_response_body(body);
    assert!(parsed.is_err())
}
