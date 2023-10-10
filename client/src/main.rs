// use std::io::{Read, Write};
// use std::net::TcpStream;

fn main() {
    let server_address = "127.0.0.1";
    let server_port = "8080";

    let hostname = "localhost";
    let port = "8080";
    let server_address = format!("{}:{}", hostname, port);



    // let mut stream = TcpStream::connect(SERVER_ADDRESS).unwrap();

    // let message = "Hello, server!";
    // stream.write(message.as_bytes()).unwrap();

    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    // let response = String::from_utf8_lossy(&buffer[..]);
    // println!("Server response: {}", response);
}
