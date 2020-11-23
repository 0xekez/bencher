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
}

/// Get's the config information for this session. Eventually this
/// should probably read from a toml file, but this works for now.
pub fn get_config() -> Config {
    toml::from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap()
}
