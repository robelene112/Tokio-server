#![allow(unused_imports, unused_variables, dead_code)]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const ECHO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    println!("Starting server on {}...", ECHO_SERVER_ADDRESS);

    if let Ok(listener) = TcpListener::bind(ECHO_SERVER_ADDRESS).await {
        println!("Listener bound on: {:?}", listener.local_addr().unwrap());

        loop {
            match listener.accept().await {
                Ok((stream, socketaddress)) => {
                    println!("Found new client on Ip: {}", socketaddress.ip());
                    println!("Connected on port: {}", socketaddress.port());
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
