use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayConnectivity {
    pub status: String,
    pub protocol_version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gateway {
    pub gateway_id: String,
    pub connectivity: GatewayConnectivity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeviceStateValue {
    String(String),
    Int(i64),
    Map(HashMap<String, String>),
    Array(Vec<String>),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAttributeProcedureParams {
    pub duration: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAttributeProcedure {
    pub procedure_name: String,
    pub params: Option<DeviceAttributeProcedureParams>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeviceAttributeValue {
    String(String),
    States(Vec<String>),
    Procedures(Vec<DeviceAttributeProcedure>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAttribute {
    pub value: DeviceAttributeValue,
    pub name: String,
    #[serde(rename = "type")]
    pub state_type: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceState {
    pub value: DeviceStateValue,
    pub name: String,
    #[serde(rename = "type")]
    pub state_type: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionAttribute {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionState {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionCommand {
    pub command_name: String,
    pub nparams: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinition {
    pub ui_class: String,
    pub attributes: Vec<DeviceDefinitionAttribute>,
    #[serde(rename = "type")]
    pub state_type: i64,
    pub states: Vec<DeviceDefinitionState>,
    pub commands: Vec<DeviceDefinitionCommand>,
    pub widget_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    #[serde(rename = "deviceURL")]
    pub device_url: String,
    pub label: String,
    pub controllable_name: String,
    pub subsystem_id: i64,
    #[serde(rename = "type")]
    pub device_type: i64,
    pub available: bool,
    pub synced: bool,
    pub enabled: bool,
    pub states: Vec<DeviceState>,
    pub attributes: Vec<DeviceAttribute>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Setup {
    pub gateways: Vec<Gateway>,
    pub devices: Vec<Device>,
}

impl Display for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = format!(
            "{}|{}|{}",
            self.label, self.device_url, self.controllable_name
        );
        f.write_str(&str)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventListener {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    name: String,
    protocol_type: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub parameters: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    #[serde(rename = "deviceURL")]
    pub device_url: String,
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionGroup {
    pub label: Option<String>,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionGroupExecutionId {
    pub exec_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionGroupExecution {
    pub owner: String,
    pub id: String,
    pub execution_type: String,
    pub execution_sub_type: String,
    pub description: String,
    pub start_time: i64,
    pub action_group: ActionGroup,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelAllExecutionsResult {
    // Empty object, keeping for type safety
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelExecutionResult {
    // Empty object, keeping for type safety
}
