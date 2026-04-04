use rcgen::{generate_simple_self_signed, CertifiedKey as RcgenCertifiedKey};
use std::fs;
use std::path::PathBuf;
use openssl::pkey::PKey;
use openssl::x509::X509;

/// Класс создание и проверки сертификатов
pub struct Security {
    path_to_certs: Option<String>,
}

impl Security {
    /// Создание класса
    pub fn new(path: Option<String>) -> Self {
        Security { path_to_certs: path }
    }
    
    /// Генерация сертифката и ключа
    pub fn generate_certs(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Проверка пути 
        let path = self.path_to_certs.as_ref().ok_or("Path to certificates not provided")?;
        // Создание сертификата и ключа
        let subject_alt_names = vec!["localhost".to_string()];

        let RcgenCertifiedKey { cert, signing_key } = generate_simple_self_signed(subject_alt_names)?;

        let cert_path = PathBuf::from(path).join("cert.pem");
        let key_path = PathBuf::from(path).join("privkey.pem");

        fs::create_dir_all(path)?;
        fs::write(cert_path, cert.pem())?;
        fs::write(key_path, signing_key.serialize_pem())?;

        Ok(())
    }
    
    /// Проверка соответствия ключа и сертификата
    pub fn validate_certs(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Проверка пути  
        let path = self.path_to_certs.as_ref().ok_or("Path to certificates not provided")?;
        let cert_path = PathBuf::from(path).join("cert.pem");
        let key_path = PathBuf::from(path).join("privkey.pem");
        
        // Проверка наличия файлов
        if !cert_path.exists() {
            return Err(format!("Certificate file not found: {}", cert_path.display()).into());
        }
        if !key_path.exists() {
            return Err(format!("Private key file not found: {}", key_path.display()).into());
        }

        let cert_pem = fs::read(&cert_path)?;
        let key_pem = fs::read(&key_path)?;

        let cert = X509::from_pem(&cert_pem)?;
        let pkey = PKey::private_key_from_pem(&key_pem)?;

        if cert.public_key()?.public_eq(&pkey) {
            Ok(())
        } else {
            Err("Certificate and private key do not match.".into())
        }
    }
}
