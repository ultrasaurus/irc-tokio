use crate::error::Error;

#[derive(Debug, PartialEq)]
pub struct Message<'a> {
  pub prefix: Option<&'a str>,
  pub command: &'a str,
  pub params: Vec<&'a str>,
}

impl<'m> Message<'m> {
  pub fn from_string(line: &'m str) -> Result<Message<'m>, Error> {
    if line.is_empty() {
        return Err(Error::MessageMissingCommand);
    }
    if line.chars().count() > 512 {
        return Err(Error::MessageStringTooLong);
    }

    let mut parts = line.split(' ');

    let prefix = line.chars().nth(0)
        .and_then(|c| {
            if c == ':' {
                parts.next()
            } else {
                None
            }
        });

    let command = parts.next()
        .ok_or(Error::MessageMissingCommand)?;

    let params = parts.collect();

    Ok(Message {
      prefix,
      command,
      params,
    })
  }
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO use a macro to match error enum variant
    // Since we can't use PartialEq, only pattern matching

    #[test]
    fn can_create_message() {
        let m = Message::from_string(":me!me@irc.gitter.im JOIN #mychannel").unwrap();
        assert_eq!(m.prefix, Some(":me!me@irc.gitter.im"));
        assert_eq!(m.command, "JOIN");
        assert_eq!(m.params[0], "#mychannel");
    }

    #[test]
    fn test_missing_command() {
        let m = Message::from_string("");
        match m {
            Err(Error::MessageMissingCommand) => (),
            _ => panic!(),
        }

        let m = Message::from_string(":me!me@irc.gitter.im");
        match m {
            Err(Error::MessageMissingCommand) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn test_string_too_long() {
        let prefix_command = ":me!me@irc.gitter.im JOIN #";
        let channel_name = "n".repeat(486);
        assert_eq!(prefix_command.chars().count() + channel_name.chars().count(), 513);

        let message = format!("{}{}", prefix_command, channel_name);
        let m = Message::from_string(&message);
        match m {
            Err(Error::MessageStringTooLong) => (),
            _ => panic!(),
        }
    }
}
