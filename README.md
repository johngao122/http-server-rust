# HTTP Server Implementation in Rust

A lightweight HTTP server that supports:

-   Basic GET/POST requests
-   File operations
-   Gzip compression
-   Concurrent connections

## Features

-   `GET /` - Returns 200 OK
-   `GET /echo/<message>` - Returns message with optional gzip compression
-   `GET /user-agent` - Returns the client's User-Agent
-   `GET /files/<filename>` - Serves files from specified directory
-   `POST /files/<filename>` - Saves files to specified directory
-   Concurrent request handling using threads
-   Content-Type and Content-Length headers
-   Gzip compression when supported by client

## Usage

```bash
# Run server with default directory (.)
./your_program.sh

# Run server with custom files directory
./your_program.sh --directory /path/to/files

# Example requests
curl -v http://localhost:4221/
curl -v http://localhost:4221/echo/hello
curl -v -H "Accept-Encoding: gzip" http://localhost:4221/echo/hello
curl -v http://localhost:4221/user-agent
curl -v http://localhost:4221/files/example.txt
curl -v --data "content" -H "Content-Type: application/octet-stream" http://localhost:4221/files/example.txt
```

## Implementation Details

-   Built with Rust standard library and minimal dependencies
-   Uses `flate2` for gzip compression
-   Thread-per-connection model for concurrency
-   Proper handling of HTTP headers and response codes
-   Binary file support with proper Content-Type headers
