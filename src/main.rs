use std::env;
use std::fs::File;
#[allow(unused_imports)]
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    let directory = args
        .iter()
        .position(|x| x == "--directory")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string())
        .unwrap_or_else(|| ".".to_string());

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let directory = directory.clone();
                thread::spawn(move || handle_connection(stream, &directory));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: std::net::TcpStream, directory: &str) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    let first_line = request.lines().next().unwrap_or("");
    let path = first_line.split_whitespace().nth(1).unwrap_or("");
    let request_type = first_line.split_whitespace().next().unwrap_or("");

    if request_type == "POST" && path.starts_with("/files/") {
        let file_name = &path[7..];
        let file_path = Path::new(directory).join(file_name);

        if let Some(body_start) = request.find("\r\n\r\n") {
            let body_start = body_start + 4;
            let body = &buffer[body_start..];

            if let Some(len_line) = request
                .lines()
                .find(|line| line.starts_with("Content-Length: "))
            {
                if let Ok(content_length) = len_line[16..].parse::<usize>() {
                    let mut file = File::create(file_path).unwrap();
                    file.write_all(&body[..content_length]).unwrap();

                    let response = "HTTP/1.1 201 Created\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                    return;
                }
            }
        }
    }

    let response = if path == "/" {
        String::from("HTTP/1.1 200 OK\r\n\r\n")
    } else if path.starts_with("/echo/") {
        let body = &path[6..];
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
            body.len(),
            body
        )
    } else if path.starts_with("/user-agent") {
        let user_agent = request
            .lines()
            .find(|line| line.starts_with("User-Agent: "))
            .map(|line| &line[12..])
            .unwrap_or("Unknown");
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
            user_agent.len(),
            user_agent
        )
    } else if path.starts_with("/files/") {
        let file_name = &path[7..];
        let file_path = Path::new(directory).join(file_name);
        match std::fs::read(file_path) {
            Ok(content) => format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                content.len(),
                String::from_utf8_lossy(&content)
            ),
            Err(_) => String::from("HTTP/1.1 404 Not Found\r\n\r\n")
        }
    } else {
        String::from("HTTP/1.1 404 Not Found\r\n\r\n")
    };
    stream.write_all(response.as_bytes()).unwrap();
}
