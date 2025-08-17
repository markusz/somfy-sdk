# Somfy SDK

A Rust library providing type-safe, async access to the Somfy TaHoma Local API for controlling smart home devices.

[![Crates.io](https://img.shields.io/crates/v/somfy-sdk-cli.svg)](https://crates.io/crates/somfy-sdk-cli)
[![Documentation](https://docs.rs/somfy-sdk-cli/badge.svg)](https://docs.rs/somfy-sdk-cli)

## Overview

The SDK provides a comprehensive, type-safe interface for interacting with Somfy smart home devices through the TaHoma Local API. It supports device discovery, state management, event handling, and action execution with built-in error handling and TLS support for self-signed certificates.

## Features

- **Type-safe API client** with async support using Tokio
- **Comprehensive API coverage** - all Somfy TaHoma Local API endpoints
- **Extensible command system** for adding new API endpoints
- **Robust error handling** with custom error types
- **TLS/SSL support** with custom certificate handling
- **Bearer token authentication** for secure API access
- **Structured logging** with configurable log levels

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sdk = { path = "path/to/somfy-sdk-cli/sdk" }
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use sdk::api_client::ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Create API client using gateway ID and API key
    let client = ApiClient::from("0000-1111-2222", "your-api-key");

    // Get API version
    let version = client.get_version().await?;
    println!("Protocol version: {}", version.protocol_version);
    
    // Get all devices
    let devices = client.get_devices().await?;
    for device in &devices {
        println!("Device: {} ({})", device.label, device.device_url);
    }
    
    Ok(())
}
```

## Supported API Endpoints

This SDK implements the complete [Somfy TaHoma Local API](https://somfy-developer.github.io/Somfy-TaHoma-Developer-Mode/openapi.yaml):

| Category | Endpoint | Method | SDK Method | Description |
|----------|----------|--------|------------|-------------|
| **System** | `/apiVersion` | GET | `get_version()` | Get API protocol version |
| **Setup** | `/setup/gateways` | GET | `get_gateways()` | List available gateways |
| **Setup** | `/setup` | GET | `get_setup()` | Get complete setup information |
| **Setup** | `/setup/devices` | GET | `get_devices()` | List all devices |
| **Setup** | `/setup/devices/{deviceURL}` | GET | `get_device()` | Get specific device details |
| **Setup** | `/setup/devices/{deviceURL}/states` | GET | `get_device_states()` | Get device states |
| **Setup** | `/setup/devices/{deviceURL}/states/{name}` | GET | `get_device_state()` | Get specific device state |
| **Setup** | `/setup/devices/controllables/{controllableName}` | GET | `get_devices_by_controllable()` | Get devices by controllable type |
| **Events** | `/events/register` | POST | `register_event_listener()` | Register event listener |
| **Events** | `/events/{listenerId}/fetch` | POST | `fetch_events()` | Fetch events from listener |
| **Events** | `/events/{listenerId}/unregister` | POST | `unregister_event_listener()` | Unregister event listener |
| **Execution** | `/exec/apply` | POST | `execute_actions()` | Execute action group |
| **Execution** | `/exec/current` | GET | `get_current_executions()` | Get all current executions |
| **Execution** | `/exec/current/{executionId}` | GET | `get_execution()` | Get specific execution status |
| **Execution** | `/exec/current/setup` | DELETE | `cancel_all_executions()` | Cancel all executions |
| **Execution** | `/exec/current/setup/{executionId}` | DELETE | `cancel_execution()` | Cancel specific execution |

## Configuration

### Easy Setup

The simplest way to create a client:

```rust
// Gateway ID format: "0000-1111-2222" 
// This automatically configures HTTPS, port 8443, and certificate handling
let client = ApiClient::from("your-gateway-id", "your-api-key");
```

### Advanced Configuration

For more control, use the full configuration:

```rust
use sdk::api_client::{ApiClient, ApiClientConfig, HttpProtocol, CertificateHandling};

let config = ApiClientConfig {
    url: "gateway-0000-1111-2222.local".to_string(),
    port: 8443,
    api_key: "your-api-key".to_string(),
    protocol: HttpProtocol::HTTPS,
    cert_handling: CertificateHandling::DefaultCert,
};

let client = ApiClient::new(config);
```

## API Reference

### Core Types

#### `ApiClient`

The main client for interacting with Somfy APIs:

```rust
impl ApiClient {
    // Core client creation
    pub fn new(config: ApiClientConfig) -> Self;
    pub fn from(id: &str, api_key: &str) -> Self;
    
    // System information
    pub async fn get_version(&self) -> Result<GetVersionCommandResponse, RequestError>;
    
    // Setup and device discovery
    pub async fn get_gateways(&self) -> Result<GetGatewaysResponse, RequestError>;
    pub async fn get_setup(&self) -> Result<GetSetupResponse, RequestError>;
    pub async fn get_devices(&self) -> Result<GetDevicesResponse, RequestError>;
    pub async fn get_device(&self, device_url: &str) -> Result<GetDeviceResponse, RequestError>;
    pub async fn get_device_states(&self, device_url: &str) -> Result<GetDeviceStatesResponse, RequestError>;
    pub async fn get_device_state(&self, device_url: &str, state_name: &str) -> Result<GetDeviceStateResponse, RequestError>;
    pub async fn get_devices_by_controllable(&self, controllable_name: &str) -> Result<GetDevicesByControllableResponse, RequestError>;
    
    // Event management
    pub async fn register_event_listener(&self) -> Result<RegisterEventListenerResponse, RequestError>;
    pub async fn fetch_events(&self, listener_id: &str) -> Result<FetchEventsResponse, RequestError>;
    pub async fn unregister_event_listener(&self, listener_id: &str) -> Result<UnregisterEventListenerResponse, RequestError>;
    
    // Action execution
    pub async fn execute_actions(&self, request: ExecuteRequest) -> Result<ExecuteActionsResponse, RequestError>;
    pub async fn get_current_executions(&self) -> Result<GetCurrentExecutionsResponse, RequestError>;
    pub async fn get_execution(&self, execution_id: &str) -> Result<GetExecutionResponse, RequestError>;
    pub async fn cancel_all_executions(&self) -> Result<CancelAllExecutionsResponse, RequestError>;
    pub async fn cancel_execution(&self, execution_id: &str) -> Result<CancelExecutionResponse, RequestError>;
}
```

## Usage Examples

### Device Discovery and Management

```rust
// Get complete setup information
let setup = client.get_setup().await?;
println!("Setup contains {} gateways and {} devices", 
         setup.gateways.len(), 
         setup.devices.len());

// Get all devices
let devices = client.get_devices().await?;
for device in devices {
    println!("Device: {} ({})", device.label, device.controllable_name);
}

// Get device states
if let Some(device) = devices.first() {
    let states = client.get_device_states(&device.device_url).await?;
    for state in states {
        println!("State {}: {:?}", state.name, state.value);
    }
}
```

### Event Management

```rust
// Register event listener
let listener = client.register_event_listener().await?;
println!("Event listener registered with ID: {}", listener.id);

// Fetch events (typically done in a loop)
let events = client.fetch_events(&listener.id).await?;
println!("Fetched events: {:?}", events);

// Unregister when done
client.unregister_event_listener(&listener.id).await?;
```

### Action Execution

```rust
use sdk::commands::types::{Action, Command, ExecuteRequest};

let actions = vec![Action {
    device_url: "io://0000-1111-2222/12345678".to_string(),
    commands: vec![Command {
        name: "open".to_string(),
        parameters: vec![],
    }],
}];

let request = ExecuteRequest {
    label: Some("Open blinds".to_string()),
    actions,
};

let execution = client.execute_actions(request).await?;
println!("Execution started: {}", execution.id);

// Monitor execution
let execution_details = client.get_execution(&execution.id).await?;
println!("Execution status: {:?}", execution_details);
```

## Error Handling

The SDK provides comprehensive error handling through the `RequestError` enum:

```rust
use sdk::err::http::RequestError;

match client.get_version().await {
    Ok(version) => println!("Version: {}", version.protocol_version),
    Err(RequestError::CertError) => eprintln!("Certificate validation failed"),
    Err(RequestError::AuthError) => eprintln!("Authentication failed - check API key"),
    Err(RequestError::InvalidBody) => eprintln!("Invalid response format"),
    Err(RequestError::UnknownError) => eprintln!("Unknown error occurred"),
    // ... other error types
}
```

### Error Types

- `CertError` - TLS certificate validation issues (common with self-signed certs)
- `AuthError` - Authentication failures (invalid API key, unauthorized)
- `InvalidBody` - JSON parsing or response format errors
- `InvalidRequestError` - Malformed requests
- `NotFoundError` - Resource not found (404)
- `ServerError` - Server-side errors (5xx)
- `UnknownError` - Catch-all for unexpected errors

## Testing

Run the SDK tests:

```bash
# Run SDK tests only
cargo test -p sdk

# Run tests with output
cargo test -p sdk -- --nocapture
```

## Architecture

### SDK Structure

```
sdk/
├── src/
│   ├── api_client.rs           # Main API client implementation
│   ├── commands/               # API command definitions
│   │   ├── traits.rs           # Command traits and interfaces
│   │   ├── types.rs            # Shared types and data structures
│   │   ├── get_version.rs      # Version command implementation
│   │   ├── get_setup.rs        # Setup command implementation
│   │   └── ...                 # Other command implementations
│   ├── config/                 # Configuration modules
│   ├── err/                    # Error handling
│   └── lib.rs                  # Library root
└── tests/                      # Integration tests
    └── fixtures/               # Test data
```

### Adding New Commands

1. Create a new module in `sdk/src/commands/`
2. Implement the required traits (`SomfyApiRequestCommand`, `SomfyApiRequestResponse`)
3. Add the command to the `ApiRequest` enum
4. Add the response to the `ApiResponse` enum
5. Update the client's `map_request_to_response` method
6. Add a convenience method to `ApiClient`

Example:

```rust
// In sdk/src/commands/new_command.rs
use crate::commands::traits::{RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse};

#[derive(Debug, Clone, PartialEq)]
pub struct NewCommand;

impl SomfyApiRequestCommand for NewCommand {
    fn to_request(&self) -> RequestData {
        RequestData {
            path: "/api/new".to_string(),
            method: HttpMethod::GET,
            // ... other fields
        }
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.