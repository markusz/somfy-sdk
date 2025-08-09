use crate::api_client::ApiResponse;
use crate::commands::traits::{RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse};
use crate::commands::types::Gateway;
use crate::err::http::RequestError;
use reqwest::Body;
use reqwest::header::HeaderMap;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GetGatewaysCommand;

impl SomfyApiRequestCommand for GetGatewaysCommand {
    fn to_request(&self) -> RequestData {
        RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/setup/gateways".to_string(),
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type GetGatewaysResponse = Vec<Gateway>;

impl SomfyApiRequestResponse for GetGatewaysResponse {
    fn from_response_body(body: &str) -> Result<ApiResponse, RequestError> {
        let resp: GetGatewaysResponse =
            serde_json::from_str(body).map_err(|_| RequestError::InvalidBody)?;
        Ok(ApiResponse::GetGateways(resp))
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
		"gatewayId": "0812-2424-9999"
	}
    ]"#;
    let parsed =
        GetGatewaysResponse::from_response_body(body).expect("should parse valid body correctly");

    let ApiResponse::GetGateways(payload) = parsed else {
        panic!("should have correct type")
    };
    assert_eq!(
        payload,
        vec![Gateway {
            gateway_id: "0812-2424-9999".to_string(),
            connectivity: GatewayConnectivity {
                status: "OK".to_string(),
                protocol_version: "2025.3.2-7".to_string()
            }
        }]
    )
}

#[test]
fn errs_for_invalid_body() {}
