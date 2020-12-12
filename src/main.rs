use bencher::{self, config};

fn main() {
    // Serves file at http://localhost:8000/10mb.pdf. Note that this
    // will show some error output. This isn't really an issue. See:
    // https://github.com/SergioBenitez/Rocket/issues/286
    let handle = bencher::server::start_on_new_thread();
    let config = config::get_config();

    // We unwrap these as an error here is presumed to be fatal.
    bencher::do_pings_and_requests(&config).unwrap();
    handle.join().unwrap();
}
