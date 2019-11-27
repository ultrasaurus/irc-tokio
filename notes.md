
## setup to make this

```
cargo new irc-tokio
cargo add tokio
```

* add `rust-toolchain`
* paste code from [doc](//https://docs.rs/tokio/0.2.0-alpha.5/tokio/net/tcp/struct.TcpStream.html) into `src/main.rs`


## ideas for the future

### testing

Would be nice to have something like this for client testing:

```
let fake = FakeIrcServer.new();
let mut a_called = false;
let mut b_called = false;

let irc = Protocol::new(fake, "username");
irc.register_handler("JOIN A", {
  a_called = true;
  irc.send("A: hello")
})
irc.register_handler( {
  b_called = true;
  irc.send("B: hello")
})
irc.connect();
irc.join("A")
irc.join("B")
fake.flush();
assert(a_called);
assert(b_called);
```