use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::Setup;
use crate::err::http::RequestError;
use reqwest::header::HeaderMap;
use reqwest::Body;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GetSetupCommand;

impl SomfyApiRequestCommand for GetSetupCommand {
    type Response = GetSetupResponse;
    fn to_request(&self) -> Result<RequestData, RequestError> {
        Ok(RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/setup".to_string(),
            method: HttpMethod::GET,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        })
    }
}

pub type GetSetupResponse = Setup;

impl SomfyApiRequestResponse for GetSetupResponse {}

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
    let resp = GetSetupResponse::from_body(body).expect("should parse valid body correctly");

    assert_eq!(resp.gateways.len(), 1);
    assert_eq!(resp.devices.len(), 1);
    assert_eq!(resp.gateways[0].gateway_id, "0000-1111-2222");
    assert_eq!(resp.devices[0].label, "Test Device");
}

#[test]
fn errs_for_invalid_body() {}
