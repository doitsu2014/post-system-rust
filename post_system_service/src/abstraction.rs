use std::env::var;

use crate::extension::StringExtensions;

#[derive(Debug, Clone)]
pub struct Setting {
    pub rust_log: String,
    pub tls: bool,
    pub tls_key_path: String,
    pub tls_cert_path: String,
    pub log_file_path: String
}

impl Setting {
    pub fn new() -> Setting {
        Setting {
            rust_log: var("RUST_LOG").unwrap_or_else(|_| "info,tracing=info,warp=debug".to_owned()),
            tls: var("TLS").to_boolean(),
            tls_key_path: var("TLS_KEY")
                .unwrap_or_else(|_| "tls-certs/localhost/localhost.decrypted.key".to_owned()),
            tls_cert_path: var("TLS_CERT")
                .unwrap_or_else(|_| "tls-certs/localhost/localhost.crt".to_owned()),
            log_file_path: var("LOG_FILE")
                .unwrap_or_else(|_| "./logs".to_owned()),
        }
    }
}
