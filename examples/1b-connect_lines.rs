use std::env;
use std::error::Error;

use tokio::prelude::*;
use tokio::{
    codec::{Framed, LinesCodec},
    net::TcpStream,
};


//https://docs.rs/tokio/0.2.0-alpha.5/tokio/net/tcp/struct.TcpStream.html
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let irc_user = env::var("USER")
        .expect("USER environment var required. Perhaps you forgot to `source .env`");   
    let irc_pass = env::var("PASS")
        .expect("PASS environment var required. Perhaps you forgot to `source .env`"); 

    // Connect to the server
    let addr = "127.0.0.1:1234";
    let stream = TcpStream::connect(addr).await?;
    let mut lines = Framed::new(stream, LinesCodec::new());

    // Write some data.
    let connect_str = format!("PASS {pass}\r\nNICK {name}\r\nUSER {name} 0 * {name}\r\n", 
            pass=irc_pass, name=irc_user);

    lines
        .send(connect_str)
        .await?;

    // socat output from above code
    // > 2019/09/24 15:10:19.571671  length=123 from=0 to=122
    // PASS [redacted]\r
    // NICK ultrasaurus_twitter\r
    // USER ultrasaurus_twitter 0 * ultrasaurus_twitter\r

    // Next we want to read server response that looks like this:
    // < 2019/09/24 15:10:19.671806  length=82 from=0 to=81
    // :ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im NICK :ultrasaurus_twitter\r


    // Read the first line from the `LineCodec` stream to get the server response.
    let response = lines.next().await.unwrap();
    if let Ok(line) = response {
        println!("response: {:?}", line);
    }
    // example output 
    // response: :ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im NICK :ultrasaurus_twitter
    // response: :gitter!gitter@irc.gitter.im PRIVMSG gitter : Authentication failed. Get a valid token from https://irc.gitter.im


    // TODO

    // let join_str = format!("JOIN #irc-tokio\r\nPRIVMSG #irc-tokio :Hello from my first Rust IRC client!\r\n");
    // stream.write_all(join_str.as_bytes()).await?;

    Ok(())
}