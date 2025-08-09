use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GatewayConnectivity {
    pub(crate) status: String,
    #[serde(rename = "protocolVersion")]
    pub(crate) protocol_version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gateway {
    #[serde(rename = "gatewayId")]
    pub(crate) gateway_id: String,
    pub(crate) connectivity: GatewayConnectivity,
}
