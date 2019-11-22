use std::env;
use std::error::Error;


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
    let mut irc = irc::Session::new(addr, &irc_user).await?;

    // :ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im JOIN #irc-tokio/community\r
    irc.register_handler("#irc-tokio/community JOIN response", |message| {
      println!("handler");
      if message.command == "JOIN" {
        println!("************** joined #ultrasaurus: {:?} {} {:?}", message.prefix, message.command, message.params);
      }
      ()
    });

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