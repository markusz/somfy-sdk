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
