#[cfg(test)]
mod http_integration_tests {
    use std::time::Duration;
    use rstest::*;
    use sdk::api_client::{ApiClient, ApiClientConfig, CertificateHandling, HttpProtocol};
    use sdk::commands::types::DeviceStateValue::{Boolean, Int, String};

    #[fixture]
    fn api_client_localhost() -> ApiClient {
        ApiClient::new(ApiClientConfig {
            protocol: HttpProtocol::HTTP,
            port: 3000,
            url: "localhost".to_string(),
            api_key: "my_key".to_string(),
            cert_handling: CertificateHandling::CertProvided(
                "./src/cert/overkiz-root-ca-2048.crt".to_string(),
            ),
        })
    }

    #[rstest]
    #[tokio::test]
    #[timeout(Duration::from_millis(300))]
    async fn http_get_version() {
        let res = api_client_localhost()
            .get_version()
            .await
            .expect("should get a correct response from the getVersion endpoint");
        assert_eq!(res.protocol_version, "2022.1.3-1".to_string())
    }

    #[rstest]
    #[tokio::test]
    #[timeout(Duration::from_millis(300))]
    async fn http_get_gateways() {
        let res = api_client_localhost()
            .get_gateways()
            .await
            .expect("should get a correct response from get gateways");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].gateway_id, "0000-1111-2222");
        assert_eq!(res[0].connectivity.status, "OK")
    }

    #[rstest]
    #[tokio::test]
    #[timeout(Duration::from_millis(300))]
    async fn http_get_setup() {
        let res = api_client_localhost()
            .get_setup()
            .await
            .expect("should get a correct response from get setup");
        assert_eq!(res.devices.len(), 2);
        assert_eq!(res.gateways.len(), 1);
        assert_eq!(res.gateways[0].gateway_id, "0000-1111-2222");
        assert_eq!(res.devices[0].device_url, "io://0812-2424-9999/246132")
    }

    #[rstest]
    #[tokio::test]
    #[timeout(Duration::from_millis(300))]
    async fn http_get_devices() {
        let res = api_client_localhost()
            .get_devices()
            .await
            .expect("should get a correct response from get devices");
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].device_url, "io://0812-2424-9999/246132")
    }

    #[rstest]
    #[tokio::test]
    #[timeout(Duration::from_millis(300))]
    async fn http_get_device() {
        let res = api_client_localhost()
            .get_device("d123")
            .await
            .expect("should get a correct response from get devices");
        assert_eq!(res.device_url, "io://0812-2424-9999/246132");
        assert_eq!(res.controllable_name, "io:ExteriorVenetianBlindIOComponent")
    }

    #[rstest]
    #[tokio::test]
    #[timeout(Duration::from_millis(300))]
    async fn http_get_device_states() {
        let res = api_client_localhost()
            .get_device_states("d123")
            .await
            .expect("should get a correct response from get device states");
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].name, "core:StatusState");
        assert_eq!(res[0].value, String("available".to_string()));
        assert_eq!(res[1].value, Boolean(false))
    }

    #[rstest]
    #[tokio::test]
    #[timeout(Duration::from_millis(300))]
    async fn http_get_device_state() {
        let res = api_client_localhost()
            .get_device_state("d123", "s123")
            .await
            .expect("should get a correct response from get device state");
        assert_eq!(res.name, "core:StatusState");
        assert_eq!(res.value, Int(5))
    }

    #[rstest]
    #[tokio::test]
    #[timeout(Duration::from_millis(300))]
    async fn http_get_devices_by_controllable() {
        let res = api_client_localhost()
            .get_devices_by_controllable("c123")
            .await
            .expect("should get a correct response from get device states");
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], "io://0812-2424-9999/12936651")
    }
}
