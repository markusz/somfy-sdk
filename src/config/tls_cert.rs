use crate::err::cert::CertificateError;
use log::debug;
use reqwest::Certificate;
use std::fs::File;
use std::path::PathBuf;

pub(crate) struct TlsCertHandler;

const REMOTE_CERT_LOCATION: &str = "https://ca.overkiz.com/overkiz-root-ca-2048.crt";
const LOCAL_CERT_LOCATION_FOLDER: &str = ".somfy_sdk";
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
        std::fs::create_dir_all(folder).map_err(|e| CertificateError::FileSystemError(e.into()))?;
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
        let mut out =
            File::create(path).map_err(|e| CertificateError::FileSystemError(e.into()))?;
        std::io::copy(&mut body.as_bytes(), &mut out)
            .map_err(|e| CertificateError::FileSystemError(e.into()))?;

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

#[cfg(test)]
#[tokio::test]
async fn download_cert() {
    let path = std::path::Path::new("./tests/fixtures/temp_cert.crt").to_path_buf();
    println!("{path:?}");
    let _ = TlsCertHandler::download_cert(&path).await;

    let cert = std::fs::read(&path).expect("cert should have been downloaded");
    let first_27_chars = str::from_utf8(&cert[..27]).expect("should be utf8 text");
    assert_eq!(first_27_chars, "-----BEGIN CERTIFICATE-----");
    assert_eq!(cert.len(), 1184);
    std::fs::remove_file(&path).expect("should not fail on cleanup")
}
