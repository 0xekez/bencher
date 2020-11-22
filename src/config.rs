/// Config information for benchmarking.
pub struct Config {
    /// The ip address of our peer.
    pub peer_ip: String,
    /// The file we should request from our peer.
    pub peer_file: String,
    /// The file for writing request logs to.
    pub request_log_file: String,
    /// The file to write request errors to.
    pub request_error_file: String,
    /// The file for writing ping logs to.
    pub ping_log_file: String,
}

/// Get's the config information for this session. Eventually this
/// should probably read from a toml file, but this works for now.
pub fn get_config() -> Config {
    Config {
        peer_ip: "localhost:8000".to_string(),
        peer_file: "10mb.pdf".to_string(),
        ping_log_file: "files/ping_log.txt".to_string(),
        request_log_file: "files/request_log.txt".to_string(),
        request_error_file: "files/request_err.txt".to_string(),
    }
}
