
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
error[E0277]: `main` has invalid return type `impl std::future::Future`
 --> src/main.rs:7:20
  |
7 | async fn main() -> Result<(), Box<dyn Error>> {
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ `main` can only return types that implement `std::process::Termination`
  |
  = help: consider using `()`, or a `Result`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: Could not compile `irc-tokio`.
```