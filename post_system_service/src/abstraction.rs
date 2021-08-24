#[derive(Debug, Clone)]
pub struct Setting {
    pub rust_log: String,
    pub tls: bool,
    pub tls_key_path: String,
    pub tls_cert_path: String,
}

impl Setting {
    pub fn new() -> Setting {
        Setting {
            rust_log: std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info,tracing=info,warp=debug".to_owned()),
            tls: std::env::var("TLS")
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or_else(|_| false),
            tls_key_path: std::env::var("TLS_KEY")
                .unwrap_or_else(|_| "tls-certs/localhost/localhost.decrypted.key".to_owned()),
            tls_cert_path: std::env::var("TLS_CERT")
                .unwrap_or_else(|_| "tls-certs/localhost/localhost.crt".to_owned()),
        }
    }
}
