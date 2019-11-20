use std::env;
use std::error::Error;

use tokio::prelude::*;
use tokio::{
    io::BufReader,
    net::TcpStream,
};

struct LineHandlerInfo<'a> {
  label: &'a str,
  f: LineHandler,
}

// 1. prefix (OPTIONAL)
// 2. command
// 3. command parameters (maximum of fifteen (15)).  The prefix, command, and all 

struct Message<'a> {
  prefix: Option<&'a str>, 
  command: &'a str,
  params: Vec<&'a str>,
}

impl Message<'_> {
  fn from_string<'m>(line: &'m str) -> Result<Message<'m>, std::io::Error> {
    let mut parts = line.split(' ');
    let prefix = if line.get(0..1).unwrap() == ":" {
      parts.next()
    } else {
      None
    };
    
    Ok(Message {
      prefix,
      command: parts.next().unwrap(),
      params: parts.collect(),
    })
  }
}

struct Session<'a> {
    nick: &'a str,
    stream: BufReader<TcpStream>,
    handlers: Vec<LineHandlerInfo<'a>>,
}

type LineHandler = fn(line: &Message) -> ();

impl<'imp> Session<'imp> {
  async fn new<'a>(addr: &'a str, nick: &'a str) -> Result<Session<'a>, Box<dyn Error>> {
    let tcp = TcpStream::connect(addr).await?;
    let stream = BufReader::new(tcp); 
    Ok(Session {
      nick,
      stream,
      handlers: Vec::new(),
    })
  }

  async fn connect<'a>(&'a mut self, pass: &'a str) -> Result<(), Box<dyn Error>> {
    let connect_str = format!("PASS {pass}\r\nNICK {name}\r\nUSER {name} 0 * {name}\r\n", 
        pass=pass, name=self.nick);
    self.stream.write_all(connect_str.as_bytes()).await?; 
    
    Ok(())
  }

  async fn command<'a>(&'a mut self, cmd_str: &'a str) -> Result<(), std::io::Error> {
    self.stream.write_all(cmd_str.as_bytes()).await
  }

  // TODO: why not &'imp mut self ???
  fn register_handler(&mut self, label: &'imp str, f: LineHandler) {
    self.handlers.push(LineHandlerInfo {
      label,
      f,
    })
  }

  async fn handle_lines(&mut self) -> Result<(), std::io::Error> {
    let mut count = 0;
    loop {
      let mut response = String::new();
      self.stream.read_line(&mut response).await?;
      println!("{} {}", count, response);
      {
        let message = Message::from_string(&response)?;
        for info in &self.handlers {
            println!("about to call: {}", info.label);
            (info.f)(&message);
        }  
      }
      count += 1;
      if count > 18 {break};
    };
    Ok(())
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
    let mut irc = Session::new(addr, &irc_user).await?;

    // let join_response = format!("{name}@irc.gitter.im JOIN #irc-tokio/community\r\n", 
    //     name=irc_user);
// :ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im JOIN #irc-tokio/community\r

    irc.register_handler("#irc-tokio/community JOIN response", |message: &Message| {
      println!("handler");
      if message.command == "JOIN" {
        println!("************** joined #ultrasaurus: {:?} {} {:?}", message.prefix, message.command, message.params);
      } 
      ()
    });

    // irc.match_str("Joining #ultrasaurus", &join_response, |line| {
    //   println!("************** joined #ultrasaurus: {}", line);
    //   // do something
    //   None
    // });


    // irc.match_str("Joining ultrasaurus", join_response, ultra_handler);

    // irc.match_fn("When pinged", "fn", ping_handler);
    // irc.match_regex("when asked for help", "/^@irc_user: help/", help_handler);

    // irc.event_disconnect("when disconnected", reconnect);
    // irc.init("/join #ultrasaurus");
    irc.connect(&irc_pass).await?;    // read loop
    println!("                                               -----------------------------");
    irc.command("JOIN #irc-tokio/community\r\n").await?;
    irc.handle_lines().await?;



    Ok(())
}