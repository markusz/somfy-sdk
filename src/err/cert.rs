use thiserror::Error;

#[derive(Debug, Error)]
pub enum CertificateError {
    #[error("remote cert could not be retrieved")]
    RemoteCertError,
    #[error("local cert is invalid")]
    InvalidLocalCert,
    #[error("fs error: {0}")]
    FileSystemError(#[from] anyhow::Error),
}
