pub mod api_client;
pub mod err {
    pub mod http;
}
pub(crate) mod config {
    pub(crate) mod tls_cert;
}
pub mod commands {
    pub mod get_version;
    pub mod traits;
}
