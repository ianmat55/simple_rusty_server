use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

pub mod utils;
pub mod http;
pub mod error;
pub mod guess;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer: [u8; 1024] = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    let res = http::handle_response(&buffer[..bytes_read]);
    let formatted_res = res.format();

    // send response back to client
    let _ = stream.write(&formatted_res)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    // create tcp socket at address, bind to port
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "3000";
    let endpoint: String = HOST.to_owned() + ":" + PORT;

    let listener = TcpListener::bind(endpoint)?;

    println!("Web server listenening at {}:{}...", HOST, PORT);

    // listen for tcp connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connected...");
                std::thread::spawn(move || {
                    match handle_client(stream) {
                        Ok(_) => println!("Response sent..."),
                        Err(e) => println!("Error: {}", e),
                    };
                });
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}
