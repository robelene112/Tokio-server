#![allow(unused_imports, unused_variables, dead_code)]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const ECHO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    println!("Starting server on {}...", ECHO_SERVER_ADDRESS);

    if let Ok(listener) = TcpListener::bind(ECHO_SERVER_ADDRESS).await {
        println!("Listener bound on: {:?}", listener.local_addr().unwrap());

        loop {
            match listener.accept().await {
                Ok((mut stream, socketaddress)) => {
                    println!("Found new client on Ip: {}", socketaddress.ip());
                    println!("Connected on port: {}", socketaddress.port());

                    handle_connection(&mut stream).await;
                }
                Err(e) => {
                    println!("Could not recieve client {}", e)
                }
            }
        }
    } else {
        println!("Failed to bind server on {}", ECHO_SERVER_ADDRESS)
    }
}

async fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).await;
    let message = String::from_utf8_lossy(&buffer);
    println!("{}", message);
}
