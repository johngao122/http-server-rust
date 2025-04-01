#[allow(unused_imports)]
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_connection(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    let first_line = request.lines().next().unwrap_or("");
    let path = first_line.split_whitespace().nth(1).unwrap_or("");

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
    } else {
        String::from("HTTP/1.1 404 Not Found\r\n\r\n")
    };
    stream.write_all(response.as_bytes()).unwrap();
}
