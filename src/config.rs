use serde_derive::Deserialize;
use toml;

/// Config information for benchmarking.
#[derive(Deserialize)]
pub struct Config {
    pub peer_public: String,
    pub peer_private: String,

    pub peer_file: String,

    pub pub_request_log_file: String,
    pub priv_request_log_file: String,

    pub pub_request_err_file: String,
    pub priv_request_err_file: String,

    pub pub_ping_log_file: String,
    pub priv_ping_log_file: String,

    pub pub_ping_err_file: String,
    pub priv_ping_err_file: String,

    pub ping_count: u32,
    pub ping_timeout: u32,
}

/// Get's the config information for this session. Expects a file
/// named "config.toml" to be in the cwd of this thread.
pub fn get_config() -> Config {
    toml::from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap()
}
