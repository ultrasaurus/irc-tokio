use std::borrow::Cow;
use std::env;
use std::error::Error;

use tokio::prelude::*;
use tokio::{
    io::BufReader,
    net::TcpStream,
};

#[derive(Debug)]
struct Session<'a> {
    nick: &'a str,
    stream: BufReader<&'a mut TcpStream>,
}


impl Session<'_> {
  async fn connect<'a>(addr: &'a str, nick: &'a str, pass: &'a str) -> Result<Session<'a>, Box<dyn Error>> {
    let mut tcp_stream = TcpStream::connect(addr).await?;
    let mut stream = BufReader::new(&mut tcp_stream);

    let connect_str = format!("PASS {pass}\r\nNICK {name}\r\nUSER {name} 0 * {name}\r\n",
            pass=pass, name=nick);
    stream.write_all(connect_str.as_bytes()).await?;

    let mut response = String::new();
    stream.read_line(&mut response).await?;
    println!("response: {}", response);

    Ok(Session {
      nick,
      stream,
    })
  }

}

/**
 * error[E0515]: cannot return value referencing local variable `tcp_stream`
  --> src/main.rs:31:5
   |
21 |       let mut stream = BufReader::new(&mut tcp_stream);
   |                                       --------------- `tcp_stream` is borrowed here
...
31 | /     Ok(Session {
32 | |       nick,
33 | |       stream,
34 | |     })
   | |______^ returns a value referencing data owned by the current function
 */

//https://docs.rs/tokio/0.2.0-alpha.5/tokio/net/tcp/struct.TcpStream.html
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let irc_user = env::var("USER")
        .expect("USER environment var required. Perhaps you forgot to `source .env`");   
    let irc_pass = env::var("PASS")
        .expect("PASS environment var required. Perhaps you forgot to `source .env`"); 

    // Connect to the server
    let addr = "127.0.0.1:1234";

    let irc = Session::connect(addr, &irc_user, &irc_pass).await?;
    // let join_request = "JOIN #ultrasaurus";
    // let join_response = ":ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im JOIN #ultrasaurus";
    // response = irc.request(join_request, join_response).await;
    // irc.message("PRIVMSG #ultrasaurus hi");

    // irc.message("PRIVMSG #ultrasaurus bye");

    Ok(())
}