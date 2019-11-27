use std::error::Error;

use tokio::prelude::*;
use tokio::{
    io::BufReader,
    net::TcpStream,
};

pub mod message;
pub use self::message::Message; // Re-export `Message` as part of irc module

type LineHandler = fn(line: &Message) -> ();

struct LineHandlerInfo<'a> {
  label: &'a str,
  f: LineHandler,
}

pub struct Protocol<'a, Connection = TcpStream> {
  nick: &'a str,
  bufconn: BufReader<Connection>,
  handlers: Vec<LineHandlerInfo<'a>>,
}

// T - tcp, tcp/tls, or test fake
impl<'imp, X> Protocol<'imp> for X
  where
    X: AsyncRead + AsyncWrite,
    {
       fn new<'a>(tcp: i32, nick: &'a str) -> () { // Protocol<'a, C> {
        // let p = Protocol {
        //   nick,
        //   bufconn: BufReader::new(tcp),
        //   handlers: Vec::new(),
        // };
        ()
    }

    async fn connect<'a>(&'a mut self, pass: &'a str) -> Result<(), Box<dyn Error>> {
      let connect_str = format!("PASS {pass}\r\nNICK {name}\r\nUSER {name} 0 * {name}\r\n",
          pass=pass, name=self.nick);
      self.bufconn.write_all(connect_str.as_bytes()).await?;

      Ok(())
    }
}

// tokio::test::io::Mock
// tokio::fs::File
// tokio::io::{BufReader, BufWriter}





// impl<'imp> Session<'imp> {
//   pub async fn new<'a>(addr: &'a str, nick: &'a str) -> Result<Session<'a>, Box<dyn Error>> {
//     let tcp = TcpStream::connect(addr).await?;
//     let stream = BufReader::new(tcp);
//     Ok(Session {
//       nick,
//       stream,
//       handlers: Vec::new(),
//     })
//   }

//   pub async fn connect<'a>(&'a mut self, pass: &'a str) -> Result<(), Box<dyn Error>> {
//     let connect_str = format!("PASS {pass}\r\nNICK {name}\r\nUSER {name} 0 * {name}\r\n",
//         pass=pass, name=self.nick);
//     self.stream.write_all(connect_str.as_bytes()).await?;

//     Ok(())
//   }

//   // TODO: maybe name this send_command
//   pub async fn command<'a>(&'a mut self, cmd_str: &'a str) -> Result<(), std::io::Error> {
//     self.stream.write_all(cmd_str.as_bytes()).await
//   }

//   // TODO: why not &'imp mut self ???
//   pub fn register_handler(&mut self, label: &'imp str, f: LineHandler) {
//     self.handlers.push(LineHandlerInfo {
//       label,
//       f,
//     })
//   }

//   pub async fn handle_lines(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//     let mut count = 0;
//     loop {
//       let mut response = String::new();
//       self.stream.read_line(&mut response).await?;
//       {
//         let message = Message::from_string(&response)
//             .ok_or("Could not parse message")?;
//         for info in &self.handlers {
//             (info.f)(&message);
//         }
//       }
//       count += 1;
//       if count > 18 {break};
//     };
//     Ok(())
//   }

// }

#[tokio::test]
async fn can_create_protocol() {
  use tokio::io::{AsyncReadExt, AsyncWriteExt};
  use tokio_test::io::Builder;
  use tokio_test::io::Mock;

  let mock_connection: Mock = Builder::new().write(b"PASS secret\r\nNICK maria\r\nUSER maria 0 * maria\r\n")
                        .read(b":maria!maria@irc.gitter.im NICK :maria\r\n").build();

  let irc = Protocol::new(mock_connection, "maria");
  irc.connect("secret").await.expect("irc.connect");

  // how to test that the write and read actually happened
}
