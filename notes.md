
## setup to make this

```
cargo new irc-tokio
cargo add tokio
```
paste code from [doc](//https://docs.rs/tokio/0.2.0-alpha.5/tokio/net/tcp/struct.TcpStream.html) into `src/main.rs`


```
cargo +nightly build
```

```
error[E0308]: mismatched types
 --> src/main.rs:9:41
  |
9 |     let mut stream = TcpStream::connect("127.0.0.1:1234").await?;
  |                                         ^^^^^^^^^^^^^^^^ expected enum `std::net::SocketAddr`, found str
  |
  = note: expected type `&std::net::SocketAddr`
             found type `&'static str`

error[E0277]: the trait bound `tokio_tcp::stream::ConnectFuture: std::future::Future` is not satisfied
 --> src/main.rs:9:22
  |
9 |     let mut stream = TcpStream::connect("127.0.0.1:1234").await?;
  |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `tokio_tcp::stream::ConnectFuture`
  |
  = note: required by `std::future::poll_with_tls_context`

error[E0277]: `main` has invalid return type `impl std::future::Future`
 --> src/main.rs:7:20
  |
7 | async fn main() -> Result<(), Box<dyn Error>> {
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ `main` can only return types that implement `std::process::Termination`
  |
  = help: consider using `()`, or a `Result`

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: Could not compile `irc-tokio`.
```