use tokio::net::TcpStream;
use tokio::prelude::*;
use std::error::Error;

//https://docs.rs/tokio/0.2.0-alpha.5/tokio/net/tcp/struct.TcpStream.html
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:1234").await?;

    // Write some data.
    stream.write_all(b"USER ").await?;

    Ok(())
}