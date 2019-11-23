# Making an IRC bot in Rust

Work-in-progress
- step 1 is mostly complete in main.rs (though might not create module in tutorial)
- examples includes some ideas for possible mini-steps to get to step 1

Steps of the tutorial
  1. establish connection/login handshake (pass, user, etc)
      a) write sequence of commands
      b) handle server response: success / error
  2. bot sends hello messages
  3. change to says something every day at some time... (Good morning San Francisco)
  4. Bot responds to a question
  5. no channel activity for certain amount of time, say "is anyone here?"

Thinking about best timing to fold into the tutorial
- [ ] error handling
- [ ] testing


## 1a: Connect to gitter via IRC

[![Join the chat at https://gitter.im/irc-tokio/community](https://badges.gitter.im/irc-tokio/community.svg)](https://gitter.im/irc-tokio/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

1. join gitter
2. get IRC password from: https://irc.gitter.im/
3. on the command-line

```
socat -v tcp4-listen:1234,reuseaddr,fork ssl:irc.gitter.im:6667,verify=0
```

4. in another terminal window, in local repo directory

```cargo run```

sample expected output (just for showing progress for now)
```
**** joined #ultrasaurus!
 Some(":ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im")
 JOIN
 ["#irc-tokio/community\r\n"]
```

socat output:
```
> 2019/09/21 17:07:51.380401  length=123 from=0 to=122
PASS [redacted]\r
NICK ultrasaurus_twitter\r
USER ultrasaurus_twitter 0 * ultrasaurus_twitter\r
< 2019/09/21 17:07:51.476298  length=82 from=0 to=81
:ultrasaurus_twitter!ultrasaurus_twitter@irc.gitter.im NICK :ultrasaurus_twitter\r
...
```


## Notes from initial brainstorm

potential learning sequence...

1. connect to server with just write_all,
   maybe initial hard-coded reading of server response
2. [split](https://docs.rs/tokio/0.2.0-alpha.5/tokio/net/tcp/struct.TcpStream.html#method.split) - zero cost split, must be on same task (takes stream reference), assuming read/write operations share state so this is likely to be optimal
3. probably write [Framed codec](https://docs.rs/tokio/0.2.0-alpha.5/tokio/codec/struct.Framed.html), though maybe [Lines codec](https://docs.rs/tokio/0.2.0-alpha.5/tokio/codec/struct.LinesCodec.html) can work or should be made to work
