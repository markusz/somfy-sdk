use crate::commands::get_version::{GetVersionCommand, GetVersionCommandResponse};
use crate::commands::traits::SomfyApiRequestResponse;
use crate::commands::traits::{RequestData, SomfyApiRequestCommand};
use crate::config::tls_cert::TlsCertHandler;
use crate::err::http::RequestError;
use log::debug;
use reqwest::{Certificate, ClientBuilder};

#[derive(Debug, Clone, PartialEq)]
pub enum HttpProtocol {
    HTTP,
    HTTPS,
}
#[derive(Debug, Clone, PartialEq)]
pub enum CertificateHandling {
    CertProvided(String),
    DefaultCert,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApiClientConfig {
    pub cert_handling: CertificateHandling,
    pub protocol: HttpProtocol,
    pub url: String,
    pub port: usize,
    pub api_key: String,
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

const DEFAULT_PORT: usize = 8443;

impl ApiClient {
    pub fn new(config: ApiClientConfig) -> Self {
        debug!("Initialized ApiClient with Config: {config:?}");
        ApiClient { config }
    }

    pub fn from(id: &str, api_key: &str) -> Self {
        Self::new(ApiClientConfig {
            url: format!("gateway-{id}.local"),
            port: DEFAULT_PORT,
            api_key: api_key.to_string(),
            protocol: HttpProtocol::HTTPS,
            cert_handling: CertificateHandling::DefaultCert,
        })
    }

    pub async fn execute(&self, command: ApiRequest) -> Result<ApiResponse, RequestError> {
        let request_data: RequestData = (&command).into();
        let protocol = match self.config.protocol {
            HttpProtocol::HTTP => "http",
            HttpProtocol::HTTPS => "https",
        };

        let path = format!(
            "{}://{}:{}{}",
            protocol, self.config.url, self.config.port, request_data.path
        );
        let mut client_builder = ClientBuilder::new();

        let cert: Certificate = match &self.config.cert_handling {
            CertificateHandling::CertProvided(path) => {
                let crt = std::fs::read(path).map_err(|_| RequestError::CertError)?;
                Certificate::from_pem(&crt)?
            }
            CertificateHandling::DefaultCert => TlsCertHandler::ensure_local_certificate()
                .await
                .map_err(|_| RequestError::CertError)?,
        };



        client_builder = client_builder.add_root_certificate(cert);

        let client = client_builder.build()?;
        let body = client.get(path).send().await?.text().await?;

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
    use crate::api_client::{
        ApiClient, ApiClientConfig, ApiRequest, ApiResponse, CertificateHandling, DEFAULT_PORT,
        HttpProtocol,
    };
    use crate::commands::get_version::GetVersionCommand;
    use rstest::*;

    #[fixture]
    fn api_client() -> ApiClient {
        ApiClient::from("0000-1111-2222", "my_key")
    }

    #[test]
    fn creates_api_client_with_new() {
        let api_client = ApiClient::new(ApiClientConfig {
            protocol: HttpProtocol::HTTP,
            port: 2000,
            url: "somedomain.com".to_string(),
            api_key: "my_key".to_string(),
            cert_handling: CertificateHandling::DefaultCert,
        });
        assert_eq!(api_client.config.protocol, HttpProtocol::HTTP);
        assert_eq!(api_client.config.port, 2000);
        assert_eq!(api_client.config.url, "somedomain.com".to_string());
        assert_eq!(api_client.config.api_key, "my_key".to_string());
        assert_eq!(
            api_client.config.cert_handling,
            CertificateHandling::DefaultCert
        );
    }

    #[test]
    fn creates_api_client_with_from() {
        let api_client = ApiClient::from("0000-1111-2222", "my_key");
        assert_eq!(api_client.config.port, DEFAULT_PORT);
        assert_eq!(
            api_client.config.url,
            "gateway-0000-1111-2222.local".to_string()
        );
        assert_eq!(
            api_client.config.cert_handling,
            CertificateHandling::DefaultCert
        );
        assert_eq!(api_client.config.protocol, HttpProtocol::HTTPS);
        assert_eq!(api_client.config.api_key, "my_key".to_string());
    }

    #[tokio::test]
    async fn responds_with_correct_type() {
        // Body parsing is tested only as a side_effect, refer to respective command struct for primary testing
        let valid_body = r#"{ "protocolVersion": "2022.1.3-1" }"#;
        let request = ApiRequest::GetVersion(GetVersionCommand);
        let response = ApiClient::map_request_to_response(request, valid_body)
            .expect("should return a ApiResponse::GetVersion");

        // let is_right_type = (&response as &dyn Any).is::<ApiResponse::GetVersion(GetVersionCommandResponse)>();
        // assert!(is_right_type)
        assert!(matches!(response, ApiResponse::GetVersion(_)))
    }
}
