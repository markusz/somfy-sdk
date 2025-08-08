use log::debug;
use reqwest::Certificate;
use std::fs::File;
use std::path::PathBuf;

pub(crate) struct TlsCertHandler;
pub enum CertificateError {
    RemoteCertError,
    InvalidLocalCert,
    FileSystemError,
}

const REMOTE_CERT_LOCATION: &str = "https://ca.overkiz.com/overkiz-root-ca-2048.crt";
const LOCAL_CERT_LOCATION_FOLDER: &str = ".somfycli";
const LOCAL_CERT_LOCATION_FILENAME: &str = "cert.crt";

impl TlsCertHandler {
    pub(crate) async fn ensure_local_certificate() -> Result<Certificate, CertificateError> {
        let folder = Self::get_folder_location();
        let path = Self::get_file_location();

        Self::ensure_local_folder(folder)?;
        if std::fs::read(&path).is_err() {
            debug!("Cert not available at {path:?}. Downloading from {REMOTE_CERT_LOCATION}");
            Self::download_cert(&path).await?;
        }

        let cert = std::fs::read(&path).map_err(|_| CertificateError::InvalidLocalCert)?;
        Certificate::from_pem(&cert).map_err(|_| CertificateError::InvalidLocalCert)
    }

    fn ensure_local_folder(folder: PathBuf) -> Result<(), CertificateError> {
        std::fs::create_dir_all(folder).map_err(|_| CertificateError::FileSystemError)?;
        Ok(())
    }

    async fn download_cert(path: &PathBuf) -> Result<(), CertificateError> {
        let resp = reqwest::get(REMOTE_CERT_LOCATION)
            .await
            .map_err(|_| CertificateError::RemoteCertError)?;
        let body = resp
            .text()
            .await
            .map_err(|_| CertificateError::RemoteCertError)?;
        let mut out = File::create(path).map_err(|_| CertificateError::FileSystemError)?;
        std::io::copy(&mut body.as_bytes(), &mut out)
            .map_err(|_| CertificateError::FileSystemError)?;

        Ok(())
    }

    fn get_file_location() -> PathBuf {
        let mut path = Self::get_folder_location();
        path.push(LOCAL_CERT_LOCATION_FILENAME);
        path
    }

    fn get_folder_location() -> PathBuf {
        let mut path = PathBuf::new();
        let home = dirs::home_dir().unwrap_or(PathBuf::from("."));
        path.push(home);
        path.push(LOCAL_CERT_LOCATION_FOLDER);

        path
    }
}
