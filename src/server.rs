use std::net::TcpListener;
use std::io;
use crate::handlers::handle_connection;

pub fn run_server(address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Server running on http://{}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }

    Ok(())
}
