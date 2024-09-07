use std::net::TcpStream;
use std::io::{BufReader, prelude::*};
use std::path::PathBuf;
use url_escape::decode;
use crate::file_utils::{is_safe_path, send_directory_listing, send_file, send_raw_file};
use crate::http_utils::{send_404, send_403, send_500};

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => {
            send_500(&mut stream);
            return;
        }
    };

    println!("Request: {}", request_line);

    let path = decode(request_line.split_whitespace().nth(1).unwrap_or("/")).to_string();
    let root_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    println!("Root dir: {:?}", root_dir);
    let full_path = if path.starts_with('/') {
        root_dir.join(path.trim_start_matches('/'))
    } else {
        root_dir.join(&path)
    };

    println!("Requested path: {:?}", full_path);

    if is_safe_path(&root_dir, &full_path) {
        println!("Path is safe");
        if full_path.is_dir() {
            send_directory_listing(&mut stream, &full_path, &root_dir);
        } else if full_path.is_file() {
            if path.starts_with("/raw/") {
                send_raw_file(&mut stream, &full_path);
            } else {
                send_file(&mut stream, &full_path);
            }
        } else {
            println!("File not found: {:?}", full_path);
            send_404(&mut stream);
        }
    } else {
        println!("Unsafe path: {:?}", full_path);
        send_403(&mut stream);
    }
}
