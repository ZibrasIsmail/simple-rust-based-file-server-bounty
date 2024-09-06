mod server;
mod handlers;
mod file_utils;
mod http_utils;

fn main() {
    server::run_server("127.0.0.1:8080").unwrap_or_else(|error| {
        eprintln!("Server error: {}", error);
        std::process::exit(1);
    });
}