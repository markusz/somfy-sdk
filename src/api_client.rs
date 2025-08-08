use crate::commands::get_version::{GetVersionCommand, GetVersionCommandResponse};
use crate::commands::traits::SomfyApiRequestResponse;
use crate::commands::traits::{RequestData, SomfyApiRequestCommand};
use crate::err::http::RequestError;

#[derive(Debug, Clone, PartialEq)]
pub struct ApiClientConfig {
    url: String,
    port: usize,
    api_key: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApiRequest {
    GetVersion(GetVersionCommand),
    // RegisterEventListener,
    // FetchEvents,
    // UnregisterEventListener
}

impl From<ApiRequest> for RequestData {
    fn from(value: ApiRequest) -> Self {
        match value {
            ApiRequest::GetVersion(c) => c.to_request(),
        }
    }
}

impl From<&ApiRequest> for RequestData {
    fn from(value: &ApiRequest) -> Self {
        match value {
            ApiRequest::GetVersion(c) => c.to_request(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApiResponse {
    GetVersion(GetVersionCommandResponse),
}
#[derive(Debug, Clone, PartialEq)]
pub struct ApiClient {
    config: ApiClientConfig,
}

impl ApiClient {
    pub fn new(config: ApiClientConfig) -> Self {
        ApiClient { config }
    }

    pub fn from(url: String, port: usize, api_key: String) -> Self {
        ApiClient {
            config: ApiClientConfig { url, port, api_key },
        }
    }

    pub async fn execute(&self, command: ApiRequest) -> Result<ApiResponse, RequestError> {
        let request_data: RequestData = (&command).into();
        let path = format!(
            "http://{}:{}{}",
            self.config.url, self.config.port, request_data.path
        );

        println!("{path:?}");
        let body = reqwest::get(path).await?.text().await?;

        Self::map_request_to_response(command, &body)
    }

    fn map_request_to_response(
        command: ApiRequest,
        body: &str,
    ) -> Result<ApiResponse, RequestError> {
        match command {
            ApiRequest::GetVersion(_) => GetVersionCommandResponse::from_response_body(body),
        }
    }

    pub async fn get_version(&self) -> Result<GetVersionCommandResponse, RequestError> {
        let command = ApiRequest::GetVersion(GetVersionCommand);
        let res = self.execute(command).await?;

        match res {
            ApiResponse::GetVersion(res) => Ok(res),
        }
    }
}

#[cfg(test)]
mod api_client_tests {
    use crate::api_client::{ApiClient, ApiClientConfig, ApiRequest};
    use crate::commands::get_version::{GetVersionCommand, GetVersionCommandResponse};
    use rstest::*;

    #[fixture]
    fn api_client() -> ApiClient {
        ApiClient::from("testurl.com".to_string(), 1000, "1234".to_string())
    }

    #[test]
    fn creates_api_client_with_new() {
        let api_client = ApiClient::new(ApiClientConfig {
            port: 1000,
            url: "testurl.com".to_string(),
            api_key: "1234".to_string(),
        });
        assert_eq!(api_client.config.port, 1000);
        assert_eq!(api_client.config.url, "testurl.com".to_string());
        assert_eq!(api_client.config.api_key, "1234".to_string());
    }

    #[test]
    fn creates_api_client_with_from() {
        let api_client = ApiClient::from("testurl.com".to_string(), 1000, "1234".to_string());
        assert_eq!(api_client.config.port, 1000);
        assert_eq!(api_client.config.url, "testurl.com".to_string());
        assert_eq!(api_client.config.api_key, "1234".to_string());
    }

    #[tokio::test]
    async fn responds_with_correct_type() {
        // Body parsing is tested only as a side_effect, refer to respective command struct for primary testing
        let valid_body = r#"{ "protocolVersion": "2022.1.3-1" }"#;
        let request = ApiRequest::GetVersion(GetVersionCommand);
        let response = ApiClient::map_request_to_response(request, valid_body)
            .expect("should return a ApiResponse::GetVersion");

        assert!(matches!(response, GetVersionCommandResponse))
    }
}
