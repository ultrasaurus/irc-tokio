use tokio::net::TcpStream;
use tokio::prelude::*;
use std::env;
use std::error::Error;

//https://docs.rs/tokio/0.2.0-alpha.5/tokio/net/tcp/struct.TcpStream.html
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let irc_user = env::var("USER")?;
    let irc_pass = env::var("PASS")?;

    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:1234").await?;

    // Write some data.
    let connect_str = format!("PASS {pass}\r\nNICK {name}\r\nUSER {name} 0 * {name}\r\n", 
            pass=irc_pass, name=irc_user);
    stream.write_all(connect_str.as_bytes()).await?;

    // Read some data
    // TODO

    // let join_str = format!("JOIN #irc-tokio\r\nPRIVMSG #irc-tokio :Hello from my first Rust IRC client!\r\n");
    // stream.write_all(join_str.as_bytes()).await?;

    Ok(())
}