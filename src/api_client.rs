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
use crate::commands::get_version::{GetVersionCommand, GetVersionCommandResponse};
use crate::commands::register_event_listener::{
    RegisterEventListenerCommand, RegisterEventListenerResponse,
};
use crate::commands::traits::SomfyApiRequestResponse;
use crate::commands::traits::{HttpMethod, RequestData, SomfyApiRequestCommand};
use crate::commands::unregister_event_listener::{
    UnregisterEventListenerCommand, UnregisterEventListenerResponse,
};
use crate::config::tls_cert::TlsCertHandler;
use crate::err::http::{RequestError, RequestResponseMappingError};
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Certificate, ClientBuilder, Response, header};

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
    GetGateways(GetGatewaysCommand),
    GetDevices(GetDevicesCommand),
    GetDevice(GetDeviceCommand),
    GetSetup(GetSetupCommand),
    GetDeviceStates(GetDeviceStatesCommand),
    GetDeviceState(GetDeviceStateCommand),
    GetDevicesByControllable(GetDevicesByControllableCommand),
    RegisterEventListener(RegisterEventListenerCommand),
    FetchEvents(FetchEventsCommand),
    UnregisterEventListener(UnregisterEventListenerCommand),
    ExecuteActions(ExecuteActionGroupCommand),
    GetCurrentExecutions(GetCurrentExecutionsCommand),
    GetExecution(GetExecutionCommand),
    CancelAllExecutions(CancelAllExecutionsCommand),
    CancelExecution(CancelExecutionCommand),
}

impl From<ApiRequest> for RequestData {
    fn from(value: ApiRequest) -> Self {
        match value {
            ApiRequest::GetVersion(c) => c.to_request(),
            ApiRequest::GetGateways(c) => c.to_request(),
            ApiRequest::GetDevices(c) => c.to_request(),
            ApiRequest::GetDevice(c) => c.to_request(),
            ApiRequest::GetSetup(c) => c.to_request(),
            ApiRequest::GetDeviceStates(c) => c.to_request(),
            ApiRequest::GetDeviceState(c) => c.to_request(),
            ApiRequest::GetDevicesByControllable(c) => c.to_request(),
            ApiRequest::RegisterEventListener(c) => c.to_request(),
            ApiRequest::FetchEvents(c) => c.to_request(),
            ApiRequest::UnregisterEventListener(c) => c.to_request(),
            ApiRequest::ExecuteActions(c) => c.to_request(),
            ApiRequest::GetCurrentExecutions(c) => c.to_request(),
            ApiRequest::GetExecution(c) => c.to_request(),
            ApiRequest::CancelAllExecutions(c) => c.to_request(),
            ApiRequest::CancelExecution(c) => c.to_request(),
        }
    }
}

