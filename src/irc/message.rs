pub struct Message<'a> {
  pub prefix: Option<&'a str>,
  pub command: &'a str,
  pub params: Vec<&'a str>,
}

impl Message<'_> {
  pub fn from_string<'m>(line: &'m str) -> Result<Message<'m>, std::io::Error> {
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

#[test]
fn can_create_message() {
    let m = (Message::from_string(":me!me@irc.gitter.im JOIN #mychannel")).unwrap();
    assert_eq!(m.prefix, Some(":me!me@irc.gitter.im"));
    assert_eq!(m.command, "JOIN");
    assert_eq!(m.params[0], "#mychannel");
}