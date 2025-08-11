use crate::api_client::ApiResponse;
use crate::commands::traits::{RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse};
use crate::commands::types::Setup;
use crate::err::http::RequestError;
use reqwest::Body;
use reqwest::header::HeaderMap;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GetSetupCommand;

impl SomfyApiRequestCommand for GetSetupCommand {
    fn to_request(&self) -> RequestData {
        RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/setup".to_string(),
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type GetSetupResponse = Setup;

impl SomfyApiRequestResponse for GetSetupResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp: GetSetupResponse = serde_json::from_str(body)?;
        Ok(ApiResponse::GetSetup(resp))
    }
}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    let body = r#"{
        "gateways": [
            {
                "connectivity": {
                    "status": "OK",
                    "protocolVersion": "2025.3.2-7"
                },
                "gatewayId": "0000-1111-2222"
            }
        ],
        "devices": [
            {
                "deviceURL": "io://0000-1111-2222/12345678",
                "label": "Test Device",
                "controllableName": "io:StackComponent",
                "subsystemId": 1,
                "type": 1,
                "available": true,
                "synced": true,
                "enabled": true,
                "states": [],
                "attributes": []
            }
        ]
    }"#;
    let parsed =
        GetSetupResponse::from_response_body(body).expect("should parse valid body correctly");

    let ApiResponse::GetSetup(payload) = parsed else {
        panic!("should have correct type")
    };

    assert_eq!(payload.gateways.len(), 1);
    assert_eq!(payload.devices.len(), 1);
    assert_eq!(payload.gateways[0].gateway_id, "0000-1111-2222");
    assert_eq!(payload.devices[0].label, "Test Device");
}

#[test]
fn errs_for_invalid_body() {}
