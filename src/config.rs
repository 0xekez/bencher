use serde_derive::Deserialize;
use toml;

/// Config information for benchmarking.
#[derive(Deserialize)]
pub struct Config {
    pub hot_public: String,
    pub cold_public: String,

    pub peer_port: String,
    pub peer_file: String,

    pub hot_request_log_file: String,
    pub cold_request_log_file: String,

    pub hot_request_err_file: String,
    pub cold_request_err_file: String,

    pub hot_ping_log_file: String,
    pub cold_ping_log_file: String,

    pub hot_ping_err_file: String,
    pub cold_ping_err_file: String,

    pub ping_count: u32,
    pub ping_timeout: u32,
}

/// Get's the config information for this session. Expects a file
/// named "config.toml" to be in the cwd of this thread.
pub fn get_config() -> Config {
    toml::from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap()
}
