use reqwest::Body;
use somfy_sdk::api_client::ApiClient;
use somfy_sdk::commands::execute_action_group::ExecuteActionGroupResponse;
use somfy_sdk::commands::traits::{HttpMethod, RequestData, SomfyApiRequestCommand};
use somfy_sdk::commands::types::{Action, ActionGroup, Command};
use somfy_sdk::err::http::RequestError;
use std::collections::HashMap;

// Type-safe command for a specific device with validation
#[derive(Debug, Clone, PartialEq)]
pub struct CloseLivingRoomShuttersCommand {
    pub position: u8, // 0-100, validated at compile time via newtypes if needed
}

impl SomfyApiRequestCommand for CloseLivingRoomShuttersCommand {
    type Response = ExecuteActionGroupResponse;

    fn to_request(&self) -> Result<RequestData, RequestError> {
        // Hard-coded device URLs - impossible to target wrong devices
        const LIVING_ROOM_SHUTTER_EAST_URL: &str = "io://0000-1111-2222/12345678";
        const LIVING_ROOM_SHUTTER_SOUTH_URL: &str = "io://0000-1111-2222/87654321";

        // Validate position at runtime (or use newtypes for compile-time validation)
        let position = self.position.min(100);

        let action_group = ActionGroup {
            label: Some("Close living room shutters".to_string()),
            actions: vec![
                Action {
                    device_url: LIVING_ROOM_SHUTTER_EAST_URL.to_string(),
                    commands: vec![Command {
                        name: "setClosure".to_string(),
                        parameters: vec![position.to_string()],
                    }],
                },
                Action {
                    device_url: LIVING_ROOM_SHUTTER_SOUTH_URL.to_string(),
                    commands: vec![Command {
                        name: "setClosure".to_string(),
                        parameters: vec![position.to_string()],
                    }],
                },
            ],
        };

        let body_json = serde_json::to_string(&action_group)?;

        Ok(RequestData {
            path: "/enduser-mobile-web/1/enduserAPI/exec/apply".to_string(),
            method: HttpMethod::POST,
            body: Body::from(body_json),
            query_params: HashMap::new(),
            header_map: RequestData::default_post_headers()?,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), RequestError> {
    let client = ApiClient::from("gateway-id", "api-key");
    let response = client
        .execute(CloseLivingRoomShuttersCommand { position: 75 })
        .await?;
    println!("Started execution: {}", response.exec_id);
    Ok(())
}
