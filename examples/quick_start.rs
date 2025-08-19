use somfy_sdk::api_client::ApiClient;
use somfy_sdk::err::http::RequestError;

#[tokio::main]
async fn main() -> Result<(), RequestError> {
    // Create API client using gateway ID and API key
    let client = ApiClient::from("0000-1111-2222", "your-api-key").await?;

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