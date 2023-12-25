use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use rand::Rng;

pub mod utils;

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();


    let res = utils::handle_response(&buffer);
    let formatted_res = res.format();
    // send response back to client
    let let_ = stream.write(&formatted_res);
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
        println!("Connection established!\n");

        handle_client(stream?);
    }

    Ok(())
}
