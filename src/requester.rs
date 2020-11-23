use crate::config;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::thread;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Gets the number of ms since the epoch in the form of a massive
/// unsigned integer.
fn ms_since_epoch() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}

/// Essentially a wrapper for opening a file that creates it if it
/// doesn't exist and appends to it if it does.
fn open_logfile(name: &String) -> Result<File, String> {
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(name.clone())
        .map_err(|e| e.to_string())
}

/// Sends a request to the specified ip for a specified file. Returns
/// Ok if the request succedes and an error with information about the
/// failure code otherwise.
fn send_file_request(peer_ip: &String, peer_file: &String) -> Result<(), String> {
    let url = format!("http://{}/{}", peer_ip, peer_file);
    let resp = reqwest::blocking::get(&url).map_err(|e| e.to_string())?;
    if resp.status().is_success() {
        Ok(())
    } else {
        Err(format!("Bad response from server: {}", resp.status()))
    }
}

/// Sends a file request and logs information about it to the
/// logfiles. Information about each request is written to the log
/// file in the form <request start time>, <request elapsed time>.
fn do_file_request(
    peer_ip: &String,
    peer_file: &String,
    logfile: &mut File,
    errfile: &mut File,
) -> Result<(), String> {
    let absolute_start = ms_since_epoch();
    let start = Instant::now();
    match send_file_request(peer_ip, peer_file) {
        Ok(_) => {
            let duration = start.elapsed();
            writeln!(logfile, "{}, {:?}", absolute_start, duration).map_err(|e| e.to_string())
        }
        Err(what) => writeln!(errfile, "(), {}", absolute_start, what).map_err(|e| e.to_string()),
    }
}

/// Sends file requests to the peer ip addresses at a fixed
/// interval. This should never return unless an error occured.
pub fn do_file_requests(config: &config::Config) -> Result<(), String> {
    let mut pub_logfile = open_logfile(&config.pub_request_log_file)?;
    let mut pub_errfile = open_logfile(&config.pub_request_err_file)?;

    let mut priv_logfile = open_logfile(&config.priv_request_log_file)?;
    let mut priv_errfile = open_logfile(&config.priv_request_err_file)?;

    loop {
        // Make a file request via the public ip address.
        do_file_request(
            &config.peer_public,
            &config.peer_file,
            &mut pub_logfile,
            &mut pub_errfile,
        )?;

        // Make a file request via the private ip address.
        do_file_request(
            &config.peer_private,
            &config.peer_file,
            &mut priv_logfile,
            &mut priv_errfile,
        )?;

        thread::sleep(std::time::Duration::from_secs(3600));
    }
}
