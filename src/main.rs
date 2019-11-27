use std::env;
use std::error::Error;
use tokio::{
  net::TcpStream,
};

mod irc;

//https://docs.rs/tokio/0.2.0-alpha.5/tokio/net/tcp/struct.TcpStream.html
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let irc_user = env::var("USER")
        .expect("USER environment var required. Perhaps you forgot to `source .env`");
    let irc_pass = env::var("PASS")
        .expect("PASS environment var required. Perhaps you forgot to `source .env`");

    // Connect to the server

    let addr = "127.0.0.1:1234";
    let tcp = TcpStream::connect(addr).await?;
    //     let tcp = TcpStream::connect(addr).await?;

    let irc: irc::Protocol<'_, TcpStream> = irc::Protocol::new(tcp, &irc_user);

    // let mut irc = irc::Session::new(addr, &irc_user).await?;

    // // :ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im JOIN #irc-tokio/community\r
    // irc.register_handler("#irc-tokio/community JOIN response", |message| {
    //   if message.command == "JOIN" {
    //     println!("**** joined #ultrasaurus!\n {:?}\n {}\n {:?}\n", message.prefix, message.command, message.params);
    //   }
    //   ()
    // });

    // // TODO: handle ping (maybe inside Session impl)

    // irc.connect(&irc_pass).await?;    // read loop
    // irc.command("JOIN #irc-tokio/community\r\n").await?;
    // irc.handle_lines().await?;
    Ok(())
}