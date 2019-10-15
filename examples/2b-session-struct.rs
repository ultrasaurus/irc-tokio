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
    stream: BufReader<TcpStream>,
}

impl Session<'_> {
  async fn read_expect<'a>(&'a mut self, expected_response: &'a str) -> Result<(), Box<dyn Error>> {
    let mut response = String::new();
    self.stream.read_line(&mut response).await?;
    if response == expected_response.to_string() {
      Ok(())
    } else {
      let error_str = format!("expected: {}, got: {}", expected_response, response);
      panic!(error_str)      // not sure what's a good pattern for returning an error here
    }
  }

  async fn connect<'a>(addr: &'a str, nick: &'a str, pass: &'a str) -> Result<Session<'a>, Box<dyn Error>> {
    let tcp = TcpStream::connect(addr).await?;
    let stream = BufReader::new(tcp); 
    let mut session = Session {
      nick,
      stream
    };
    session.read_expect(":ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im NICK :ultrasaurus_twitter\r\n").await?;
    session.read_expect(":irc.gitter.im 001 ultrasaurus_twitter Gitter :ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im\r\n").await?;
    session.read_expect(":irc.gitter.im 002 ultrasaurus_twitter Version: 1.4.0\r\n").await?;

    Ok(session)
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
    let irc = Session::connect(addr, &irc_user, &irc_pass).await?;

    // let join_request = "JOIN #ultrasaurus";
    // let join_response = ":ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im JOIN #ultrasaurus";
    // response = irc.request(join_request, join_response).await;
    // irc.message("PRIVMSG #ultrasaurus hi");

    // irc.message("PRIVMSG #ultrasaurus bye");

    Ok(())
}