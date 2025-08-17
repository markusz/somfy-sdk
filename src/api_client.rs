use crate::commands::cancel_all_executions::{
    CancelAllExecutionsCommand, CancelAllExecutionsResponse,
};
use crate::commands::cancel_execution::{CancelExecutionCommand, CancelExecutionResponse};
use crate::commands::execute_action_group::{
    ExecuteActionGroupCommand, ExecuteActionGroupResponse,
};
use crate::commands::fetch_events::{FetchEventsCommand, FetchEventsResponse};
use crate::commands::get_current_executions::{
    GetCurrentExecutionsCommand, GetCurrentExecutionsResponse,
};
use crate::commands::get_device::{GetDeviceCommand, GetDeviceResponse};
use crate::commands::get_device_state::{GetDeviceStateCommand, GetDeviceStateResponse};
use crate::commands::get_device_states::{GetDeviceStatesCommand, GetDeviceStatesResponse};
use crate::commands::get_devices::{GetDevicesCommand, GetDevicesResponse};
use crate::commands::get_devices_by_controllable::{
    GetDevicesByControllableCommand, GetDevicesByControllableResponse,
};
use crate::commands::get_execution::{GetExecutionCommand, GetExecutionResponse};
use crate::commands::get_setup::{GetSetupCommand, GetSetupResponse};
use crate::commands::get_setup_gateways::{GetGatewaysCommand, GetGatewaysResponse};
use crate::commands::get_version::{GetVersionCommand, GetVersionResponse};
use crate::commands::register_event_listener::{
    RegisterEventListenerCommand, RegisterEventListenerResponse,
};
use crate::commands::traits::SomfyApiRequestResponse;
use crate::commands::traits::{HttpMethod, RequestData, SomfyApiRequestCommand};
use crate::commands::types::ActionGroup;
use crate::commands::unregister_event_listener::{
    UnregisterEventListenerCommand, UnregisterEventListenerResponse,
};
use crate::config::tls_cert::TlsCertHandler;
use crate::err::http::RequestError;
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Certificate, ClientBuilder, Response};

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

    pub async fn execute<C>(&self, command: C) -> Result<C::Response, RequestError>
    where
        C: SomfyApiRequestCommand,
    {
        let cert: Certificate = self.ensure_cert().await?;
        let headers = self.generate_default_headers()?;

        let client = ClientBuilder::new()
            .add_root_certificate(cert)
            .default_headers(headers)
            .build()?;

        let request_data: RequestData = command.to_request();
        let path = self.generate_base_url(&request_data);

        let response: Result<Response, reqwest::Error> = match request_data.method {
            HttpMethod::GET => client.get(&path).send().await?.error_for_status(),
            HttpMethod::POST => {
                let content_len = request_data.get_content_length();

                client
                    .post(&path)
                    .body(request_data.body)
                    .header("content-length", content_len)
                    .header("content-type", "application/json")
                    .send()
                    .await?
                    .error_for_status()
            }
            HttpMethod::DELETE => client.delete(&path).send().await?.error_for_status(),
        };

        let body = response?.text().await?;
        C::Response::from_body(body.as_str())
    }

    async fn ensure_cert(&self) -> Result<Certificate, RequestError> {
        Ok(match &self.config.cert_handling {
            CertificateHandling::CertProvided(path) => {
                let crt = std::fs::read(path).map_err(|_| RequestError::Cert)?;
                Certificate::from_pem(&crt)?
            }
            CertificateHandling::DefaultCert => TlsCertHandler::ensure_local_certificate()
                .await
                .map_err(|_| RequestError::Cert)?,
        })
    }

    fn generate_base_url(&self, request_data: &RequestData) -> String {
        let protocol = match self.config.protocol {
            HttpProtocol::HTTP => "http",
            HttpProtocol::HTTPS => "https",
        };

        let path = format!(
            "{}://{}:{}{}",
            protocol, self.config.url, self.config.port, request_data.path
        );
        path
    }

    fn generate_default_headers(&self) -> Result<HeaderMap, RequestError> {
        let mut headers = HeaderMap::new();
        let bearer_token =
            HeaderValue::from_str(format!("Bearer {}", self.config.api_key).as_str())
                .map_err(|e| RequestError::Server(e.into()))?;
        headers.insert(AUTHORIZATION, bearer_token);
        Ok(headers)
    }

    pub async fn get_version(&self) -> Result<GetVersionResponse, RequestError> {
        self.execute(GetVersionCommand).await
    }

    pub async fn get_gateways(&self) -> Result<GetGatewaysResponse, RequestError> {
        self.execute(GetGatewaysCommand).await
    }

    pub async fn get_devices(&self) -> Result<GetDevicesResponse, RequestError> {
        self.execute(GetDevicesCommand).await
    }

    pub async fn get_device(&self, device_url: &str) -> Result<GetDeviceResponse, RequestError> {
        self.execute(GetDeviceCommand { device_url }).await
    }

    pub async fn get_setup(&self) -> Result<GetSetupResponse, RequestError> {
        self.execute(GetSetupCommand).await
    }

    pub async fn get_device_states(
        &self,
        device_url: &str,
    ) -> Result<GetDeviceStatesResponse, RequestError> {
        self.execute(GetDeviceStatesCommand { device_url }).await
    }

    pub async fn get_device_state(
        &self,
        device_url: &str,
        state_name: &str,
    ) -> Result<GetDeviceStateResponse, RequestError> {
        self.execute(GetDeviceStateCommand {
            device_url,
            state_name,
        })
        .await
    }

    pub async fn get_devices_by_controllable(
        &self,
        controllable_name: &str,
    ) -> Result<GetDevicesByControllableResponse, RequestError> {
        self.execute(GetDevicesByControllableCommand { controllable_name })
            .await
    }

    pub async fn register_event_listener(
        &self,
    ) -> Result<RegisterEventListenerResponse, RequestError> {
        self.execute(RegisterEventListenerCommand).await
    }

    pub async fn fetch_events(
        &self,
        listener_id: &str,
    ) -> Result<FetchEventsResponse, RequestError> {
        self.execute(FetchEventsCommand { listener_id }).await
    }

    pub async fn unregister_event_listener(
        &self,
        listener_id: &str,
    ) -> Result<UnregisterEventListenerResponse, RequestError> {
        self.execute(UnregisterEventListenerCommand { listener_id })
            .await
    }

    pub async fn execute_actions(
        &self,
        action_group: ActionGroup,
    ) -> Result<ExecuteActionGroupResponse, RequestError> {
        self.execute(ExecuteActionGroupCommand { action_group })
            .await
    }

    pub async fn get_current_executions(
        &self,
    ) -> Result<GetCurrentExecutionsResponse, RequestError> {
        self.execute(GetCurrentExecutionsCommand).await
    }

    pub async fn get_execution(
        &self,
        execution_id: &str,
    ) -> Result<GetExecutionResponse, RequestError> {
        self.execute(GetExecutionCommand { execution_id }).await
    }

    pub async fn cancel_all_executions(&self) -> Result<CancelAllExecutionsResponse, RequestError> {
        self.execute(CancelAllExecutionsCommand).await
    }

    pub async fn cancel_execution(
        &self,
        execution_id: &str,
    ) -> Result<CancelExecutionResponse, RequestError> {
        self.execute(CancelExecutionCommand { execution_id }).await
    }
}

#[cfg(test)]
mod api_client_tests {
    use crate::api_client::{
        ApiClient, ApiClientConfig, CertificateHandling, HttpProtocol, DEFAULT_PORT,
    };
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
}
