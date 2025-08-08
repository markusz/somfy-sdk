use sdk::api_client::ApiClient;

#[cfg(test)]
mod http_integration_tests {
    use rstest::*;
    use sdk::api_client::ApiRequest::GetVersion;
    use sdk::api_client::{ApiClient, ApiRequest};
    use sdk::commands::get_version::GetVersionCommand;

    #[fixture]
    fn api_client_localhost() -> ApiClient {
        ApiClient::from("localhost".to_string(), 3000, "MY_API_KEY".to_string())
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
