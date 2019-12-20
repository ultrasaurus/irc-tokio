pub mod error;
mod message;

use tokio::prelude::*;
use tokio::{io::BufReader, net::TcpStream};

pub use crate::message::Message; // Re-export `Message` as part of irc module
use crate::error::Error;

type LineHandler = fn(line: &Message) -> ();

struct LineHandlerInfo {
  #[allow(dead_code)]
  label: String,
  f: LineHandler,
}

pub struct Protocol<T = TcpStream> {
  nick: String,
  bufconn: BufReader<T>,
  handlers: Vec<LineHandlerInfo>,
}

// T - tcp, tcp/tls, or test fake
impl<Connection: AsyncRead + AsyncWrite + Unpin> Protocol<Connection> {
  pub fn new(tcp: Connection, nick: String) -> Self {
    Protocol {
      nick,
      bufconn: BufReader::new(tcp),
      handlers: Vec::new(),
    }
  }

  pub async fn connect(&mut self, pass: &str) -> Result<(), Error> {
    let connect_str = format!(
      "PASS {pass}\r\nNICK {name}\r\nUSER {name} 0 * {name}\r\n",
      pass = pass,
      name = self.nick
    );
    self.bufconn.write_all(connect_str.as_bytes()).await?;
    Ok(())
  }

  // TODO: maybe name this send_command
  pub async fn command(&mut self, cmd_str: &str) -> Result<(), Error> {
    println!("command: {:?}", cmd_str);
    self.bufconn.write_all(cmd_str.as_bytes()).await?;
    Ok(())
  }

  // TODO: why not &'imp mut self ???
  pub fn register_handler(&mut self, label: String, f: LineHandler) {
    self.handlers.push(LineHandlerInfo { label, f })
  }

  pub async fn handle_lines(&mut self) -> Result<(), Error> {
    let mut count = 0;
    loop {
      let mut response = String::new();
      self.bufconn.read_line(&mut response).await?;
      {
        let message = Message::from_string(&response)?;

        for info in &self.handlers {
          (info.f)(&message);
        }
      }
      count += 1;
      if count > 18 {
        break;
      };
    }
    Ok(())
  }
}

#[tokio::test]
async fn can_create_protocol() {
  use tokio_test::io::Builder;
  use tokio_test::io::Mock;

  let mock_connection: Mock = Builder::new()
    .write(b"PASS secret\r\nNICK maria\r\nUSER maria 0 * maria\r\n")
    .read(b":maria!maria@irc.gitter.im NICK :maria\r\n")
    .build();

  let mut irc = Protocol::new(mock_connection, "maria".into());
  irc.connect("secret").await.expect("irc.connect");

  // how to test that the write and read actually happened
}
