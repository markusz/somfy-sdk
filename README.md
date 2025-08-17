# Somfy SDK

A Rust library providing type-safe, async access to the Somfy TaHoma Local API for controlling smart home devices.

[![Unit & Integration Tests](https://github.com/markusz/somfy-sdk/actions/workflows/tests.yml/badge.svg)](https://github.com/markusz/somfy-sdk/actions/workflows/tests.yml)
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
somfy_sdk = { package = "somfy-sdk", version = "0.1.0", path = "path/to/somfy-sdk-cli/sdk" }
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
| **Execution** | `/exec/apply` | POST | `execute_actions()` ⚠️ | Execute action group (requires `generic-exec` feature) |
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

## Feature Flags

The SDK uses feature flags to control access to potentially dangerous functionality:

### `generic-exec` Feature

The `execute_actions()` method is gated behind the `generic-exec` feature flag because it provides raw access to the `/exec/apply` endpoint, which can potentially harm your Somfy devices if used incorrectly.

#### Enabling the Feature

Add the feature to your `Cargo.toml`:

```toml
[dependencies]
somfy_sdk = { package = "somfy-sdk", version = "0.1.0", path = "path/to/somfy-sdk-cli/sdk", features = ["generic-exec"]}
```

#### Why is this Feature Gated?

The generic execution API allows sending arbitrary commands to any device:

```rust
// ⚠️ This can be dangerous - wrong device URL or command can cause damage
let actions = vec![Action {
    device_url: "io://0000-1111-2222/12345678".to_string(),
    commands: vec![Command {
        name: "writeManufacturerData".to_string(),  // Could brick your device!
        parameters: vec!["invalid-data".to_string()],
    }],
}];

client.execute_actions(ExecuteRequest { 
    label: Some("Dangerous operation".to_string()), 
    actions 
}).await?;
```

#### Safer Alternative: Custom Commands

Instead of using the generic API, we **strongly recommend** creating type-safe, domain-specific commands (see [Extending the SDK](#extending-the-sdk-with-custom-commands) section). These provide compile-time safety and prevent accidental misuse.

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
    // ⚠️ execute_actions needs to be enabled via the generic-exec feature flag. Be very careful when using it, as it can potentially harm your Somfy devices
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

## Extending the SDK with Custom Commands

**The SDK is built for extensibility.** You can adapt to API changes, handle undocumented behaviors, and create type-safe, domain-specific commands by implementing the required traits.

### Why Extend the SDK?

There are two primary use cases for creating custom commands:

#### 1. **Adapting to API Changes and Undocumented Behavior**

The real-world API sometimes deviates from the API specification (e.g., see example below). While we strive to find and cover all such scenarios (please raise an issue [here](https://github.com/markusz/somfy-sdk-cli/issues)), this may happen with your specific Somfy configuration. 
In such scenarios, you can use a custom command to work around this behavior until the fix is implemented in mainline.

```rust
// ./sdk/src/get_execution.rs
impl SomfyApiRequestResponse for GetExecutionResponse {
    fn from_body(body: &str) -> Result<GetExecutionResponse, RequestError> {
        // Handle undocumented API behavior:
        // - For existing but past execId, returns "null"
        // - For non-existing execId, returns "[]"  
        if body == "null" || body == "[]" {
            return Err(RequestError::Status {
                source: None,
                status: StatusCode::NOT_FOUND,
            });
        }
        Ok(serde_json::from_str(body)?)
    }
}
```

#### 2. **Creating Type-Safe, Domain-Specific Commands**

The generic execute actions API (`/exec/apply`) is powerful but can be dangerous if misused. 
It is thus disabled by default and needs to be enabled through the "generic-exec" feature flag.

Custom commands provide **compile-time safety** and **prevent accidental misuse** by making commands explicit and known at compile time.

Consider the following example:

```rust
// ❌ Generic API with client.execute_actions(action) enabled - easy to make potentially destructive mistakes
let request = ActionGroup {
    label: Some(action_group_label),
    actions: vec![Action {
        device_url: "device-url".to_string(),
        commands: vec![Command {
            name: "writeManufacturerData".to_string(),  // 💀 Running this can really ruin your day
            parameters: vec!["some-config".to_string()],
        }],
    }],
};

api_client.execute_actions(request).await

// ✅ Type-safe domain command - impossible to misuse, client.execute_actions(..) not even available

let cmd = CloseLivingRoomShuttersCommand { position: 75 }; // see implementation below
client.execute(cmd).await?;
```

### Implementation Examples

#### Type-Safe Device Commands

Here's how to create a domain-specific command that prevents dangerous mistakes:

```rust
use serde::{Serialize, Deserialize};
use somfy_sdk::commands::execute_action_group::ExecuteActionGroupResponse;
use somfy_sdk::commands::traits::{RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse, HttpMethod};
use reqwest::{Body, header::HeaderMap};
use std::collections::HashMap;

// Type-safe command for a specific device with validation
#[derive(Debug, Clone, PartialEq)]
pub struct CloseLivingRoomShuttersCommand {
    pub position: u8,  // 0-100, validated at compile time via newtypes if needed
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct SetClosureAction {
    position: u8,
}

impl SomfyApiRequestCommand for CloseLivingRoomShuttersCommand {
    type Response = ExecuteActionGroupResponse;

    fn to_request(&self) -> RequestData {
        // Hard-coded device URL - impossible to target wrong device
        const LIVING_ROOM_SHUTTERS_URL: &str = "io://0000-1111-2222/12345678";
        
        // Validate position at runtime (or use newtypes for compile-time validation)
        let position = self.position.min(100);
        
        let action = serde_json::json!({
            "label": "Close living room shutters",
            "actions": [{
                "deviceURL": LIVING_ROOM_SHUTTERS_URL,
                "commands": [{
                    "name": "setClosure",
                    "parameters": [position.to_string()]
                }]
            }]
        });

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/exec/apply".to_string(),
            method: HttpMethod::POST,
            body: Body::from(action.to_string()),
            query_params: HashMap::new(),
            header_map: headers,
        }
    }
}

// Usage - impossible to misuse!
let client = ApiClient::from("gateway-id", "api-key");
let response = client.execute(CloseLivingRoomShuttersCommand { 
    position: 75 
}).await?;
println!("Started execution: {}", response.exec_id);
```

#### Handling API Quirks and Custom Response Processing

Adapt to undocumented behaviors by customizing response handling. Here's a hypothetical example where the API introduces inconsistent response formats:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceStatusCommand<'a> {
    pub device_url: &'a str,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DeviceStatusResponse {
    pub status: String,
    pub is_online: bool,
}

impl SomfyApiRequestResponse for DeviceStatusResponse {
    fn from_body(body: &str) -> Result<Self, RequestError> {
        // Handle API returning different formats based on device state
        if body.trim().is_empty() {
            // Empty response means device is offline
            return Ok(DeviceStatusResponse {
                status: "offline".to_string(),
                is_online: false,
            });
        }
        
        if body == "\"maintenance\"" {
            // API sometimes returns a plain string for maintenance mode
            return Ok(DeviceStatusResponse {
                status: "maintenance".to_string(),
                is_online: false,
            });
        }
        
        // Try to parse as regular JSON
        match serde_json::from_str::<serde_json::Value>(body)? {
            serde_json::Value::Object(map) => {
                let status = map.get("status")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                    
                let is_online = status == "available" || status == "online";
                
                Ok(DeviceStatusResponse { status, is_online })
            }
            _ => Err(RequestError::InvalidBody),
        }
    }
}
```

### Best Practices for Custom Commands

1. **Safety First**: Use type-safe, domain-specific commands for potentially harmful operations
2. **Handle API quirks**: Override `from_body()` to handle undocumented behaviors gracefully
3. **Validation**: Validate parameters at compile-time with newtypes or at runtime with bounds checking
4. **Hard-code device URLs**: For device-specific commands, hard-code URLs to prevent targeting wrong devices
5. **Meaningful errors**: Provide clear error messages for validation failures
6. **Testing**: Add comprehensive unit tests, especially for edge cases and API quirks
7. **Documentation**: Document any API behaviors your commands work around

### Integration with Built-in Commands

Your custom commands work seamlessly with the existing SDK infrastructure:

```rust
// Mix custom and built-in commands
let version = client.get_version().await?;
let response = client.execute(CloseLivingRoomShuttersCommand { position: 50 }).await?;
let devices = client.get_devices().await?;

println!("API Version: {}, Execution: {}", version.protocol_version, response.exec_id);
```

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.