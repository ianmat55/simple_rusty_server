use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // read incoming bytes
    stream.read(&mut buffer).unwrap();

    // decode, print to console
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    
    // send something back
    let contents = fs::read_to_string("index.html").unwrap();

    let response = format!(
        "HTTP/1.1 200\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}
fn main() -> std::io::Result<()> {
    // simple http server

    // create tcp socket at address, bind to port
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "3000";
    let endpoint: String = HOST.to_owned() + ":" + PORT;

    let listener = TcpListener::bind(endpoint)?;

    println!("Web server listenening at {}:{}...", HOST, PORT);

    // lsiten for tcp connection
    for stream in listener.incoming() {
        println!("Connection established!");
        handle_client(stream?);
    }

    // return something
    Ok(())
}
