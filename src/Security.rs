use rcgen::{generate_simple_self_signed, CertifiedKey as RcgenCertifiedKey};
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use rustls_pemfile::{certs, pkcs8_private_keys};
use rustls::{Certificate, PrivateKey, ServerConfig};

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

        let mut cert_file = BufReader::new(fs::File::open(&cert_path)?);
        let mut key_file = BufReader::new(fs::File::open(&key_path)?);

        let mut certs_pem = certs(&mut cert_file)?;
        let mut keys_pem = pkcs8_private_keys(&mut key_file)?;
        
        // Проверка содержимого файлов
        if certs_pem.is_empty() {
            return Err("No certificates found in cert.pem".into());
        }
        if keys_pem.is_empty() {
            return Err("No private keys found in privkey.pem".into());
        }

        let cert = Certificate(certs_pem.remove(0));
        let key = PrivateKey(keys_pem.remove(0));

        // Проверка соответствия ключа и сертификата через создание ServerConfig
        ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(vec![cert], key)
            .map_err(|e| format!("Certificate and key do not match: {}", e))?;

        Ok(())
    }
}

