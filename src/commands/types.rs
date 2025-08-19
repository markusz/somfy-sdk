use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Result};

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

impl Display for DeviceAttributeProcedure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.procedure_name.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeviceAttributeValue {
    String(String),
    States(Vec<String>),
    Procedures(Vec<DeviceAttributeProcedure>),
}

impl Display for DeviceAttributeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            DeviceAttributeValue::String(s) => f.write_str(s),
            DeviceAttributeValue::States(states) => f.write_str(states.join(", ").as_str()),
            DeviceAttributeValue::Procedures(procedures) => {
                let s = procedures
                    .iter()
                    .map(|p| p.procedure_name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                f.write_str(s.as_str())
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_attribute_procedure_display() {
        let procedure = DeviceAttributeProcedure {
            procedure_name: "test_procedure".to_string(),
            params: None,
        };
        assert_eq!(procedure.to_string(), "test_procedure");
    }

    #[test]
    fn test_device_attribute_value_display_string() {
        let value = DeviceAttributeValue::String("test_string".to_string());
        assert_eq!(value.to_string(), "test_string");
    }

    #[test]
    fn test_device_attribute_value_display_states() {
        let value = DeviceAttributeValue::States(vec!["state1".to_string(), "state2".to_string()]);
        assert_eq!(value.to_string(), "state1, state2");
    }

    #[test]
    fn test_device_attribute_value_display_procedures() {
        let procedures = vec![
            DeviceAttributeProcedure {
                procedure_name: "proc1".to_string(),
                params: None,
            },
            DeviceAttributeProcedure {
                procedure_name: "proc2".to_string(),
                params: None,
            },
        ];
        let value = DeviceAttributeValue::Procedures(procedures);
        assert_eq!(value.to_string(), "proc1, proc2");
    }

    #[test]
    fn test_device_display() {
        let device = Device {
            device_url: "io://test-device".to_string(),
            label: "Test Device".to_string(),
            controllable_name: "TestController".to_string(),
            subsystem_id: 1,
            device_type: 2,
            available: true,
            synced: true,
            enabled: true,
            states: vec![],
            attributes: vec![],
        };
        assert_eq!(
            device.to_string(),
            "Test Device|io://test-device|TestController"
        );
    }
}
