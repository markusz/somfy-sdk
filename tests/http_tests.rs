#[cfg(test)]
mod http_integration_tests {
    use rstest::*;
    use sdk::api_client::{ApiClient, ApiClientConfig, CertificateHandling, HttpProtocol};

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
    async fn http_get_version_ok() {
        let res = api_client_localhost()
            .get_version()
            .await
            .expect("should get a correct reponse from the getVersion endpoint");
        assert_eq!(res.protocol_version, "2022.1.3-1".to_string())
    }
}
