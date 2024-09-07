# Simple File Server

This project is a simple file server implemented in Rust, created as part of the StackUp bounty challenge. It serves files as a simple HTML document, listing all directories and folders as links.

## Test Files

To help test the functionality of this file server, we've included sample files in the `test_files` directory:

- `simple-http/index.html`: A simple HTML file to test basic file serving
- `document.pdf`: A PDF file to test PDF viewing capabilities
- `video_sample.mp4`: A short video file to test video playback

## Features

- Traverse up and into directories
- Read and view files
- Watch videos (if supported by the browser)
- Properly decode special characters (e.g., CJK characters)
- Prevent backtracking beyond the server's root directory

## Requirements

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Dependencies

This project uses minimal external dependencies to meet the bounty requirements:

- `url-escape`: For handling CJK characters and special characters in URLs
- `mime_guess`: For inferring MIME types of files

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/your-username/simple-file-server.git
   cd simple-file-server
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run the server using:

```
cargo run --release
```

By default, the server will start on `http://localhost:8080`. Open this URL in your web browser to access the file server.

## Project Structure

- `src/main.rs`: Entry point of the application
- `src/server.rs`: Server setup and connection handling
- `src/handlers.rs`: Request handling functions
- `src/file_utils.rs`: File-related utility functions
- `src/http_utils.rs`: HTTP-related utility functions

## Implementation Details

- The server uses Rust's standard library for networking (`std::net`)
- Directory traversal is implemented using `std::path`
- File type detection is done using file extensions and the `mime_guess` crate
- URL decoding is handled by the `url-escape` crate
- Backtracking prevention is implemented by comparing canonicalized paths

## Limitations

- The server is designed for local use and hasn't been tested for production environments
- Video playback support depends on the browser's capabilities
