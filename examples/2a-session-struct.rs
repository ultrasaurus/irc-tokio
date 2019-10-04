use std::env;
use std::error::Error;

use tokio::prelude::*;
use tokio::{
    net::TcpStream,
    io::BufReader,
    net::tcp::split::WriteHalf,
    net::tcp::split::ReadHalf,
};

struct Session<'a> {
    pub nick: &'a str,
    pub state: State,
    rx: BufReader<ReadHalf<'a>>,
    tx: WriteHalf<'a>,
}

enum State {
    Connecting,
    Connected,
    Error,
}

impl Session<'_> {
    pub async fn connect<'a>(tcp: &'a mut TcpStream, nick: &'a str, pass: &'a str) -> Result<Session<'a>, Box<dyn Error>> {
        let connect_str = format!("PASS {pass}\r\nNICK {name}\r\nUSER {name} 0 * {name}\r\n", 
            pass=pass, name=nick);
            let (rx, mut tx) = tcp.split();
            tx.write_all(connect_str.as_bytes()).await?;
            let mut response = String::new();
            let mut reader = tokio::io::BufReader::new(rx);
            reader.read_line(&mut response).await?;
            println!("response: {}", response);

            Ok(Session {
                nick,
                rx: reader, 
                tx,
                state: State::Connected
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
    let mut stream = TcpStream::connect(addr).await?;

    let irc = Session::connect(&mut stream, &irc_user, &irc_pass).await?;
    println!("session {:?}", irc.nick);

    Ok(())
}