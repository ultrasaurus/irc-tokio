# Making an IRC bot in Rust

Work-in-progress

* Please put general feedback in the [tokio/doc-push issue](https://github.com/tokio-rs/doc-push/issues/98)
* Pull requests welcome! specifically...
  * refactors or additions to `main.rs` (which is where we are working on something "directional" like what we might have after completing
  the tutorial and then maybe doing some more work)
  * examples has ideas for steps in the tutorial (small bits of working code that could build toward the full IRC bot)
  * ASK.md - these are questions or errors that would benefit from answering in
  docs or somewhere.  Add questions or answers or both!

- step 1 is mostly complete in main.rs (though might not create module in tutorial)
- examples includes some ideas for possible mini-steps to get to step 1

Steps of the tutorial

Part A) write an IRC bot (and IRC client library with subset of IRC protocol)
  1. establish connection/login handshake (pass, user, etc)
      a) write/read from socket using simplest possible syntax
      b) refactor to impl to make it easier modify in next steps
         - additional logic needed to complete connection sequence
         - write simple unit tests (maybe test first?)
      c) handle server response: success / error
      d) add JOIN/PING-PONG command, if we didn't already handle above
  2. bot sends hello messages
  3. change to says something every day at some time... (Good morning San Francisco)
  4. Bot responds to a question
  5. no channel activity for certain amount of time, say "is anyone here?"

Part B) write an IRC server

Part C) More testing!
  1. Write FakeIrcServer based on real server implementation (might need server refactor)
  2. Refactor part A to use FakeIrcServer for testing for integration tests


Thinking about best timing to fold into the tutorial
- [ ] error handling

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
