use crate::commands::traits::{
    HttpMethod, RequestData, SomfyApiRequestCommand, SomfyApiRequestResponse,
};
use crate::commands::types::Device;
use reqwest::header::HeaderMap;
use reqwest::Body;
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Debug, Clone, PartialEq)]
pub struct GetDeviceCommand<'a> {
    pub device_url: &'a str,
}

impl SomfyApiRequestCommand for GetDeviceCommand<'_> {
    type Response = GetDeviceResponse;

    fn to_request(&self) -> RequestData {
        let device_url = &self.device_url;
        let path = format!(
            "/enduser-mobile-web/1/enduserAPI/setup/devices/{}",
            encode(device_url)
        );
        RequestData {
            path: path.to_string(),
            method: HttpMethod::GET,
            body: Body::default(),
            query_params: HashMap::default(),
            header_map: HeaderMap::default(),
        }
    }
}

pub type GetDeviceResponse = Device;

impl SomfyApiRequestResponse for GetDeviceResponse {}

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
        use crate::commands::get_device::device_json_parser::load_json;
        use crate::commands::types::DeviceAttribute;

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
        use crate::commands::get_device::device_json_parser::load_json;
        use crate::commands::types::DeviceState;

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
