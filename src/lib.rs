pub mod api_client;
pub mod err {
    pub mod http;
}
pub(crate) mod config {
    pub(crate) mod tls_cert;
}
pub mod commands {
    pub mod get_device;
    pub mod get_devices;
    pub mod get_setup_gateways;
    pub mod get_version;
    pub mod traits;
    pub mod types;
}
