use crate::config;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

fn send_file_request(config: &config::Config) -> Result<(), String> {
    let url = format!("http://{}/{}", config.peer_ip, config.peer_file);
    let resp = reqwest::blocking::get(&url).map_err(|e| e.to_string())?;
    if resp.status().is_success() {
        Ok(())
    } else {
        Err(format!("Bad response from server: {}", resp.status()))
    }
}

pub fn do_file_requests(config: &config::Config) -> Result<(), String> {
    let mut logfile = OpenOptions::new()
        .append(true)
        .open(config.request_log_file.clone())
        .map_err(|e| e.to_string())?;
    let mut errfile = OpenOptions::new()
        .append(true)
        .open(config.request_error_file.clone())
        .map_err(|e| e.to_string())?;
    loop {
        let start = Instant::now();
        match send_file_request(config) {
            Ok(_) => {
                let duration = start.elapsed();
                writeln!(logfile, "{:?}", duration).map_err(|e| e.to_string())?;
            }
            Err(what) => {
                writeln!(errfile, "{}", what).map_err(|e| e.to_string())?;
            }
        }
        thread::sleep(Duration::from_secs(5));
    }
}
