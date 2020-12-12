use crate::ms_since_epoch;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

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
pub(crate) fn do_file_request(
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
        Err(what) => writeln!(errfile, "{}, {}", absolute_start, what).map_err(|e| e.to_string()),
    }
}
