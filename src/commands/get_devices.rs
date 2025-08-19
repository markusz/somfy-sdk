use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::Device;
use crate::err::http::RequestError;
use reqwest::header::HeaderMap;
use reqwest::Body;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GetDevicesCommand;

impl SomfyApiRequestCommand for GetDevicesCommand {
    type Response = GetDevicesResponse;
    fn to_request(&self) -> Result<RequestData, RequestError> {
        Ok(RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/setup/devices".to_string(),
            method: HttpMethod::GET,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        })
    }
}

pub type GetDevicesResponse = Vec<Device>;

impl SomfyApiRequestResponse for GetDevicesResponse {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_request() {
        let command = GetDevicesCommand;
        let request = command
            .to_request()
            .expect("should create valid request data");

        assert_eq!(
            request.path,
            "/enduser-mobile-web/1/enduserAPI/setup/devices"
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
                "controllableName": "io:ExteriorVenetianBlindIOComponent",
                "deviceURL": "io://0812-2424-9999/246132",
                "label": "Test Device",
                "available": true,
                "enabled": true,
                "synced": true,
                "subsystemId": 0,
                "states": [],
                "attributes": [],
                "type": 1
            }
        ]"#;

        let response =
            GetDevicesResponse::from_body(body).expect("should parse valid devices response");
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].device_url, "io://0812-2424-9999/246132");
        assert_eq!(
            response[0].controllable_name,
            "io:ExteriorVenetianBlindIOComponent"
        );
    }
}
