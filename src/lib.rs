pub mod config;
pub mod pinger;
pub mod requester;
pub mod server;

use std::fs::{File, OpenOptions};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

/// Gets the number of ms since the epoch in the form of a massive
/// unsigned integer.
pub(crate) fn ms_since_epoch() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}

/// Essentially a wrapper for opening a file that creates it if it
/// doesn't exist and appends to it if it does.
pub(crate) fn open_logfile(name: &String) -> Result<File, String> {
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(name.clone())
        .map_err(|e| e.to_string())
}

/// Does the grunt work of the library. Sends pings and requests to
/// according to the configuration at a rate of once per hour.
pub fn do_pings_and_requests(config: &config::Config) -> Result<(), String> {
    let mut pub_req_logfile = open_logfile(&config.pub_request_log_file)?;
    let mut pub_req_errfile = open_logfile(&config.pub_request_err_file)?;

    let mut priv_req_logfile = open_logfile(&config.priv_request_log_file)?;
    let mut priv_req_errfile = open_logfile(&config.priv_request_err_file)?;

    let mut pub_png_logfile = open_logfile(&config.pub_ping_log_file)?;
    let mut pub_png_errfile = open_logfile(&config.pub_ping_err_file)?;

    let mut priv_png_logfile = open_logfile(&config.priv_ping_log_file)?;
    let mut priv_png_errfile = open_logfile(&config.priv_ping_err_file)?;

    loop {
        // Make a file request via the public ip address.
        requester::do_file_request(
            &config.peer_public,
            &config.peer_file,
            &mut pub_req_logfile,
            &mut pub_req_errfile,
        )?;

        // Make a file request via the private ip address.
        requester::do_file_request(
            &config.peer_private,
            &config.peer_file,
            &mut priv_req_logfile,
            &mut priv_req_errfile,
        )?;

        // Make a ping request to the public ip address.
        pinger::do_ping(
            config.ping_count,
            config.ping_timeout,
            &config.peer_public,
            &mut pub_png_logfile,
            &mut pub_png_errfile,
        )?;

        // Make a ping request to the private ip address.
        pinger::do_ping(
            config.ping_count,
            config.ping_timeout,
            &config.peer_public,
            &mut priv_png_logfile,
            &mut priv_png_errfile,
        )?;

        // Sleep a while.
        thread::sleep(std::time::Duration::from_secs(3600));
    }
}