impl From<&ApiRequest> for RequestData {
    fn from(value: &ApiRequest) -> Self {
        match value {
            ApiRequest::GetVersion(c) => c.to_request(),
            ApiRequest::GetGateways(c) => c.to_request(),
            ApiRequest::GetDevices(c) => c.to_request(),
            ApiRequest::GetDevice(c) => c.to_request(),
            ApiRequest::GetSetup(c) => c.to_request(),
            ApiRequest::GetDeviceStates(c) => c.to_request(),
            ApiRequest::GetDeviceState(c) => c.to_request(),
            ApiRequest::GetDevicesByControllable(c) => c.to_request(),
            ApiRequest::RegisterEventListener(c) => c.to_request(),
            ApiRequest::FetchEvents(c) => c.to_request(),
            ApiRequest::UnregisterEventListener(c) => c.to_request(),
            ApiRequest::ExecuteActions(c) => c.to_request(),
            ApiRequest::GetCurrentExecutions(c) => c.to_request(),
            ApiRequest::GetExecution(c) => c.to_request(),
            ApiRequest::CancelAllExecutions(c) => c.to_request(),
            ApiRequest::CancelExecution(c) => c.to_request(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApiResponse {
    GetVersion(GetVersionCommandResponse),
    GetGateways(GetGatewaysResponse),
    GetDevices(GetDevicesResponse),
    GetDevice(GetDeviceResponse),
    GetSetup(GetSetupResponse),
    GetDeviceStates(GetDeviceStatesResponse),
    GetDeviceState(GetDeviceStateResponse),
    GetDevicesByControllable(GetDevicesByControllableResponse),
    RegisterEventListener(RegisterEventListenerResponse),
    FetchEvents(FetchEventsResponse),
    UnregisterEventListener(UnregisterEventListenerResponse),
    ExecuteActions(ExecuteActionGroupResponse),
    GetCurrentExecutions(GetCurrentExecutionsResponse),
    GetExecution(GetExecutionResponse),
    CancelAllExecutions(CancelAllExecutionsResponse),
    CancelExecution(CancelExecutionResponse),
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
        let cert: Certificate = self.ensure_cert().await?;
        let headers = self.generate_default_headers()?;

        let client = ClientBuilder::new()
            .add_root_certificate(cert)
            .default_headers(headers)
            .build()?;

        let request_data: RequestData = (&command).into();
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
        Self::map_request_to_response(command, &body)
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
        headers.insert(header::AUTHORIZATION, bearer_token);
        Ok(headers)
    }

    fn map_request_to_response(
        command: ApiRequest,
        body: &str,
    ) -> Result<ApiResponse, RequestError> {
        match command {
            ApiRequest::GetVersion(_) => GetVersionCommandResponse::from_response_body(body),
            ApiRequest::GetGateways(_) => GetGatewaysResponse::from_response_body(body),
            ApiRequest::GetDevices(_) => GetDevicesResponse::from_response_body(body),
            ApiRequest::GetDevice(_) => GetDeviceResponse::from_response_body(body),
            ApiRequest::GetSetup(_) => GetSetupResponse::from_response_body(body),
            ApiRequest::GetDeviceStates(_) => GetDeviceStatesResponse::from_response_body(body),
            ApiRequest::GetDeviceState(_) => GetDeviceStateResponse::from_response_body(body),
            ApiRequest::GetDevicesByControllable(_) => {
                GetDevicesByControllableResponse::from_response_body(body)
            }
            ApiRequest::RegisterEventListener(_) => {
                RegisterEventListenerResponse::from_response_body(body)
            }
            ApiRequest::FetchEvents(_) => FetchEventsResponse::from_response_body(body),
            ApiRequest::UnregisterEventListener(_) => {
                UnregisterEventListenerResponse::from_response_body(body)
            }
            ApiRequest::ExecuteActions(_) => ExecuteActionGroupResponse::from_response_body(body),
            ApiRequest::GetCurrentExecutions(_) => {
                GetCurrentExecutionsResponse::from_response_body(body)
            }
            ApiRequest::GetExecution(_) => GetExecutionResponse::from_response_body(body),
            ApiRequest::CancelAllExecutions(_) => {
                CancelAllExecutionsResponse::from_response_body(body)
            }
            ApiRequest::CancelExecution(_) => CancelExecutionResponse::from_response_body(body),
        }
    }

    pub async fn get_version(&self) -> Result<GetVersionCommandResponse, RequestError> {
        let command = ApiRequest::GetVersion(GetVersionCommand);
        let res = self.execute(command).await?;

        match res {
            ApiResponse::GetVersion(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn get_gateways(&self) -> Result<GetGatewaysResponse, RequestError> {
        let command = ApiRequest::GetGateways(GetGatewaysCommand);
        let res = self.execute(command).await?;

        match res {
            ApiResponse::GetGateways(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn get_devices(&self) -> Result<GetDevicesResponse, RequestError> {
        let command = ApiRequest::GetDevices(GetDevicesCommand);
        let res = self.execute(command).await?;

        match res {
            ApiResponse::GetDevices(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn get_device(&self, device_url: &str) -> Result<GetDeviceResponse, RequestError> {
        let command = ApiRequest::GetDevice(GetDeviceCommand {
            device_url: device_url.to_string(),
        });
        let res = self.execute(command).await?;

        match res {
            ApiResponse::GetDevice(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn get_setup(&self) -> Result<GetSetupResponse, RequestError> {
        let command = ApiRequest::GetSetup(GetSetupCommand);
        let res = self.execute(command).await?;

        match res {
            ApiResponse::GetSetup(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn get_device_states(
        &self,
        device_url: &str,
    ) -> Result<GetDeviceStatesResponse, RequestError> {
        let command = ApiRequest::GetDeviceStates(GetDeviceStatesCommand {
            device_url: device_url.to_string(),
        });
        let res = self.execute(command).await?;

        match res {
            ApiResponse::GetDeviceStates(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn get_device_state(
        &self,
        device_url: &str,
        state_name: &str,
    ) -> Result<GetDeviceStateResponse, RequestError> {
        let command = ApiRequest::GetDeviceState(GetDeviceStateCommand {
            device_url: device_url.to_string(),
            state_name: state_name.to_string(),
        });
        let res = self.execute(command).await?;

        match res {
            ApiResponse::GetDeviceState(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn get_devices_by_controllable(
        &self,
        controllable_name: &str,
    ) -> Result<GetDevicesByControllableResponse, RequestError> {
        let command = ApiRequest::GetDevicesByControllable(GetDevicesByControllableCommand {
            controllable_name: controllable_name.to_string(),
        });
        let res = self.execute(command).await?;

        match res {
            ApiResponse::GetDevicesByControllable(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn register_event_listener(
        &self,
    ) -> Result<RegisterEventListenerResponse, RequestError> {
        let command = ApiRequest::RegisterEventListener(RegisterEventListenerCommand);
        let res = self.execute(command).await?;

        match res {
            ApiResponse::RegisterEventListener(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn fetch_events(
        &self,
        listener_id: &str,
    ) -> Result<FetchEventsResponse, RequestError> {
        let command = ApiRequest::FetchEvents(FetchEventsCommand {
            listener_id: listener_id.to_string(),
        });
        let res = self.execute(command).await?;

        match res {
            ApiResponse::FetchEvents(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn unregister_event_listener(
        &self,
        listener_id: &str,
    ) -> Result<UnregisterEventListenerResponse, RequestError> {
        let command = ApiRequest::UnregisterEventListener(UnregisterEventListenerCommand {
            listener_id: listener_id.to_string(),
        });
        let res = self.execute(command).await?;

        match res {
            ApiResponse::UnregisterEventListener(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn execute_actions(
        &self,
        execute_request: crate::commands::types::ActionGroup,
    ) -> Result<ExecuteActionGroupResponse, RequestError> {
        let command = ApiRequest::ExecuteActions(ExecuteActionGroupCommand {
            action_group: execute_request,
        });
        let res = self.execute(command).await?;

        match res {
            ApiResponse::ExecuteActions(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn get_current_executions(
        &self,
    ) -> Result<GetCurrentExecutionsResponse, RequestError> {
        let command = ApiRequest::GetCurrentExecutions(GetCurrentExecutionsCommand);
        let res = self.execute(command).await?;

        match res {
            ApiResponse::GetCurrentExecutions(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn get_execution(
        &self,
        execution_id: &str,
    ) -> Result<GetExecutionResponse, RequestError> {
        let command = ApiRequest::GetExecution(GetExecutionCommand {
            execution_id: execution_id.to_string(),
        });
        let res = self.execute(command).await?;

        match res {
            ApiResponse::GetExecution(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn cancel_all_executions(&self) -> Result<CancelAllExecutionsResponse, RequestError> {
        let command = ApiRequest::CancelAllExecutions(CancelAllExecutionsCommand);
        let res = self.execute(command).await?;

        match res {
            ApiResponse::CancelAllExecutions(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }

    pub async fn cancel_execution(
        &self,
        execution_id: &str,
    ) -> Result<CancelExecutionResponse, RequestError> {
        let command = ApiRequest::CancelExecution(CancelExecutionCommand {
            execution_id: execution_id.to_string(),
        });
        let res = self.execute(command).await?;

        match res {
            ApiResponse::CancelExecution(res) => Ok(res),
            _ => Err(RequestResponseMappingError.into()),
        }
    }
}

#[cfg(test)]
mod api_client_tests {
    use crate::api_client::{
        ApiClient, ApiClientConfig, ApiRequest, ApiResponse, CertificateHandling, DEFAULT_PORT,
        HttpProtocol,
    };
    use crate::commands::get_device::GetDeviceCommand;
    use crate::commands::get_device_state::GetDeviceStateCommand;
    use crate::commands::get_device_states::GetDeviceStatesCommand;
    use crate::commands::get_devices::GetDevicesCommand;
    use crate::commands::get_devices_by_controllable::GetDevicesByControllableCommand;
    use crate::commands::get_setup::GetSetupCommand;
    use crate::commands::get_setup_gateways::GetGatewaysCommand;
    use crate::commands::get_version::GetVersionCommand;
    use crate::commands::register_event_listener::RegisterEventListenerCommand;
    use rstest::*;
    use std::path::PathBuf;

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
    async fn type_mapping_correct() {
        fn payload_to_response(
            api_request: ApiRequest,
            request_type: &str,
            filename: &str,
        ) -> ApiResponse {
            let mut path = PathBuf::new();
            path.push(".");
            path.push("tests");
            path.push("fixtures");
            path.push("api_responses");
            path.push(request_type);
            path.push(filename);

            let json_str = std::fs::read_to_string(&path).expect("should have fixture");

            ApiClient::map_request_to_response(api_request, json_str.as_str())
                .expect("should return a ApiResponse")
        }
        // Body parsing is tested only as a side_effect, refer to respective command struct for primary testing

        let version_resp = payload_to_response(
            ApiRequest::GetVersion(GetVersionCommand),
            "get_version",
            "version_valid_1.json",
        );
        assert!(matches!(version_resp, ApiResponse::GetVersion(_)));

        let devices_resp = payload_to_response(
            ApiRequest::GetDevices(GetDevicesCommand),
            "get_devices",
            "devices_valid_1.json",
        );
        assert!(matches!(devices_resp, ApiResponse::GetDevices(_)));

        let device_resp = payload_to_response(
            ApiRequest::GetDevice(GetDeviceCommand {
                device_url: "doesnotmatter".to_string(),
            }),
            "get_device",
            "device_valid_1.json",
        );
        assert!(matches!(device_resp, ApiResponse::GetDevice(_)));

        let gateway_resp = payload_to_response(
            ApiRequest::GetGateways(GetGatewaysCommand),
            "get_gateways",
            "gateways_valid_1.json",
        );
        assert!(matches!(gateway_resp, ApiResponse::GetGateways(_)));

        let setup_resp = payload_to_response(
            ApiRequest::GetSetup(GetSetupCommand),
            "get_setup",
            "setup_valid_1.json",
        );
        assert!(matches!(setup_resp, ApiResponse::GetSetup(_)));

        let device_states_resp = payload_to_response(
            ApiRequest::GetDeviceStates(GetDeviceStatesCommand {
                device_url: "doesnotmatter".to_string(),
            }),
            "get_device_states",
            "device_states_valid_1.json",
        );
        assert!(matches!(
            device_states_resp,
            ApiResponse::GetDeviceStates(_)
        ));

        let device_state_resp = payload_to_response(
            ApiRequest::GetDeviceState(GetDeviceStateCommand {
                device_url: "doesnotmatter".to_string(),
                state_name: "doesnotmatter".to_string(),
            }),
            "get_device_state",
            "device_state_valid_1.json",
        );
        assert!(matches!(device_state_resp, ApiResponse::GetDeviceState(_)));

        let devices_by_controllable_resp = payload_to_response(
            ApiRequest::GetDevicesByControllable(GetDevicesByControllableCommand {
                controllable_name: "doesnotmatter".to_string(),
            }),
            "get_devices_by_controllable",
            "devices_by_controllable_valid_1.json",
        );
        assert!(matches!(
            devices_by_controllable_resp,
            ApiResponse::GetDevicesByControllable(_)
        ));

        let register_event_listener_resp = payload_to_response(
            ApiRequest::RegisterEventListener(RegisterEventListenerCommand),
            "register_event_listener",
            "event_listener_valid_1.json",
        );
        assert!(matches!(
            register_event_listener_resp,
            ApiResponse::RegisterEventListener(_)
        ));
    }
}
