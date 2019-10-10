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
  async fn connect<'a>(stream: BufReader<&'a mut TcpStream>, nick: &'a str, pass: &'a str) -> Result<Session<'a>, Box<dyn Error>> {
    Ok(Session {
      nick,
      stream
    })
  }

  

}

//https://docs.rs/tokio/0.2.0-alpha.5/tokio/net/tcp/struct.TcpStream.html
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let irc_user = env::var("USER")
        .expect("USER environment var required. Perhaps you forgot to `source .env`");   
    let irc_pass = env::var("PASS")
        .expect("PASS environment var required. Perhaps you forgot to `source .env`"); 

    // Connect to the server
    let addr = "127.0.0.1:1234";
    let mut raw_tcp = TcpStream::connect(addr).await?;
    let mut buffered_stream = BufReader::new(&mut raw_tcp); 

    let irc = Session::connect(buffered_stream, &irc_user, &irc_pass).await?;
    // let join_request = "JOIN #ultrasaurus";
    // let join_response = ":ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im JOIN #ultrasaurus";
    // response = irc.request(join_request, join_response).await;
    // irc.message("PRIVMSG #ultrasaurus hi");

    // irc.message("PRIVMSG #ultrasaurus bye");

    Ok(())
}