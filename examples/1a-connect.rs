use tokio::net::TcpStream;
use tokio::prelude::*;
use std::env;
use std::error::Error;


// As a tutorial, this will likely be in several steps:
// 1. Connect - see error 
// 2. Set up env vars based on gitter & write_all
// 3. Read response
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let irc_user = env::var("USER")?;
    let irc_pass = env::var("PASS")?;
    // if env vars not set up, you get Error: NotPresent

    let mut stream = TcpStream::connect("127.0.0.1:1234").await?;
    // Error: Os { code: 61, kind: ConnectionRefused, message: "Connection refused" }
    
    // Write some data.
    let connect_str = format!("PASS {pass}\r\nNICK {name}\r\nUSER {name} 0 * {name}\r\n", 
            pass=irc_pass, name=irc_user);
    stream.write_all(connect_str.as_bytes()).await?;

    // success output from socat:
    // > 2019/09/24 15:10:19.571671  length=123 from=0 to=122
    // PASS [redacted]\r
    // NICK ultrasaurus_twitter\r
    // USER ultrasaurus_twitter 0 * ultrasaurus_twitter\r
    // < 2019/09/24 15:10:19.671806  length=82 from=0 to=81
    // :ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im NICK :ultrasaurus_twitter\r

    Ok(())
}