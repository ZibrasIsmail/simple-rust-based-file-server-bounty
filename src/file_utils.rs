use std::path::{Path, PathBuf};
use std::fs;
use std::net::TcpStream;
use std::io::prelude::*;
use crate::http_utils::send_404;

pub fn is_safe_path(root_dir: &Path, requested_path: &Path) -> bool {
    requested_path.starts_with(root_dir) && requested_path.canonicalize().is_ok()
}

pub fn send_directory_listing(stream: &mut TcpStream, dir_path: &Path, root_dir: &Path) {
    let mut contents = String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Directory Listing</title></head><body>"
    );

    let relative_path = dir_path.strip_prefix(root_dir).unwrap_or(Path::new("/"));
    contents.push_str(&format!("<h1>Directory: {}</h1>", relative_path.display()));

    if dir_path != root_dir {
        if let Some(parent) = relative_path.parent() {
            contents.push_str(&format!("<p><a href=\"/{}\">Parent Directory</a></p>", 
                parent.to_string_lossy()));
        }
    }

    contents.push_str("<ul>");
    
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                let relative_path = path.strip_prefix(root_dir).unwrap_or(&path).to_string_lossy();
                let link = format!("<li><a href=\"/{}\">{}{}</a></li>", 
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

        if content_type == "application/pdf" {
            send_pdf_viewer(stream, file_name);
        } else {
            send_raw_content(stream, &contents, &content_type);
        }
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
            send_raw_content(stream, &contents, &content_type);
        }
        Err(e) => {
            eprintln!("Failed to read file {:?}: {}", file_path, e);
            send_404(stream);
        }
    }
}

fn send_pdf_viewer(stream: &mut TcpStream, file_name: &str) {
    let html_content = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>{}</title>
            <script src="https://cdnjs.cloudflare.com/ajax/libs/pdf.js/2.9.359/pdf.min.js"></script>
            <style>
                body, html {{ height: 100%; margin: 0; padding: 0; }}
                #pdf-viewer {{ width: 100%; height: 100%; }}
            </style>
        </head>
        <body>
            <canvas id="pdf-viewer"></canvas>
            <script>
                pdfjsLib.GlobalWorkerOptions.workerSrc = 'https://cdnjs.cloudflare.com/ajax/libs/pdf.js/2.9.359/pdf.worker.min.js';
                
                const loadingTask = pdfjsLib.getDocument('/raw/{}');
                loadingTask.promise.then(function(pdf) {{
                    pdf.getPage(1).then(function(page) {{
                        const scale = 1.5;
                        const viewport = page.getViewport({{scale: scale}});
                        const canvas = document.getElementById('pdf-viewer');
                        const context = canvas.getContext('2d');
                        canvas.height = viewport.height;
                        canvas.width = viewport.width;
                        const renderContext = {{
                            canvasContext: context,
                            viewport: viewport
                        }};
                        page.render(renderContext);
                    }});
                }});
            </script>
        </body>
        </html>"#,
        file_name,
        file_name
    );

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        html_content.len(),
        html_content
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn send_raw_content(stream: &mut TcpStream, contents: &[u8], content_type: &str) {
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
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
