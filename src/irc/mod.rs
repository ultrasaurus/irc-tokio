use std::error::Error;

use tokio::prelude::*;
use tokio::{
    io::BufReader,
    net::TcpStream,
};

pub mod message;
pub use self::message::Message; // Re-export `Message` as part of irc module

struct LineHandlerInfo<'a> {
  label: &'a str,
  f: LineHandler,
}

pub struct Session<'a> {
    nick: &'a str,
    stream: BufReader<TcpStream>,
    handlers: Vec<LineHandlerInfo<'a>>,
}

type LineHandler = fn(line: &Message) -> ();

impl<'imp> Session<'imp> {
  pub async fn new<'a>(addr: &'a str, nick: &'a str) -> Result<Session<'a>, Box<dyn Error>> {
    let tcp = TcpStream::connect(addr).await?;
    let stream = BufReader::new(tcp);
    Ok(Session {
      nick,
      stream,
      handlers: Vec::new(),
    })
  }

  pub async fn connect<'a>(&'a mut self, pass: &'a str) -> Result<(), Box<dyn Error>> {
    let connect_str = format!("PASS {pass}\r\nNICK {name}\r\nUSER {name} 0 * {name}\r\n",
        pass=pass, name=self.nick);
    self.stream.write_all(connect_str.as_bytes()).await?;

    Ok(())
  }

  // TODO: maybe name this send_command
  pub async fn command<'a>(&'a mut self, cmd_str: &'a str) -> Result<(), std::io::Error> {
    self.stream.write_all(cmd_str.as_bytes()).await
  }

  // TODO: why not &'imp mut self ???
  pub fn register_handler(&mut self, label: &'imp str, f: LineHandler) {
    self.handlers.push(LineHandlerInfo {
      label,
      f,
    })
  }

  pub async fn handle_lines(&mut self) -> Result<(), std::io::Error> {
    let mut count = 0;
    loop {
      let mut response = String::new();
      self.stream.read_line(&mut response).await?;
      {
        let message = Message::from_string(&response)?;
        for info in &self.handlers {
            (info.f)(&message);
        }
      }
      count += 1;
      if count > 18 {break};
    };
    Ok(())
  }

}

