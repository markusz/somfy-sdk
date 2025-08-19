use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::err::http::RequestError;
use reqwest::header::HeaderMap;
use reqwest::Body;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GetVersionCommand;
impl SomfyApiRequestCommand for GetVersionCommand {
    type Response = GetVersionResponse;
    fn to_request(&self) -> Result<RequestData, RequestError> {
        Ok(RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/apiVersion".to_string(),
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
            method: HttpMethod::GET,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVersionResponse {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
}

impl SomfyApiRequestResponse for GetVersionResponse {}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"{ "protocolVersion": "2022.1.3-1" }"#;
    let resp = GetVersionResponse::from_body(body).expect("should parse valid body correctly");

    assert_eq!(
        resp,
        GetVersionResponse {
            protocol_version: "2022.1.3-1".to_string()
        }
    )
}

#[test]
fn errs_for_invalid_body() {
    let body = r#"{ "protVer": "2022.1.3-1" }"#;
    let parsed = GetVersionResponse::from_body(body);
    assert!(parsed.is_err())
}
