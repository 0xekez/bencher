use bencher::{self, config};

fn main() {
    let handle = bencher::server::start_on_new_thread();
    let config = config::get_config();
    bencher::requester::do_file_requests(&config);
    handle.join();
}
