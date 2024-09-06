use std::net::TcpStream;
use std::io::prelude::*;

pub fn send_404(stream: &mut TcpStream) {
    let contents = "404 Not Found";
    let response = format!(
        "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn send_403(stream: &mut TcpStream) {
    let contents = "403 Forbidden";
    let response = format!(
        "HTTP/1.1 403 FORBIDDEN\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn send_500(stream: &mut TcpStream) {
    let contents = "500 Internal Server Error";
    let response = format!(
        "HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}
