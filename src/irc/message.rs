pub struct Message<'a> {
  pub prefix: Option<&'a str>,
  pub command: &'a str,
  pub params: Vec<&'a str>,
}

impl<'m> Message<'m> {
  pub fn from_string(line: &'m str) -> Option<Message<'m>> {
    let mut parts = line.split(' ');

    let prefix = line.chars().nth(0)
        .and_then(|c| {
            if c == ':' {
                parts.next()
            } else {
                None
            }
        });

    let command = parts.next()?;

    let params = parts.collect();

    Some(Message {
      prefix,
      command,
      params,
    })
  }
}

#[test]
fn can_create_message() {
    let m = Message::from_string(":me!me@irc.gitter.im JOIN #mychannel").unwrap();
    assert_eq!(m.prefix, Some(":me!me@irc.gitter.im"));
    assert_eq!(m.command, "JOIN");
    assert_eq!(m.params[0], "#mychannel");
}
