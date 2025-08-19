use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::Gateway;
use crate::err::http::RequestError;
use reqwest::header::HeaderMap;
use reqwest::Body;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GetGatewaysCommand;

impl SomfyApiRequestCommand for GetGatewaysCommand {
    type Response = GetGatewaysResponse;
    fn to_request(&self) -> Result<RequestData, RequestError> {
        Ok(RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/setup/gateways".to_string(),
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
            method: HttpMethod::GET,
        })
    }
}

pub type GetGatewaysResponse = Vec<Gateway>;

impl SomfyApiRequestResponse for GetGatewaysResponse {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_request() {
        let command = GetGatewaysCommand;
        let request = command
            .to_request()
            .expect("should create valid request data");

        assert_eq!(
            request.path,
            "/enduser-mobile-web/1/enduserAPI/setup/gateways"
        );
        assert_eq!(request.method, HttpMethod::GET);
        assert!(request.query_params.is_empty());
        assert!(request.header_map.is_empty());
        assert!(request
            .body
            .as_bytes()
            .expect("should read body bytes")
            .is_empty());
    }

    #[test]
    fn test_from_body() {
        let body = r#"[
            {
                "gatewayId": "0000-1111-2222",
                "connectivity": {
                    "status": "OK",
                    "protocolVersion": "2022.1.3-1"
                }
            }
        ]"#;

        let response =
            GetGatewaysResponse::from_body(body).expect("should parse valid gateways response");
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].gateway_id, "0000-1111-2222");
        assert_eq!(response[0].connectivity.status, "OK");
    }
}

#[cfg(test)]
#[test]
fn parse_valid_body_correctly() {
    use crate::commands::types::GatewayConnectivity;
    let body = r#"[
	{
		"connectivity": {
			"status": "OK",
			"protocolVersion": "2025.3.2-7"
		},
		"gatewayId": "0000-1111-2222"
	}
    ]"#;
    let resp = GetGatewaysResponse::from_body(body).expect("should parse valid body correctly");

    assert_eq!(
        resp,
        vec![Gateway {
            gateway_id: "0000-1111-2222".to_string(),
            connectivity: GatewayConnectivity {
                status: "OK".to_string(),
                protocol_version: "2025.3.2-7".to_string()
            }
        }]
    )
}

#[test]
fn errs_for_invalid_body() {}
