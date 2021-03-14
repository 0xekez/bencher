use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Output};
use std::time::Duration;

/// The result of sending a ping request.
#[derive(Debug)]
pub struct PingResult {
    /// The number of packets that we transmitted.
    pub transmitted: usize,
    /// The number of packets that we got back.
    pub received: usize,
    /// The percent of packets that we lost.
    pub percent_lost: f32,

    /// Min transmit time in ms.
    pub min: f32,
    /// Max transmit time in ms.
    pub max: f32,
    /// Average transmit time in ms.
    pub ave: f32,
    /// Standard devliation of transmit time in ms.
    pub stddev: f32,
}

impl std::fmt::Display for PingResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Writes the PingResult out in exactly the same form it was
        // birthed in by its ping invocation. This should make it easy
        // enough to deserialize it as we already have the code to
        // parse ping output.
        write!(
            f,
            "{} packets transmitted, {} received, {}% packet loss
rtt min/avg/max/mdev = {}/{}/{}/{} ms",
            self.transmitted,
            self.received,
            self.percent_lost,
            self.min,
            self.max,
            self.ave,
            self.stddev
        )
    }
}

/// Sends REQUEST_COUNT pings to PEER_ADDRESS with TIMEOUT
/// timeout. Returns the raw output from the unix `ping` command on
/// success and an error message on failure.
fn send_ping(request_count: u32, peer_address: &str, timeout: Duration) -> Result<Output, String> {
    // See `man ping` for a description.
    Command::new("ping")
        .arg(peer_address)
        .arg("-c")
        .arg(format!("{}", request_count))
        .arg("-W")
        .arg(format!(
            "{}",
            // On macos this is in milliseconds and on our ec2 boxes
            // it is in seconds.
            if cfg!(target_os = "macos") {
                timeout.as_secs_f32() * 1000f32
            } else if cfg!(target_os = "linux") {
                timeout.as_secs_f32()
            } else {
                return Err("unsupported os for ping commands".to_string());
            }
        ))
        .output()
        .map_err(|e| e.to_string())
}

/// Extracts transmission statistics from ping OUTPUT. Returns a tuple
/// containing the number of packets transmitted, the number of
/// packets received, and the percentage of packets that were dropped.
fn get_transmission_stats(output: &str) -> Option<(usize, usize, f32)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(\d+) packets transmitted, (\d+) (?:packets )?received, (\d+\.?\d*)% packet loss"
        )
        .unwrap();
    };
    let captures = RE.captures(output)?;

    let transmitted: usize = captures.get(1)?.as_str().parse().ok()?;
    let received: usize = captures.get(2)?.as_str().parse().ok()?;
    let loss: f32 = captures.get(3)?.as_str().parse().ok()?;

    Some((transmitted, received, loss))
}

/// Extracts transit statistics from ping OUTPUT. Returns a tuple
/// containing the min transit time, the average transit time, the max
/// transit time, the the standard devliation of transit times.
fn get_transit_stats(output: &str) -> Option<(f32, f32, f32, f32)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+.\d+)/(\d+.\d+)/(\d+.\d+)/(\d+.\d+)").unwrap();
    };
    let captures = RE.captures(output)?;

    let min: f32 = captures.get(1)?.as_str().parse().ok()?;
    let ave: f32 = captures.get(2)?.as_str().parse().ok()?;
    let max: f32 = captures.get(3)?.as_str().parse().ok()?;
    let std: f32 = captures.get(4)?.as_str().parse().ok()?;

    Some((min, ave, max, std))
}

/// Makes a ping error. This is formatted as a message followed by the
/// raw output from the ping invocation.
fn make_ping_error(message: &str, output: &Output) -> Result<PingResult, String> {
    Err(format!(
        "message: {}\n\nstdout: {}\n\nstderr: {}",
        message,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    ))
}

/// Sends REQUEST_COUNT pings to PEER_ADDRESS with timeout TIMEOUT in
/// seconds and writes the results to LOGFILE and ERRFILE.
pub fn do_ping(
    request_count: u32,
    timeout: u32,
    peer_address: &str,
    peer_port: &str,
    tcp_ping: bool,
    logfile: &mut File,
    errfile: &mut File,
) -> Result<(), String> {
    let absolute_start = crate::ms_since_epoch();
    let res = if tcp_ping {
        do_tcp_ping(peer_address, peer_port)
    } else {
        do_ping_internal(
            request_count,
            peer_address,
            Duration::new(timeout.into(), 0),
        )
        .map(|pr| pr.to_string())
    };
    match res {
        Err(e) => writeln!(errfile, "{}, {}", absolute_start, e).map_err(|e| e.to_string()),
        Ok(pr) => writeln!(logfile, "{}\n{}\n", absolute_start, pr).map_err(|e| e.to_string()),
    }
}

/// Uses nmap to perform a tcp ping of sorts to PEER_ADDRESS on
/// PEER_PORT.
///
/// TODO(zeke): this could really be more robust. So far over all of
/// our testing we have not seen a case where a ping fails so I'm just
/// assuming that this works. If nmap fails to reach the peer we'll
/// just end up returning an empty string.
pub fn do_tcp_ping(peer_address: &str, peer_port: &str) -> Result<String, String> {
    let output = Command::new("sh")
        .arg("do_tcp_ping.sh")
        .arg(peer_address)
        .arg(peer_port)
        .output()
        .map_err(|e| e.to_string())?;
    let latency = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("got ({})", latency);
    let latency = latency.parse::<f32>().map_err(|e| e.to_string())?;
    // nmap gives us a number in seconds.
    let latency = format!("{}ms", latency * 1000f32);
    Ok(latency)
}

/// Used internally to send REQUEST_COUNT pings to PEER_ADDRESS with
/// timeout TIMEOUT. `do_ping` essentially wraps this to convert `u32`
/// to `Duration` and to write error / success messages to
/// STDOUT. Returns a PingResult on success.
fn do_ping_internal(
    request_count: u32,
    peer_address: &str,
    timeout: Duration,
) -> Result<PingResult, String> {
    let output = send_ping(request_count, peer_address, timeout)?;

    // Handle the return status.
    match output.status.code() {
        None => return make_ping_error("ping invocation didn't return a status code", &output),
        Some(code) => match code {
            0 => (),
            // This indicates that "The transmission was successful
            // but no responses were received." This probably reflects
            // that every packet timed out. I'm not sure this is
            // actually an error, but if needed we can filter the logs
            // to remove it later.
            2 => {
                return make_ping_error(
                    "ping transmission successful but no responses received",
                    &output,
                )
            }
            // Alternatively, return this:
            // {
            //     return Ok(PingResult {
            //         transmitted: request_count,
            //         received: 0,
            //         percent_lost: 100f32,

            //         min: timeout.as_secs_f32() / 1000f32,
            //         max: timeout.as_secs_f32() / 1000f32,
            //         ave: timeout.as_secs_f32() / 1000f32,
            //         stddev: 0f32,
            //     })
            // }
            _ => {
                return make_ping_error(&format!("ping exited with error code: {}", code), &output)
            }
        },
    };

    let (transmitted, received, percent_lost) =
        match get_transmission_stats(&String::from_utf8_lossy(&output.stdout)) {
            Some(r) => r,
            None => return make_ping_error("failed to extract ping transmission results", &output),
        };

    let (min, max, ave, stddev) = match get_transit_stats(&String::from_utf8_lossy(&output.stdout))
    {
        Some(r) => r,
        None => return make_ping_error("failed to extract ping transit results", &output),
    };

    Ok(PingResult {
        transmitted,
        received,
        percent_lost,
        min,
        max,
        ave,
        stddev,
    })
}
