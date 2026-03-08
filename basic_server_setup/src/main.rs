use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Connecting to port 7878");

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        println!("{:?}", stream);
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // println!(
    //     "request : {}",
    //     String::from_utf8_lossy(&buffer[..])
    // );

    let validate = b"GET / HTTP/1.1\r\n";

    let (status_line, file_name) = if buffer.starts_with(validate) {
        ("HTTP/1.1 200 OK","index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND","404.html")
    };

    let content = fs::read_to_string(file_name).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
