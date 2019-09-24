
Learning sequence
1. see if we can connect to server with just write_all, 
   maybe initial hard-coded reading of server response
2. [split](https://docs.rs/tokio/0.2.0-alpha.5/tokio/net/tcp/struct.TcpStream.html#method.split) - zero cost split, must be on same task (takes stream reference), assuming read/write operations share state so this is likely to be optimal
3. probably write [Framed codec](https://docs.rs/tokio/0.2.0-alpha.5/tokio/codec/struct.Framed.html), though maybe [Lines codec](https://docs.rs/tokio/0.2.0-alpha.5/tokio/codec/struct.LinesCodec.html) can work or should be made to work

Steps of the tutorial
  1. establish connection/login handshake (pass, user, etc)
      * write sequence of commands
      * handle server response: success / error
  2. bot sends hello messages
  3. change to says something every day at some time... (Good morning San Francisco)
  4. Bot responds to a question
  5. no channel activity for certain amount of time, say "is anyone here?"

