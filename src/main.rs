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
    stream: &'a mut BufReader<&'a mut TcpStream>,
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

  async fn wait_for_connect_confirmation<'a>(&'a mut self) -> Result<(), Box<dyn Error>> {
    self.read_expect(":ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im NICK :ultrasaurus_twitter\r\n").await?;
    self.read_expect(":irc.gitter.im 001 ultrasaurus_twitter Gitter :ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im\r\n").await?;
    self.read_expect(":irc.gitter.im 002 ultrasaurus_twitter Version: 1.4.0\r\n").await?;
    Ok(())
  }
  async fn connect<'a>(stream: &'a mut BufReader<&'a mut TcpStream>, nick: &'a str, pass: &'a str) -> Result<Session<'a>, Box<dyn Error>> {
    let connect_str = format!("PASS {pass}\r\nNICK {name}\r\nUSER {name} 0 * {name}\r\n", 
        name=nick, pass=pass);
    stream.write_all(connect_str.as_bytes()).await?;
    
    Ok(Session {
      nick,
      stream
    })
  }

  // // TODO: consider String vs &str parameter
  // async fn read_line<'a>(mut self, response: &'a mut String) -> Result<(), Box<dyn Error>> {
  //   // TODO: this prolly doesn't handle CLRF correctly
  //   self.stream.read_line(response).await?;
  //   Ok(())
  // }

  async fn message<'a>(mut self, message: &'a str) -> Result<(), Box<dyn Error>> {
    // TODO: add CRLF if needed
    self.stream.write_all(message.as_bytes()).await?;
    Ok(())
  }

  // async fn request<'a>(mut self, message: &'a str, expected_response: &'a str) -> Result<(), Box<dyn Error>> {
  //   let mut response = String::new();
  //   self.write_line(message.to_string()).await?;
  //   self.read_line(&mut response);
  //   if response == expected_response.to_string() {
  //     Ok(())
  //   } else {
  //     panic!("unexpected")      // not sure what's a good pattern for returning an error here
  //   }
  // }


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

    let mut irc = Session::connect(&mut buffered_stream, &irc_user, &irc_pass).await?;
    irc.wait_for_connect_confirmation().await?;
    // let join_request = "JOIN #ultrasaurus";
    // let join_response = ":ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im JOIN #ultrasaurus";
    // irc.request(join_request, join_response).await?;

// see error below.  I wouldn't normally put the await on line 85, but program
// isn't done... 
//     irc.message("PRIVMSG #ultrasaurus hi\r\n");
//     irc.message("PRIVMSG #ultrasaurus bye\r\n").await?;
// 78 |     let irc = Session::connect(&mut buffered_stream, &irc_user, &irc_pass).await?;
//    |         --- move occurs because `irc` has type `Session<'_>`, which does not implement the `Copy` trait
// ...
// 83 |     irc.message("PRIVMSG #ultrasaurus hi\r\n");
//    |     --- value moved here
// 84 | 
// 85 |     irc.message("PRIVMSG #ultrasaurus bye\r\n").await?;
//    |     ^^^ value used here after move


// next step bot would wait to receive a message
// or time interval

    Ok(())
}