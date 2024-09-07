use std::path::{Path, PathBuf};
use std::fs;
use std::net::TcpStream;
use std::io::prelude::*;
use crate::http_utils::send_404;

pub fn is_safe_path(root_dir: &Path, requested_path: &Path) -> bool {
    requested_path.starts_with(root_dir)
}

pub fn send_directory_listing(stream: &mut TcpStream, dir_path: &Path, root_dir: &Path) {
    let mut contents = String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Directory Listing</title></head><body>"
    );

    // Display the current directory path
    contents.push_str(&format!("<h1>Directory: {}</h1>", dir_path.display()));

    // Add "Go back up a directory" link
    if dir_path != root_dir {
        let parent = dir_path.parent().unwrap_or(root_dir);
        let relative_parent = parent.strip_prefix(root_dir).unwrap_or(Path::new("/"));
        contents.push_str(&format!(
            "<p><a href=\"/{}\">&larr; Go back up a directory</a></p>",
            relative_parent.display()
        ));
    }

    contents.push_str("<ul>");

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                let relative_path = path.strip_prefix(root_dir).unwrap_or(&path).to_string_lossy();
                let link = format!(
                    "<li><a href=\"/{}\">{}{}</a></li>",
                    relative_path,
                    name,
                    if path.is_dir() { "/" } else { "" }
                );
                contents.push_str(&link);
            }
        }
    }

    contents.push_str("</ul></body></html>");

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send directory listing: {}", e);
    }
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush stream: {}", e);
    }
}



pub fn send_file(stream: &mut TcpStream, file_path: &Path) {
    if let Ok(contents) = fs::read(file_path) {
        let content_type = mime_guess::from_path(file_path)
            .first_or_octet_stream()
            .essence_str()
            .to_string();

        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        // Remove the special handling for PDFs
        send_raw_content(stream, &contents, &content_type);
    } else {
        send_404(stream);
    }
}

pub fn send_raw_file(stream: &mut TcpStream, file_path: &Path) {
    match fs::read(file_path) {
        Ok(contents) => {
            let content_type = mime_guess::from_path(file_path)
                .first_or_octet_stream()
                .essence_str()
                .to_string();
            println!("Sending raw file: {:?}, Content-Type: {}", file_path, content_type);
            send_raw_content(stream, &contents, &content_type);
        }
        Err(e) => {
            eprintln!("Failed to read file {:?}: {}", file_path, e);
            send_404(stream);
        }
    }
}

fn send_raw_content(stream: &mut TcpStream, contents: &[u8], content_type: &str) {
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: {}\r\n\
        Content-Length: {}\r\n\
        Content-Disposition: inline\r\n\
        X-Content-Type-Options: nosniff\r\n\
        \r\n",
        content_type,
        contents.len(),
    );
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write response headers: {}", e);
        return;
    }
    if let Err(e) = stream.write_all(contents) {
        eprintln!("Failed to write file contents: {}", e);
    }
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush stream: {}", e);
    }
}
