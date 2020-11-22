use rocket::error::LaunchError;
use rocket_contrib::serve::StaticFiles;
use std::thread;

/// Starts the server. The server serves files in the files
/// directory. For example, assuming the server is running on
/// http://localhost:8000, accessing http://localhost:8000/10mb.pdf
/// will serve a 10mb pdf file.
pub fn start() -> LaunchError {
    rocket::ignite()
        .mount("/", StaticFiles::from("./files"))
        .launch()
}

pub fn start_on_new_thread() -> thread::JoinHandle<LaunchError> {
    thread::spawn(start)
}
