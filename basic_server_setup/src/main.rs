use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listner =TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Connecting to port 7878");

    for stream in listner.incoming(){
            let stream = stream.unwrap();
            println!("{:?}",stream);
            handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;1024];

    stream.read(&mut buffer).unwrap();

    // println!(
    //     "request : {}",
    //     String::from_utf8_lossy(&buffer[..])
    // );

    let response = "HTTP/1.1 200 OK\r\n\r\nHello from Rust web server!";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
