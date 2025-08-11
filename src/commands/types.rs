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
    duration: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAttributeProcedure {
    procedure_name: String,
    params: Option<DeviceAttributeProcedureParams>,
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
    value: DeviceAttributeValue,
    name: String,
    #[serde(rename = "type")]
    state_type: i64,
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
    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionState {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionCommand {
    command_name: String,
    nparams: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinition {
    ui_class: String,
    attributes: Vec<DeviceDefinitionAttribute>,
    #[serde(rename = "type")]
    state_type: i64,
    states: Vec<DeviceDefinitionState>,
    commands: Vec<DeviceDefinitionCommand>,
    widget_name: String,
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

#[cfg(test)]
mod device_json_parser {
    use crate::commands::types::Device;
    use std::path::PathBuf;

    fn load_json(name: &str) -> String {
        let mut path = PathBuf::new();
        path.push(".");
        path.push("tests");
        path.push("fixtures");
        path.push("api_responses");
        path.push("get_device");
        path.push(name);

        std::fs::read_to_string(&path).expect("should have fixture")
    }

    #[test]
    fn valid_minimal_device_json() {
        let str = load_json("device_valid_1.json");
        let parsed: Device = serde_json::from_str(&str)
            .expect("should parse DeviceAttribute with value: string[] correctly");
        assert_eq!(parsed.controllable_name, "io:StackComponent")
    }

    mod attributes {
        use crate::commands::types::DeviceAttribute;
        use crate::commands::types::device_json_parser::load_json;

        #[test]
        fn parse_valid_str_array_correctly() {
            let str = load_json("attributes_valid_1.json");
            let parsed: DeviceAttribute = serde_json::from_str(&str)
                .expect("should parse DeviceAttribute with value: string[] correctly");
            assert_eq!(parsed.name, "core:SupportedManufacturerSettingsCommands")
        }

        #[test]
        fn parse_valid_procedure_array_correctly() {
            let str = load_json("attributes_valid_2.json");
            let parsed: DeviceAttribute = serde_json::from_str(&str)
                .expect("should parse DeviceAttribute with value: procedures[] correctly");
            assert_eq!(parsed.name, "core:SupportedManufacturerProcedures")
        }

        #[test]
        fn parse_valid_string_correctly() {
            let str = load_json("attributes_valid_3.json");
            let parsed: DeviceAttribute = serde_json::from_str(&str)
                .expect("should parse DeviceAttribute with value: procedures[] correctly");
            assert_eq!(parsed.name, "core:FirmwareRevision")
        }
    }

    mod states {
        use crate::commands::types::DeviceState;
        use crate::commands::types::device_json_parser::load_json;

        #[test]
        fn parse_valid_str_value_correctly() {
            let str = load_json("states_valid_1.json");
            let parsed: DeviceState = serde_json::from_str(&str)
                .expect("should parse DeviceAttribute with value: string[] correctly");
            assert_eq!(parsed.name, "core:StatusState")
        }

        #[test]
        fn parse_valid_map_value_correctly() {
            let str = load_json("states_valid_2.json");
            let parsed: DeviceState = serde_json::from_str(&str)
                .expect("should parse DeviceAttribute with value: string[] correctly");
            assert_eq!(parsed.name, "core:ManufacturerSettingsState")
        }

        #[test]
        fn parse_valid_i64_value_correctly() {
            let str = load_json("states_valid_3.json");
            let parsed: DeviceState = serde_json::from_str(&str)
                .expect("should parse DeviceAttribute with value: string[] correctly");
            assert_eq!(parsed.name, "core:Memorized1PositionState")
        }

        #[test]
        fn parse_valid_bool_value_correctly() {
            let str = load_json("states_valid_4.json");
            let parsed: DeviceState = serde_json::from_str(&str)
                .expect("should parse DeviceAttribute with value: string[] correctly");
            assert_eq!(parsed.name, "core:MovingState")
        }
    }

    mod definition {}
}
